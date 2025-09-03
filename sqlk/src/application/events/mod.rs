use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::{Duration, Instant};

use crate::{
    application::{
        app::App,
        state::{AppMessage, AppMode, StartupResult},
    },
    config::MatrixConfig,
    matrix::MatrixAnimation,
    ui::ToastType,
};

impl App {
    pub async fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);

        let mut terminal = Terminal::new(backend)?;

        if self.config.matrix.enabled {
            let terminal_size = terminal.size()?;
            let full_screen_config =
                MatrixConfig::for_full_screen(terminal_size.width, terminal_size.height);

            let matrix_animation = MatrixAnimation::new(&full_screen_config);
            self.matrix_animation = Some(matrix_animation);
        } else {
            self.perform_startup_tasks_sync().await?;
        }

        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(16);

        loop {
            if crossterm::event::poll(tick_rate)? &&
                let Event::Key(key) = event::read()? {
                    self.handle_key_event(key).await?;
            }

            if last_tick.elapsed() >= tick_rate {
                self.update().await?;
                last_tick = Instant::now();
            }

            let mut ui = std::mem::take(&mut self.ui);
            terminal.draw(|f| {
                ui.render(f, self);
            })?;
            self.ui = ui;

            if self.should_quit {
                break;
            }
        }

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    // Modify the update method to handle query results
    pub async fn update(&mut self) -> Result<()> {
        // Handle incoming query results
        while let Ok(message) = self.app_rx.try_recv() {
            match message {
                AppMessage::QueryResult(result) => {
                    self.is_querying = false; // Clear loading state

                    match result {
                        Ok(viewer) => {
                            self.table_viewer = Some(viewer);

                            if self.matrix_animation.is_none() {
                                self.current_mode = AppMode::TableViewer;
                            }

                            if self.startup_complete || !self.config.matrix.enabled {
                                self.ui.add_toast(
                                    "Query executed successfully".to_string(),
                                    ToastType::Success,
                                );
                            }
                        }
                        Err(e) => {
                            self.ui.add_toast(e, ToastType::Error);
                            self.current_mode = AppMode::FileView;
                            self.matrix_animation = None;
                        }
                    }
                }
            }
        }

        // Existing matrix animation logic
        if let Some(matrix) = &mut self.matrix_animation {
            matrix.update();

            if !self.startup_complete && self.startup_task.is_none() && matrix.get_progress() >= 0.3
            {
                self.startup_task = Some(self.spawn_startup_tasks());
            }
        }

        if let Some(handle) = &self.startup_task &&
            handle.is_finished() &&
                let Some(handle) = self.startup_task.take() {
                    match handle.await {
                        Ok(Ok(result)) => {
                            self.apply_startup_result(result).await?;
                            self.startup_complete = true;
                        }
                        Ok(Err(e)) => {
                            self.ui
                                .add_toast(format!("Startup task failed: {}", e), ToastType::Error);
                            self.startup_complete = true;
                        }
                        Err(e) => {
                            self.ui.add_toast(
                                format!("Startup task panicked: {}", e),
                                ToastType::Error,
                            );
                            self.startup_complete = true;
                        }
                    }
                }

        let mut animation_finished = false;
        if let Some(matrix) = &self.matrix_animation &&
            self.startup_complete && matrix.is_finished() {
                animation_finished = true;
        }

        if animation_finished {
            self.matrix_animation = None;
            self.current_mode = if self.table_viewer.is_some() {
                AppMode::TableViewer
            } else {
                AppMode::FileView
            };
        }

        Ok(())
    }

    pub async fn apply_startup_result(&mut self, result: StartupResult) -> Result<()> {
        if let Some(content) = result.file_content {
            self.file_content = content;
            self.query_blocks = result.query_blocks;
            self.cursor_line = 0;
            self.scroll_offset = 0;
            self.file_loading_complete = true;
        }

        if let Some(viewer) = result.table_viewer {
            self.table_viewer = Some(viewer);
        } else if self.pending_query.is_none() {
            self.current_mode = AppMode::FileView;
        }

        if let Some(success_msg) = result.success_message {
            self.ui.add_toast(success_msg, ToastType::Success);
        }
        if let Some(error_msg) = result.error_message {
            self.ui.add_toast(error_msg, ToastType::Error);
        }

        Ok(())
    }
}


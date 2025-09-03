use anyhow::Result;
use cli_clipboard::ClipboardContext;
use std::path::PathBuf;
use tokio::{sync::mpsc, task::JoinHandle};

use crate::application::state::{AppMessage, AppMode, StartupResult};
use crate::database::DatabaseManager;
use crate::table_viewer::TableViewer;
use crate::ui::UI;
use crate::{
    config::Config,
    matrix::MatrixAnimation,
    query_parser::{QueryBlock, QueryParser},
};

pub struct App {
    pub config: Config,
    pub db_manager: DatabaseManager,
    pub current_mode: AppMode,
    pub previous_mode: Option<AppMode>,
    pub current_file: Option<PathBuf>,
    pub file_content: String,
    pub matrix_animation: Option<MatrixAnimation>,
    pub table_viewer: Option<TableViewer>,
    pub foreign_key_viewer: Option<TableViewer>,
    pub search_input: String,
    pub search_cursor_position: u16,
    pub movement_multiplier: Option<usize>,
    pub ui: UI,
    pub should_quit: bool,
    pub cursor_line: usize,
    pub scroll_offset: usize,
    pub yank_sequence: String,
    pub is_querying: bool,
    pub clipboard: ClipboardContext,
    pub startup_complete: bool,
    pub pending_query: Option<String>,
    pub file_loading_complete: bool,
    pub startup_task: Option<JoinHandle<Result<StartupResult>>>,
    pub cell_info: Option<crate::table_viewer::CellInfo>,
    pub query_parser: QueryParser,
    pub query_blocks: Vec<QueryBlock>,
    pub app_tx: mpsc::Sender<AppMessage>,
    pub app_rx: mpsc::Receiver<AppMessage>,
}

impl App {
    pub async fn perform_startup_tasks_sync(&mut self) -> Result<()> {
        if let Some(file_path) = &self.current_file.clone() {
            self.load_file(file_path).await?;
        }
        self.file_loading_complete = true;

        if let Some(query) = &self.pending_query.clone() {
            self.execute_query_internal(query).await?;
        } else {
            self.current_mode = AppMode::FileView;
        }

        self.startup_complete = true;
        Ok(())
    }

    pub fn spawn_startup_tasks(&self) -> JoinHandle<Result<StartupResult>> {
        let current_file = self.current_file.clone();
        let pending_query = self.pending_query.clone();
        let config = self.config.clone();
        let db_manager = self.db_manager.clone();

        tokio::spawn(async move {
            let mut result = StartupResult {
                file_content: None,
                query_blocks: Vec::new(),
                table_viewer: None,
                success_message: None,
                error_message: None,
            };

            let query_parser = QueryParser::new();

            if let Some(file_path) = &current_file {
                match std::fs::read_to_string(file_path) {
                    Ok(content) => {
                        match query_parser.parse_query_blocks(&content) {
                            Ok(blocks) => {
                                result.query_blocks = blocks;
                                let msg = format!("Loaded and parsed file {}", file_path.display());
                                result.success_message = Some(msg);
                            }
                            Err(e) => {
                                result.error_message = Some(format!("Failed to parse file: {}", e));
                            }
                        }
                        result.file_content = Some(content);
                    }
                    Err(e) => {
                        result.error_message = Some(format!("Failed to load file: {}", e));
                    }
                }
            }

            if let Some(query) = &pending_query &&
                !query.trim().is_empty() {
                    match db_manager.execute_query(query).await {
                        Ok(query_result) => {
                            match TableViewer::new(query_result, &config, &db_manager) {
                                Ok(viewer) => {
                                    result.table_viewer = Some(viewer);
                                    result.success_message =
                                        Some("Query executed successfully".to_string());
                                }
                                Err(e) => {
                                    result.error_message =
                                        Some(format!("Failed to create table viewer: {}", e));
                                }
                            }
                        }
                        Err(e) => {
                            result.error_message = Some(format!("Query execution failed: {}", e));
                        }
                    }
                }

            Ok(result)
        })
    }
}

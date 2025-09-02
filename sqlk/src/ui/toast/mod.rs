use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
};
use ratatui::{
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use std::time::{Duration, Instant};

use crate::ui::UI;

#[derive(Debug, Clone)]
pub struct ToastMessage {
    pub message: String,
    pub message_type: ToastType,
    pub created_at: Instant,
    pub is_hovered: bool,
    pub position: ToastPosition,
}

#[derive(Debug, Clone)]
pub enum ToastType {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub enum ToastPosition {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    Center,
}

impl ToastMessage {
    pub fn new(message: String, toast_type: ToastType) -> Self {
        Self {
            message,
            message_type: toast_type,
            created_at: Instant::now(),
            is_hovered: false,
            position: ToastPosition::TopRight,
        }
    }

    pub fn with_position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    pub fn should_show(&self, hover_mouse_pos: Option<(u16, u16)>, toast_area: Rect) -> bool {
        let elapsed = self.created_at.elapsed();
        let base_duration = Duration::from_secs(3);

        if let Some((mouse_x, mouse_y)) = hover_mouse_pos {
            let is_hovering = mouse_x >= toast_area.x
                && mouse_x < toast_area.x + toast_area.width
                && mouse_y >= toast_area.y
                && mouse_y < toast_area.y + toast_area.height;

            if is_hovering {
                return true;
            }
        }

        elapsed < base_duration
    }

    pub fn get_style(&self) -> Style {
        match self.message_type {
            ToastType::Info => Style::default().fg(Color::Cyan).bg(Color::Black),
            ToastType::Success => Style::default().fg(Color::Green).bg(Color::Black),
            ToastType::Warning => Style::default().fg(Color::Yellow).bg(Color::Black),
            ToastType::Error => Style::default().fg(Color::Red).bg(Color::Black),
        }
    }

    pub fn get_icon(&self) -> &str {
        match self.message_type {
            ToastType::Info => "ℹ",
            ToastType::Success => "✓",
            ToastType::Warning => "⚠",
            ToastType::Error => "✗",
        }
    }

    pub fn get_opacity_style(&self) -> Style {
        let elapsed = self.created_at.elapsed();
        let fade_start = Duration::from_millis(2500);

        let base_style = self.get_style();

        if elapsed > fade_start {
            base_style.add_modifier(Modifier::DIM)
        } else {
            base_style.add_modifier(Modifier::BOLD)
        }
    }
}

impl UI {
    pub fn add_toast(&mut self, message: String, toast_type: ToastType) {
        let toast = ToastMessage::new(message, toast_type);
        self.toast_messages.push(toast);

        if self.toast_messages.len() > 5 {
            self.toast_messages.remove(0);
        }
    }

    pub fn update_toasts(&mut self, screen_size: Rect) {
        let mouse_pos = self.last_mouse_position;

        self.toast_messages.retain(|toast| {
            let toast_area = Self::calculate_toast_area_static(&toast.position, 0, screen_size);
            toast.should_show(mouse_pos, toast_area)
        });
    }

    pub fn render_toast_notifications(&self, f: &mut Frame) {
        for (index, toast) in self.toast_messages.iter().enumerate() {
            let toast_area = self.calculate_toast_area(&toast.position, index, f.area());

            if toast_area.width < 10 || toast_area.height < 1 {
                continue;
            }

            f.render_widget(Clear, toast_area);

            let elapsed = toast.created_at.elapsed();
            let is_fresh = elapsed < Duration::from_millis(500);

            let toast_content = format!("{} {}", toast.get_icon(), toast.message);

            let mut style = toast.get_opacity_style();

            if is_fresh {
                style = style.add_modifier(Modifier::RAPID_BLINK);
            }

            let is_hovered = if let Some((mouse_x, mouse_y)) = self.last_mouse_position {
                mouse_x >= toast_area.x
                    && mouse_x < toast_area.x + toast_area.width
                    && mouse_y >= toast_area.y
                    && mouse_y < toast_area.y + toast_area.height
            } else {
                false
            };

            if is_hovered {
                style = style.bg(Color::DarkGray).add_modifier(Modifier::BOLD);
            }

            let border_style = if is_hovered {
                Style::default().fg(Color::White)
            } else {
                match toast.message_type {
                    ToastType::Error => Style::default().fg(Color::Red),
                    ToastType::Warning => Style::default().fg(Color::Yellow),
                    ToastType::Success => Style::default().fg(Color::Green),
                    ToastType::Info => Style::default().fg(Color::Cyan),
                }
            };

            let toast_widget = Paragraph::new(toast_content)
                .style(style)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(border_style),
                )
                .wrap(Wrap { trim: true });

            f.render_widget(toast_widget, toast_area);
        }
    }

    fn calculate_toast_area_static(
        position: &ToastPosition,
        index: usize,
        screen_size: Rect,
    ) -> Rect {
        let width = 50.min(screen_size.width.saturating_sub(4));
        let height = 5;
        let margin = 1;
        let spacing = height + margin;

        match position {
            ToastPosition::TopRight => Rect {
                x: screen_size.width.saturating_sub(width + 2),
                y: 1 + (index as u16 * spacing),
                width,
                height,
            },
            ToastPosition::TopLeft => Rect {
                x: 2,
                y: 1 + (index as u16 * spacing),
                width,
                height,
            },
            ToastPosition::BottomRight => Rect {
                x: screen_size.width.saturating_sub(width + 2),
                y: screen_size
                    .height
                    .saturating_sub(height + 1 + (index as u16 * spacing)),
                width,
                height,
            },
            ToastPosition::BottomLeft => Rect {
                x: 2,
                y: screen_size
                    .height
                    .saturating_sub(height + 1 + (index as u16 * spacing)),
                width,
                height,
            },
            ToastPosition::Center => {
                let center_x = screen_size.width / 2;
                let center_y = screen_size.height / 2;
                Rect {
                    x: center_x.saturating_sub(width / 2),
                    y: center_y.saturating_sub(height / 2) + (index as u16 * spacing),
                    width,
                    height,
                }
            }
        }
    }

    fn calculate_toast_area(
        &self,
        position: &ToastPosition,
        index: usize,
        screen_size: Rect,
    ) -> Rect {
        let width = 50.min(screen_size.width.saturating_sub(4));
        let height = 5;
        let margin = 1;
        let spacing = height + margin;

        match position {
            ToastPosition::TopRight => Rect {
                x: screen_size.width.saturating_sub(width + 2),
                y: 1 + (index as u16 * spacing),
                width,
                height,
            },
            ToastPosition::TopLeft => Rect {
                x: 2,
                y: 1 + (index as u16 * spacing),
                width,
                height,
            },
            ToastPosition::BottomRight => Rect {
                x: screen_size.width.saturating_sub(width + 2),
                y: screen_size
                    .height
                    .saturating_sub(height + 1 + (index as u16 * spacing)),
                width,
                height,
            },
            ToastPosition::BottomLeft => Rect {
                x: 2,
                y: screen_size
                    .height
                    .saturating_sub(height + 1 + (index as u16 * spacing)),
                width,
                height,
            },
            ToastPosition::Center => {
                let center_x = screen_size.width / 2;
                let center_y = screen_size.height / 2;
                Rect {
                    x: center_x.saturating_sub(width / 2),
                    y: center_y.saturating_sub(height / 2) + (index as u16 * spacing),
                    width,
                    height,
                }
            }
        }
    }
}

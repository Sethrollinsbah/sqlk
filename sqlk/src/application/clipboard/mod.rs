use crate::ui::{ToastType, UI};
use anyhow::Result;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub struct ClipboardManager {
    clipboard: ClipboardContext,
}

impl ClipboardManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            clipboard: ClipboardContext::new().expect("Failed to get clipboard ctx"),
        })
    }

    pub fn copy_text(&mut self, text: &str, ui: &mut UI) {
        match self.clipboard.set_contents(text.to_string()) {
            Ok(_) => {
                ui.add_toast(format!("Yanked: {}", text), ToastType::Success);
            }
            Err(e) => {
                ui.add_toast(
                    format!("Failed to copy to clipboard: {}", e),
                    ToastType::Error,
                );
            }
        }
    }

    pub fn copy_row(&mut self, headers: &[String], values: &[String], ui: &mut UI) {
        let formatted_row = headers
            .iter()
            .zip(values.iter())
            .map(|(header, value)| format!("{}: {}", header, value))
            .collect::<Vec<String>>()
            .join(", ");

        self.copy_text(&formatted_row, ui);
    }

    pub fn copy_cell(&mut self, value: &str, ui: &mut UI) {
        self.copy_text(value, ui);
    }
}

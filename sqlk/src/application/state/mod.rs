use anyhow::Result;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use tokio::sync::mpsc;

use crate::application::app::App;
use crate::args::Args;
use crate::database::DatabaseManager;
use crate::table_viewer::TableViewer;
use crate::ui::UI;
use crate::{
    config::Config,
    query_parser::{QueryBlock, QueryParser},
};

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    FileView,
    TableViewer,
    MatrixLoading,
    Help,
    ForeignKeyView,
    CellInfoView,
    Searching,
}

#[derive(Debug)]
pub struct StartupResult {
    pub file_content: Option<String>,
    pub table_viewer: Option<TableViewer>,
    pub success_message: Option<String>,
    pub query_blocks: Vec<QueryBlock>,
    pub error_message: Option<String>,
}

pub enum AppMessage {
    QueryResult(Result<TableViewer, String>),
}

impl App {
    pub fn get_visible_file_content(&self, height: usize) -> Vec<String> {
        let lines: Vec<&str> = self.file_content.lines().collect();
        let start = self.scroll_offset;
        let end = (start + height).min(lines.len());
        lines[start..end].iter().map(|s| s.to_string()).collect()
    }
    pub fn get_current_query_block(&self) -> Option<&QueryBlock> {
        let current_line_1_based = self.cursor_line + 1;
        self.query_parser
            .find_query_at_line(&self.query_blocks, current_line_1_based)
    }

    pub async fn new(args: Args) -> Result<Self> {
        let config = Config::load(&args.env)?;
        let db_manager = DatabaseManager::new(&config).await?;

        let (app_tx, app_rx) = mpsc::channel(1);
        let app = Self {
            config,
            db_manager,
            current_mode: AppMode::MatrixLoading,
            previous_mode: None,
            current_file: args.file.clone(),
            file_content: String::new(),
            matrix_animation: None,
            table_viewer: None,
            foreign_key_viewer: None,
            search_input: String::new(),
            movement_multiplier: None,
            ui: UI::new(),
            should_quit: false,
            cursor_line: 0,
            app_tx,
            app_rx,
            scroll_offset: 0,
            yank_sequence: String::new(),
            clipboard: ClipboardContext::new().unwrap(),
            startup_complete: false,
            pending_query: args.query.clone(),
            file_loading_complete: false,
            startup_task: None,
            cell_info: None,
            query_parser: QueryParser::new(),
            query_blocks: Vec::new(),
            is_querying: false,
            search_cursor_position: 0
        };

        Ok(app)
    }

    pub fn adjust_scroll(&mut self) {
        let visible_lines = 20;
        if self.cursor_line < self.scroll_offset {
            self.scroll_offset = self.cursor_line;
        } else if self.cursor_line >= self.scroll_offset + visible_lines {
            self.scroll_offset = self.cursor_line.saturating_sub(visible_lines - 1);
        }
    }
}

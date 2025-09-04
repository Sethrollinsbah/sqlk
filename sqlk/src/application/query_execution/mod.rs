use anyhow::Result;

use crate::application::app::App;
use crate::application::state::{AppMessage, AppMode};
use crate::database::DatabaseManager;
use crate::table_viewer::TableViewer;
use crate::ui::ToastType;

impl App {
    pub fn execute_query(&mut self, query: &str) -> Result<()> {
        self.is_querying = true;

        if self.config.matrix.enabled && self.startup_complete {
            self.ui
                .add_toast("Executing query".to_string(), ToastType::Info);
        }

        let query_string = query.to_string();
        let config = self.config.clone();
        let app_tx = self.app_tx.clone();
        let db_man_clone = self.db_manager.clone();

        tokio::spawn(async move {
            let db_manager = db_man_clone
                .get_or_try_init(|| async {
                    DatabaseManager::new(&config).await
                })
                .await
                .map_err(|e| anyhow::anyhow!("Database manager initialization failed: {}", e))
                .expect("Failed to get db_manager");
            let result = match db_manager.execute_query(&query_string).await {
                Ok(query_result) => match TableViewer::new(query_result, &config, db_manager) {
                    Ok(viewer) => Ok(viewer),
                    Err(e) => Err(format!("Failed to create table viewer: {}", e)),
                },
                Err(e) => Err(format!("Query execution failed: {}", e)),
            };

            let _ = app_tx.send(AppMessage::QueryResult(result)).await;
        });

        Ok(())
    }

    pub async fn execute_query_internal(&mut self, query: &str) -> Result<()> {
        if let Err(e) = self.validate_query(query) {
            self.ui
                .add_toast(format!("Query validation failed: {}", e), ToastType::Error);
            self.is_querying = false;
            return Ok(());
        }
        let db_manager = self.db_manager
            .get_or_try_init(|| async {
                DatabaseManager::new(&self.config).await
            })
            .await
            .map_err(|e| anyhow::anyhow!("Database manager initialization failed: {}", e))
            .expect("Failed to get db_manager");

        match db_manager.execute_query(query).await {
            Ok(result) => match TableViewer::new(result, &self.config, db_manager) {
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
                    self.ui.add_toast(
                        format!("Failed to create table viewer: {}", e),
                        ToastType::Error,
                    );
                    self.current_mode = AppMode::FileView;
                }
            },
            Err(e) => {
                self.ui
                    .add_toast(format!("Query execution failed: {}", e), ToastType::Error);
                self.current_mode = AppMode::FileView;
                self.matrix_animation = None;
            }
        }
        self.is_querying = false;
        Ok(())
    }

    fn validate_query(&self, query: &str) -> Result<()> {
        if query.trim().is_empty() {
            Err(anyhow::anyhow!("Query is empty"))
        } else {
            Ok(())
        }
    }

    pub async fn execute_current_query(&mut self) -> Result<()> {
        if let Some(query_block) = self.get_current_query_block() {
            let query_text = query_block.text.clone();
            if !query_text.trim().is_empty() {
                self.execute_query(&query_text)?;
            } else {
                self.ui.add_toast(
                    "No query found at current cursor position".to_string(),
                    ToastType::Error,
                );
            }
        } else {
            self.ui.add_toast(
                "No query found at current cursor position".to_string(),
                ToastType::Error,
            );
        }
        Ok(())
    }
}

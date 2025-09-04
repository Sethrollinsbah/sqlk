use anyhow::Result;

use crate::{
    application::app::App,
    database::{DatabaseManager, ForeignKeyInfo, QueryResult},
    table_viewer::TableViewer,
};

#[derive(Debug)]
pub struct ForeignKeyLookupResult {
    pub foreign_key_info: ForeignKeyInfo,
    pub lookup_data: QueryResult,
}

impl TableViewer {
    pub async fn lookup_foreign_key_info(
        &self,
        app: &App,
    ) -> Result<Option<ForeignKeyLookupResult>> {
        let current_cell_value = self.get_current_cell_value();
        let header_name = self.data.headers.get(self.current_col).cloned();

        if let (Some(value), Some(header)) = (current_cell_value, header_name) {
            if value.is_empty() || value == "NULL" {
                return Ok(None);
            }

            let db_man_clone = app.db_manager.clone();
            // Use `get_or_try_init` on the cloned `OnceCell` inside the spawned task.
            let db_manager = db_man_clone
                .get_or_try_init(|| async {
                    // This is the correct way to initialize the DatabaseManager.
                    DatabaseManager::new(&app.config).await
                })
                .await
                .map_err(|e| anyhow::anyhow!("Database manager initialization failed: {}", e))
                .expect("Failed to get db_manager");

            if let Some(fk_info) = self.foreign_keys.get(&self.current_col) {
                match db_manager.lookup_foreign_key(&header, &value).await {
                    Ok(lookup_result) => Ok(Some(ForeignKeyLookupResult {
                        foreign_key_info: fk_info.clone(),
                        lookup_data: lookup_result,
                    })),
                    Err(_) => Ok(None),
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn lookup_foreign_key(&self, app: &App) -> Result<Option<TableViewer>> {
        let current_cell_value = self.get_current_cell_value();
        let header_name = self.data.headers.get(self.current_col).cloned();

        if let (Some(value), Some(header)) = (current_cell_value, header_name) {
            if value.is_empty() || value == "NULL" {
                return Ok(None);
            }

            let db_man_clone = app.db_manager.clone();
            // Use `get_or_try_init` on the cloned `OnceCell` inside the spawned task.
            let db_manager = db_man_clone
                .get_or_try_init(|| async {
                    // This is the correct way to initialize the DatabaseManager.
                    DatabaseManager::new(&app.config).await
                })
                .await
                .map_err(|e| anyhow::anyhow!("Database manager initialization failed: {}", e))
                .expect("Failed to get db_manager");

            let result = db_manager.lookup_foreign_key(&header, &value).await?;
            let new_viewer = TableViewer::new(result, &app.config, db_manager)?;
            Ok(Some(new_viewer))
        } else {
            Ok(None)
        }
    }
}

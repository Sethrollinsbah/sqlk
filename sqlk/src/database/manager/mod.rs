use crate::{
    config::Config,
    database::{ForeignKeyInfo, QueryResult},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait DatabaseClient: Send + Sync {
    async fn execute_query(&self, query: &str) -> Result<QueryResult>;
    async fn lookup_foreign_key(&self, column_name: &str, value: &str) -> Result<QueryResult>;
    fn get_foreign_key_info(&self, column_name: &str) -> Result<&ForeignKeyInfo>;
}

#[derive(Clone)]
pub struct DatabaseManager {
    client: Arc<dyn DatabaseClient>,
}

impl DatabaseManager {
    pub async fn new(config: &Config) -> Result<Self> {
        let client = create_database_client(config).await?;
        Ok(Self { client })
    }

    pub async fn execute_query(&self, query: &str) -> Result<QueryResult> {
        self.client.execute_query(query).await
    }

    pub async fn lookup_foreign_key(&self, column_name: &str, value: &str) -> Result<QueryResult> {
        self.client.lookup_foreign_key(column_name, value).await
    }

    pub fn get_foreign_key_info(&self, column_name: &str) -> Result<&ForeignKeyInfo> {
        self.client.get_foreign_key_info(column_name)
    }
}

async fn create_database_client(config: &Config) -> Result<Arc<dyn DatabaseClient>> {
    let postgres_client = crate::database::postgres::PostgresClient::new(config).await?;
    Ok(Arc::new(postgres_client))
}

use super::formatting::PostgresFormatter;
use crate::config::Config;
use crate::database::{DatabaseClient, ForeignKeyInfo, QueryResult, SchemaCache};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::Row;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone)]
pub struct PostgresClient {
    pool: PgPool,
    schema_cache: SchemaCache,
    formatter: PostgresFormatter,
}

impl PostgresClient {
    pub async fn new(config: &Config) -> Result<Self> {
        let database_url = config
            .get_database_url()
            .ok_or_else(|| anyhow::anyhow!("No DATABASE_URL found"))?;

        let pool = PgPool::connect(database_url).await?;
        let formatter = PostgresFormatter::new();

        let mut client = Self {
            pool,
            schema_cache: SchemaCache::new(),
            formatter,
        };

        if config.foreign_keys.enabled {
            client.schema_cache = client.analyze_schema().await?;
        }

        Ok(client)
    }

    async fn analyze_schema(&self) -> Result<HashMap<String, Vec<ForeignKeyInfo>>> {
        let mut schema_map: HashMap<String, Vec<ForeignKeyInfo>> = HashMap::new();
        let query = r#"
            SELECT 
                kcu.column_name, 
                ccu.table_name AS foreign_table_name, 
                ccu.column_name AS foreign_column_name
            FROM 
                information_schema.table_constraints AS tc 
                JOIN information_schema.key_column_usage AS kcu
                  ON tc.constraint_name = kcu.constraint_name
                  AND tc.table_schema = kcu.table_schema
                JOIN information_schema.constraint_column_usage AS ccu
                  ON ccu.constraint_name = tc.constraint_name
                  AND ccu.table_schema = tc.table_schema
            WHERE tc.constraint_type = 'FOREIGN KEY' AND tc.table_schema = 'public';
        "#;

        let rows = sqlx::query(query).fetch_all(&self.pool).await?;

        for row in rows {
            let fk_info = ForeignKeyInfo {
                column_name: row.get("column_name"),
                referenced_table: row.get("foreign_table_name"),
                referenced_column: row.get("foreign_column_name"),
            };
            schema_map
                .entry(fk_info.column_name.clone())
                .or_default()
                .push(fk_info);
        }

        Ok(schema_map)
    }
}

#[async_trait]
impl DatabaseClient for PostgresClient {
    async fn execute_query(&self, query: &str) -> Result<QueryResult> {
        let start_time = Instant::now();
        let rows = sqlx::query(query).fetch_all(&self.pool).await?;
        self.formatter.rows_to_query_result(rows, start_time)
    }

    async fn lookup_foreign_key(&self, column_name: &str, value: &str) -> Result<QueryResult> {
        let fk_info = self.get_foreign_key_info(column_name)?;

        let query_string = format!(
            "SELECT * FROM \"{}\" WHERE \"{}\" = $1 LIMIT 10",
            fk_info.referenced_table, fk_info.referenced_column
        );

        let start_time = Instant::now();

        let rows = if let Ok(int_value) = value.parse::<i64>() {
            sqlx::query(&query_string)
                .bind(int_value)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query(&query_string)
                .bind(value)
                .fetch_all(&self.pool)
                .await?
        };

        self.formatter.rows_to_query_result(rows, start_time)
    }

    fn get_foreign_key_info(&self, column_name: &str) -> Result<&ForeignKeyInfo> {
        self.schema_cache
            .get(column_name)
            .and_then(|fks| fks.first())
            .ok_or_else(|| anyhow::anyhow!("No foreign key info for column: {}", column_name))
    }
}

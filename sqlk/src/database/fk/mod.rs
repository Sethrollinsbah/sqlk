use crate::database::*;

pub trait ForeignKeyAnalyzer {
    fn analyze_foreign_keys(&self) -> anyhow::Result<SchemaCache>;
}

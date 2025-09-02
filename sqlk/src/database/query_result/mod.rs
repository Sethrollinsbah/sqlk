use crate::database::QueryResult;

impl QueryResult {
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn get_column_index(&self, column_name: &str) -> Option<usize> {
        self.headers.iter().position(|h| h == column_name)
    }
}

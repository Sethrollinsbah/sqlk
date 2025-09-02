use regex::Regex;

pub struct QueryAnalyzer;

impl QueryAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn extract_table_names(&self, query: &str) -> Vec<String> {
        let mut tables = Vec::new();
        let query_lower = query.to_lowercase();

        let re = Regex::new(
            r"(?:from|join|update|insert\s+into|alter\s+table)\s+((?:[\w_]+\.)?[\w_]+)(?:\s+(?:as\s+)?[\w_]+)?"
        ).unwrap();

        for captures in re.captures_iter(&query_lower) {
            if let Some(table_name) = captures.get(1) {
                let table = table_name.as_str().to_string();
                if self.is_valid_table_name(&table) && !tables.contains(&table) {
                    tables.push(table);
                }
            }
        }
        tables
    }

    pub fn get_query_type(&self, query: &str) -> QueryType {
        let upper_query = query.trim().to_uppercase();

        if upper_query.starts_with("SELECT") {
            QueryType::Select
        } else if upper_query.starts_with("INSERT") {
            QueryType::Insert
        } else if upper_query.starts_with("UPDATE") {
            QueryType::Update
        } else if upper_query.starts_with("DELETE") {
            QueryType::Delete
        } else if upper_query.starts_with("CREATE") {
            QueryType::Create
        } else if upper_query.starts_with("DROP") {
            QueryType::Drop
        } else if upper_query.starts_with("ALTER") {
            QueryType::Alter
        } else {
            QueryType::Other
        }
    }

    fn is_valid_table_name(&self, table: &str) -> bool {
        ![
            "select", "where", "group", "order", "having", "limit", "offset", "union", "with",
        ]
        .contains(&table)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
    Create,
    Drop,
    Alter,
    Other,
}

impl Default for QueryAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

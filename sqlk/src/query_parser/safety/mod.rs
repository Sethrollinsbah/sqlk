use regex::Regex;

pub struct SafetyChecker;

impl SafetyChecker {
    pub fn new() -> Self {
        Self
    }

    /// Checks if a query contains potentially destructive SQL commands.
    pub fn is_dangerous_query(&self, query: &str) -> bool {
        let dangerous_patterns = [
            r"\bDROP\s+(DATABASE|SCHEMA|TABLE|INDEX|VIEW)\b",
            r"\bTRUNCATE\s+TABLE\b",
            r"\bDELETE\s+FROM\b",
            r"\bUPDATE\b.*\bSET\b",
            r"\bALTER\s+(TABLE|DATABASE|SCHEMA)\b",
        ];

        let upper_query = query.to_uppercase();
        dangerous_patterns.iter().any(|pattern| {
            if let Ok(re) = Regex::new(pattern) {
                re.is_match(&upper_query)
            } else {
                false
            }
        })
    }

    pub fn get_danger_level(&self, query: &str) -> DangerLevel {
        let upper_query = query.to_uppercase();

        if self.contains_pattern(&upper_query, r"\bDROP\s+(DATABASE|SCHEMA)\b") {
            DangerLevel::Critical
        } else if self.contains_pattern(&upper_query, r"\bDROP\s+(TABLE|INDEX|VIEW)\b")
            || self.contains_pattern(&upper_query, r"\bTRUNCATE\s+TABLE\b")
        {
            DangerLevel::High
        } else if self.contains_pattern(&upper_query, r"\bDELETE\s+FROM\b")
            || self.contains_pattern(&upper_query, r"\bUPDATE\b.*\bSET\b")
        {
            DangerLevel::Medium
        } else if self.contains_pattern(&upper_query, r"\bALTER\s+(TABLE|DATABASE|SCHEMA)\b") {
            DangerLevel::Low
        } else {
            DangerLevel::Safe
        }
    }

    pub fn get_safety_warnings(&self, query: &str) -> Vec<String> {
        let mut warnings = Vec::new();
        let upper_query = query.to_uppercase();

        if self.contains_pattern(&upper_query, r"\bDROP\s+") {
            warnings.push(
                "Contains DROP statement - will permanently delete data/structure".to_string(),
            );
        }

        if self.contains_pattern(&upper_query, r"\bTRUNCATE\s+") {
            warnings.push("Contains TRUNCATE statement - will delete all table data".to_string());
        }

        if self.contains_pattern(&upper_query, r"\bDELETE\s+FROM\b")
            && !self.contains_pattern(&upper_query, r"\bWHERE\b")
        {
            warnings.push("DELETE without WHERE clause - will delete all rows".to_string());
        }

        if self.contains_pattern(&upper_query, r"\bUPDATE\b.*\bSET\b")
            && !self.contains_pattern(&upper_query, r"\bWHERE\b")
        {
            warnings.push("UPDATE without WHERE clause - will modify all rows".to_string());
        }

        warnings
    }

    fn contains_pattern(&self, query: &str, pattern: &str) -> bool {
        if let Ok(re) = Regex::new(pattern) {
            re.is_match(query)
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DangerLevel {
    Safe,
    Low,
    Medium,
    High,
    Critical,
}

impl Default for SafetyChecker {
    fn default() -> Self {
        Self::new()
    }
}

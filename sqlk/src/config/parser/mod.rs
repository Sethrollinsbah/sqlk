pub struct DatabaseUrlParser;

impl DatabaseUrlParser {
    pub fn parse_from_line(line: &str) -> Option<String> {
        let line = line.trim();

        if let Some(equals_pos) = line.find('=') {
            let key_part = line[..equals_pos].trim();
            if key_part == "DATABASE_URL" {
                let value_part = line[equals_pos + 1..].trim();
                return Self::extract_value(value_part);
            }
        }

        None
    }

    fn extract_value(value_part: &str) -> Option<String> {
        let trimmed = value_part.trim();

        if trimmed.starts_with('"')
            && trimmed.ends_with('"')
            && trimmed.len() >= 2
            && trimmed.starts_with('\'')
            && trimmed.ends_with('\'')
            && trimmed.len() >= 2
        {
            Some(trimmed[1..trimmed.len() - 1].to_string())
        } else if !trimmed.is_empty() {
            Some(trimmed.to_string())
        } else {
            None
        }
    }

    pub fn validate_url(url: &str) -> Result<(), String> {
        if url.is_empty() {
            return Err("Database URL cannot be empty".to_string());
        }

        if url.starts_with('"') || url.ends_with('"') {
            return Err(
                "Database URL contains quotes - check your .env file formatting".to_string(),
            );
        }

        if url.starts_with('\'') || url.ends_with('\'') {
            return Err(
                "Database URL contains single quotes - check your .env file formatting".to_string(),
            );
        }

        if !url.contains("://") {
            return Err("Invalid URL format: missing protocol".to_string());
        }

        Ok(())
    }
}

pub struct StringUtils;

impl StringUtils {
    pub fn is_empty_query(query: &str) -> bool {
        for line in query.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("--") {
                return false;
            }
        }
        true
    }

    pub fn clean_query(query: &str) -> String {
        query
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with("--"))
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn normalize_whitespace(text: &str) -> String {
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }
}

/// Represents a block of executable SQL text found in a file.
#[derive(Debug, Clone)]
pub struct QueryBlock {
    /// The actual SQL query text.
    pub text: String,
    /// The starting line number of the query block in the file (1-based).
    pub start_line: usize,
    /// The ending line number of the query block in the file (1-based).
    pub end_line: usize,
}

impl QueryBlock {
    pub fn new(text: String, start_line: usize, end_line: usize) -> Self {
        Self {
            text,
            start_line,
            end_line,
        }
    }

    pub fn contains_line(&self, line_number: usize) -> bool {
        line_number >= self.start_line && line_number <= self.end_line
    }

    pub fn is_empty(&self) -> bool {
        self.text.trim().is_empty()
    }
}

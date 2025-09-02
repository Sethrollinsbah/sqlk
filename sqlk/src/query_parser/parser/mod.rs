use anyhow::Result;

use crate::query_parser::{block::QueryBlock, utils::StringUtils};

#[derive(Clone)]
pub struct QueryParser;

impl QueryParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_query_blocks(&self, content: &str) -> Result<Vec<QueryBlock>> {
        let mut queries = Vec::new();
        let mut current_query_lines: Vec<&str> = Vec::new();
        let mut start_line: Option<usize> = None;
        let mut in_transaction_block = false;

        for (i, line) in content.lines().enumerate() {
            let line_number = i + 1;
            let trimmed_line = line.trim();

            if self.should_skip_line(trimmed_line, &current_query_lines) {
                continue;
            }

            if start_line.is_none() {
                start_line = Some(line_number);
            }

            if self.is_transaction_start(trimmed_line) {
                in_transaction_block = true;
            }

            current_query_lines.push(line);

            if self.should_end_query_block(trimmed_line, in_transaction_block) {
                if let Some(query_block) = self.build_query_block(
                    &current_query_lines,
                    start_line.unwrap(),
                    line_number,
                    trimmed_line,
                ) {
                    queries.push(query_block);
                }

                current_query_lines.clear();
                start_line = None;
                in_transaction_block = false;
            }
        }

        if !current_query_lines.is_empty()
            && let Some(query_block) = self.build_final_query_block(
                &current_query_lines,
                start_line.unwrap_or(1),
                content.lines().count(),
            ) {
                queries.push(query_block);
            }

        Ok(queries)
    }

    pub fn find_query_at_line<'a>(
        &self,
        queries: &'a [QueryBlock],
        line_number: usize,
    ) -> Option<&'a QueryBlock> {
        queries
            .iter()
            .find(|query| query.contains_line(line_number))
    }

    fn should_skip_line(&self, trimmed_line: &str, current_lines: &[&str]) -> bool {
        (trimmed_line.is_empty() || trimmed_line.starts_with("--")) && current_lines.is_empty()
    }

    fn is_transaction_start(&self, line: &str) -> bool {
        line.to_uppercase().starts_with("BEGIN")
    }

    fn should_end_query_block(&self, line: &str, in_transaction: bool) -> bool {
        let upper_line = line.to_uppercase();
        let is_commit_or_rollback =
            upper_line.starts_with("COMMIT;") || upper_line.starts_with("ROLLBACK;");

        (self.line_ends_statement(line) && !in_transaction) || is_commit_or_rollback
    }

    fn line_ends_statement(&self, line: &str) -> bool {
        let trimmed = line.trim_end();

        if let Some(semicolon_pos) = trimmed.rfind(';') {
            let after_semicolon = &trimmed[semicolon_pos + 1..].trim();
            after_semicolon.is_empty() || after_semicolon.starts_with("--")
        } else {
            false
        }
    }

    fn build_query_block(
        &self,
        lines: &[&str],
        start_line: usize,
        end_line: usize,
        last_line: &str,
    ) -> Option<QueryBlock> {
        let mut query_text = lines.join("\n").trim().to_string();

        let upper_last = last_line.to_uppercase();
        if (upper_last.starts_with("COMMIT;")
            || upper_last.starts_with("ROLLBACK;")
            || query_text.ends_with(';'))
            && query_text.ends_with(';')
        {
            query_text.pop();
        }

        if !StringUtils::is_empty_query(&query_text) {
            Some(QueryBlock::new(query_text, start_line, end_line))
        } else {
            None
        }
    }

    fn build_final_query_block(
        &self,
        lines: &[&str],
        start_line: usize,
        end_line: usize,
    ) -> Option<QueryBlock> {
        let query_text = lines.join("\n").trim().to_string();
        let first_line = query_text.lines().next().unwrap_or("").trim();

        if !first_line.is_empty() && !first_line.starts_with("--") {
            Some(QueryBlock::new(query_text, start_line, end_line))
        } else {
            None
        }
    }
}

impl Default for QueryParser {
    fn default() -> Self {
        Self::new()
    }
}

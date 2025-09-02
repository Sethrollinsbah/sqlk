use crate::table_viewer::{CellPosition, TableViewer};

#[derive(Debug, Default)]
pub struct SearchState {
    pub term: String,
    pub matches: Vec<CellPosition>,
    pub current_match: usize,
}

impl TableViewer {
    pub fn search(&mut self, term: &str) {
        self.search_state.term = term.to_string();
        self.search_state.matches.clear();
        self.search_state.current_match = 0;

        if term.is_empty() {
            return;
        }

        let search_term_lower = term.to_lowercase();
        for (row_idx, row) in self.data.rows.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if cell.to_lowercase().contains(&search_term_lower) {
                    self.search_state.matches.push(CellPosition {
                        row: row_idx,
                        col: col_idx,
                    });
                }
            }
        }

        if !self.search_state.matches.is_empty() {
            self.jump_to_match(0);
        }
    }

    pub fn next_search_match(&mut self, offset: usize) {
        if !self.search_state.matches.is_empty() {
            let next_match =
                (self.search_state.current_match + offset) % self.search_state.matches.len();
            self.jump_to_match(next_match);
            self.search_state.current_match = next_match;
        }
    }

    pub fn prev_search_match(&mut self, offset: usize) {
        if !self.search_state.matches.is_empty() {
            let prev_match = if self.search_state.current_match < offset {
                self.search_state
                    .matches
                    .len()
                    .saturating_sub(offset - self.search_state.current_match)
            } else {
                self.search_state.current_match - offset
            };
            self.jump_to_match(prev_match);
            self.search_state.current_match = prev_match;
        }
    }

    fn jump_to_match(&mut self, match_idx: usize) {
        if let Some(position) = self.search_state.matches.get(match_idx) {
            self.current_row = position.row;
            self.current_col = position.col;
            self.update_scroll();
        }
    }

    pub fn get_visible_search_matches(
        &self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> Vec<CellPosition> {
        self.search_state
            .matches
            .iter()
            .filter(|pos| {
                pos.row >= start_row
                    && pos.row < end_row
                    && pos.col >= start_col
                    && pos.col < end_col
            })
            .map(|pos| CellPosition {
                row: pos.row - start_row,
                col: pos.col - start_col,
            })
            .collect()
    }
}

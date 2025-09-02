pub struct TextOverlay {
    block_text: Vec<Vec<char>>,
}

impl TextOverlay {
    pub fn new() -> Self {
        let sqlk_block_text = vec![
            "  ███████   ██████  ██       ██   ██".chars().collect(),
            " ██     ██ ██    ██ ██       ██  ██ ".chars().collect(),
            " ██        ██    ██ ██       ██ ██  ".chars().collect(),
            "  ███████  ██    ██ ██       ████   ".chars().collect(),
            "        ██ ██ ▄▄ ██ ██       ██ ██  ".chars().collect(),
            " ██     ██ ██ ██ ██ ██       ██  ██ ".chars().collect(),
            "  ███████   ██████  ███████  ██   ██".chars().collect(),
            "               ██                  ".chars().collect(),
            "               ██                  ".chars().collect(),
        ];

        Self {
            block_text: sqlk_block_text,
        }
    }

    pub fn with_text(text_lines: Vec<&str>) -> Self {
        let block_text = text_lines
            .into_iter()
            .map(|line| line.chars().collect())
            .collect();

        Self { block_text }
    }

    pub fn get_char_at(
        &self,
        row: usize,
        col: usize,
        screen_width: usize,
        screen_height: usize,
    ) -> Option<char> {
        let text_height = self.block_text.len();
        let text_width = if text_height > 0 {
            self.block_text[0].len()
        } else {
            0
        };

        let start_row = screen_height.saturating_sub(text_height) / 2;
        let start_col = screen_width.saturating_sub(text_width) / 2;

        if row >= start_row
            && row < start_row + text_height
            && col >= start_col
            && col < start_col + text_width
        {
            let text_row = row - start_row;
            let text_col = col - start_col;

            if text_row < self.block_text.len() && text_col < self.block_text[text_row].len() {
                Some(self.block_text[text_row][text_col])
            } else {
                Some(' ')
            }
        } else {
            None
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        let height = self.block_text.len();
        let width = if height > 0 {
            self.block_text[0].len()
        } else {
            0
        };
        (width, height)
    }

    pub fn is_text_position(
        &self,
        row: usize,
        col: usize,
        screen_width: usize,
        screen_height: usize,
    ) -> bool {
        self.get_char_at(row, col, screen_width, screen_height)
            .is_some()
    }
}

impl Default for TextOverlay {
    fn default() -> Self {
        Self::new()
    }
}

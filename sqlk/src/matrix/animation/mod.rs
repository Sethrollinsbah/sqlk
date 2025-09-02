use crate::{
    config::MatrixConfig,
    matrix::{ColorCalculator, MatrixColumn, TextOverlay},
};
use rand::Rng;
use ratatui::style::Color;
use std::time::{Duration, Instant};

pub struct MatrixAnimation {
    columns: Vec<MatrixColumn>,
    config: MatrixConfig,
    start_time: Instant,
    matrix_chars: Vec<char>,
    text_overlay: TextOverlay,
    color_calculator: ColorCalculator,
}

impl MatrixAnimation {
    pub fn new(config: &MatrixConfig) -> Self {
        let mut rng = rand::rng();
        let matrix_chars = config.get_character_set();

        let mut columns = Vec::new();
        for _col_idx in 0..config.width {
            let initial_length = rng.random_range(0..=(config.height / 3));
            columns.push(MatrixColumn::new(initial_length as usize, &matrix_chars));
        }

        Self {
            columns,
            config: config.clone(),
            start_time: Instant::now(),
            matrix_chars,
            text_overlay: TextOverlay::new(),
            color_calculator: ColorCalculator::new(),
        }
    }

    pub fn with_custom_text(config: &MatrixConfig, text_lines: Vec<&str>) -> Self {
        let mut animation = Self::new(config);
        animation.text_overlay = TextOverlay::with_text(text_lines);
        animation
    }

    pub fn update(&mut self) {
        for column in &mut self.columns {
            column.update(&self.matrix_chars);
        }
    }

    pub fn get_frame(&self) -> Vec<String> {
        let mut lines = vec![String::new(); self.config.height as usize];

        (0..self.config.height as usize).for_each(|row_idx| {
            let mut line = String::new();
            for column in &self.columns {
                if let Some(char_data) = column.get_char_at(row_idx) {
                    line.push(char_data.character);
                } else {
                    line.push(' ');
                }
            }
            lines[row_idx] = line;
        });
        lines
    }

    pub fn get_char_color(&self, row: usize, col: usize) -> Color {
        if let Some(column) = self.columns.get(col) {
            if let Some(char_data) = column.get_char_at(row) {
                self.color_calculator
                    .get_char_color(char_data, column.intensity())
            } else {
                Color::Black
            }
        } else {
            Color::Black
        }
    }

    pub fn get_overlay_char_at(&self, row: usize, col: usize) -> Option<char> {
        self.text_overlay.get_char_at(
            row,
            col,
            self.config.width as usize,
            self.config.height as usize,
        )
    }

    pub fn get_overlay_color(&self) -> Color {
        self.color_calculator.get_overlay_color(self.get_progress())
    }

    pub fn is_finished(&self) -> bool {
        self.start_time.elapsed() >= Duration::from_millis(self.config.duration_ms)
    }

    pub fn get_progress(&self) -> f32 {
        let elapsed = self.start_time.elapsed().as_millis() as f32;
        let total = self.config.duration_ms as f32;
        (elapsed / total).min(1.0)
    }

    pub fn reset(&mut self) {
        self.start_time = Instant::now();
        let mut rng = rand::rng();

        for column in &mut self.columns {
            let initial_length = rng.random_range(0..=(self.config.height / 3));
            *column = MatrixColumn::new(initial_length as usize, &self.matrix_chars);
        }
    }

    pub fn get_dimensions(&self) -> (u16, u16) {
        (self.config.width, self.config.height)
    }
}

use ratatui::style::Color;

use crate::matrix::MatrixChar;

pub struct ColorCalculator;

impl ColorCalculator {
    pub fn new() -> Self {
        Self
    }

    pub fn get_char_color(&self, char_data: &MatrixChar, column_intensity: f32) -> Color {
        let max_age = 20.0;
        let fade_factor = 1.0 - (char_data.age as f32 / max_age).min(1.0);
        let intensity_adjusted_fade = fade_factor * column_intensity;

        if char_data.is_head && char_data.age == 0 {
            // Bright white for the very front character
            Color::White
        } else if intensity_adjusted_fade > 0.8 {
            // Bright green for characters just behind the head
            Color::Rgb(0, 255, 0)
        } else if intensity_adjusted_fade > 0.6 {
            // Medium bright green
            Color::Rgb(0, 220, 0)
        } else if intensity_adjusted_fade > 0.4 {
            // Medium green
            Color::Rgb(0, 180, 0)
        } else if intensity_adjusted_fade > 0.2 {
            // Darker green
            Color::Rgb(0, 120, 0)
        } else {
            // Very dark green for the tail
            Color::Rgb(0, 80, 0)
        }
    }

    pub fn get_overlay_color(&self, progress: f32) -> Color {
        if progress < 0.2 {
            Color::White
        } else if progress < 0.5 {
            Color::Rgb(0, 255, 100)
        } else if progress < 0.8 {
            Color::Rgb(0, 200, 150)
        } else {
            Color::Rgb(0, 150, 200)
        }
    }

    pub fn interpolate_color(&self, from: Color, to: Color, factor: f32) -> Color {
        match (from, to) {
            (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
                let r = r1 as f32 + (r2 as f32 - r1 as f32) * factor;
                let g = g1 as f32 + (g2 as f32 - g1 as f32) * factor;
                let b = b1 as f32 + (b2 as f32 - b1 as f32) * factor;
                Color::Rgb(r as u8, g as u8, b as u8)
            }
            _ => from,
        }
    }
}

impl Default for ColorCalculator {
    fn default() -> Self {
        Self::new()
    }
}

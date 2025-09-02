use crate::config::MatrixConfig;

impl MatrixConfig {
    pub fn for_full_screen(terminal_width: u16, terminal_height: u16) -> Self {
        Self {
            width: terminal_width,
            height: terminal_height,
            duration_ms: 4000,
            chars: Self::default_matrix_chars(),
            enabled: true,
        }
    }

    pub fn for_size(width: u16, height: u16, duration_ms: u64) -> Self {
        Self {
            width,
            height,
            duration_ms,
            chars: Self::default_matrix_chars(),
            enabled: true,
        }
    }

    fn default_matrix_chars() -> String {
        "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲンabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()←↑→↓∞§¤".to_string()
    }

    pub fn get_character_set(&self) -> Vec<char> {
        self.chars.chars().collect()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.width == 0 || self.height == 0 {
            return Err("Width and height must be greater than 0".to_string());
        }
        if self.chars.is_empty() {
            return Err("Character set cannot be empty".to_string());
        }
        Ok(())
    }
}

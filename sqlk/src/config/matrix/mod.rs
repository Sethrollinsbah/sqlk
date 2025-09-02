use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixConfig {
    pub enabled: bool,
    pub duration_ms: u64,
    pub chars: String,
    pub width: u16,
    pub height: u16,
}

impl Default for MatrixConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            duration_ms: 6000,
            chars: "ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾀﾇﾍ01".to_string(),
            width: 50,
            height: 12,
        }
    }
}

impl MatrixConfig {
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    pub fn with_chars(mut self, chars: String) -> Self {
        self.chars = chars;
        self
    }

    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
}

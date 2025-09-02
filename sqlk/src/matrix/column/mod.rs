use rand::Rng;

use crate::matrix::MatrixChar;

#[derive(Debug)]
pub struct MatrixColumn {
    chars: Vec<MatrixChar>,
    speed: u8,
    counter: u8,
    intensity: f32,
    max_trail_length: usize,
}

impl MatrixColumn {
    pub fn new(initial_length: usize, matrix_chars: &[char]) -> Self {
        let mut rng = rand::rng();
        let mut initial_chars = Vec::new();

        for i in 0..initial_length {
            let random_char = matrix_chars[rng.random_range(0..matrix_chars.len())];
            initial_chars.push(MatrixChar::new_with_age(random_char, i as u8));
        }

        if let Some(first_char) = initial_chars.get_mut(0) {
            first_char.mark_as_head();
        }

        Self {
            chars: initial_chars,
            speed: rng.random_range(2..=6),
            counter: rng.random_range(0..15),
            intensity: rng.random_range(0.7..1.0),
            max_trail_length: 15,
        }
    }

    pub fn update(&mut self, matrix_chars: &[char]) {
        let mut rng = rand::rng();

        self.counter += 1;
        if self.counter < self.speed {
            return;
        }

        self.counter = 0;

        if let Some(first_char) = self.chars.get_mut(0) {
            first_char.mark_as_tail();
        }

        let random_char = matrix_chars[rng.random_range(0..matrix_chars.len())];
        self.chars.insert(0, MatrixChar::new(random_char));

        for ch in &mut self.chars {
            ch.age_character();
        }

        self.chars
            .retain(|ch| !ch.is_expired(self.max_trail_length as u8));

        if rng.random_range(0..100) < 5 {
            self.intensity = rng.random_range(0.3..1.0);
        }
    }

    pub fn get_char_at(&self, row: usize) -> Option<&MatrixChar> {
        self.chars.get(row)
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }
}

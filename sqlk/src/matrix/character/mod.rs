#[derive(Debug, Clone)]
pub struct MatrixChar {
    pub character: char,
    pub age: u8,
    pub is_head: bool,
}

impl MatrixChar {
    pub fn new(character: char) -> Self {
        Self {
            character,
            age: 0,
            is_head: true,
        }
    }

    pub fn new_with_age(character: char, age: u8) -> Self {
        Self {
            character,
            age,
            is_head: false,
        }
    }

    pub fn age_character(&mut self) {
        self.age += 1;
        self.is_head = false;
    }

    pub fn mark_as_head(&mut self) {
        self.is_head = true;
    }

    pub fn mark_as_tail(&mut self) {
        self.is_head = false;
    }

    pub fn is_expired(&self, max_age: u8) -> bool {
        self.age >= max_age
    }
}

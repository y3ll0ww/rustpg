use colored::*;

#[derive(Debug, Clone)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Default
}

#[derive(Debug, Clone)]
pub struct ColoredCharacter {
    character: char,
    color: Color,
    level: usize
}

impl ColoredCharacter {
    pub fn new(character: char, color: Color, level: usize) -> Self {
        Self {
            character,
            color,
            level
        }
    }

    pub fn default() -> Self {
        Self {
            character: ' ',
            color: Color::Default,
            level: 0
        }
    }

    pub fn clear(&mut self) {
        self.character = ' ';
        self.color = Color::Default;
        self.level = 0;
    }

    pub fn level(&self) -> usize {
        self.level.clone()
    }

    pub fn render(&self) {
        let c = self.character;

        match self.color {
            Color::Red => print!("{}", draw(c).red()),
            Color::Blue => print!("{}", draw(c).blue()),
            Color::Green => print!("{}", draw(c).green()),
            Color::Yellow => print!("{}", draw(c).yellow()),
            _ => print!("{c}")
        }
    }
}

fn draw(c: char) -> String {
    format!("{}", c)
}
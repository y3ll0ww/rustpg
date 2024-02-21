use super::characters::{Color, ColoredCharacter};

pub struct ActiveObjects {
    objects: Vec<Object>
}

impl ActiveObjects {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Object> {
        self.objects.iter()
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn rmv(&mut self, name: &str) {
        self.objects.retain(|obj| obj.name() != name);
    }
}

impl IntoIterator for ActiveObjects {
    type Item = Object;
    type IntoIter = std::vec::IntoIter<Object>;

    fn into_iter(self) -> Self::IntoIter {
        self.objects.into_iter()
    }
}

#[derive(Debug)]
pub struct Object {
    name: String,
    x: usize,
    y: usize,
    level: usize,
    body: Vec<Vec<ColoredCharacter>>
}

impl Object {
    pub fn new(name: String, body: Vec<Vec<ColoredCharacter>>) -> Self {
        Self {
            name,
            x: 0,
            y: 0,
            level: 0,
            body
        }
    }

    pub fn from_str(name: String, body: String, level: usize, x: usize, y: usize, color: Color) -> Self {
        let mut result = Vec::new();

        let rows = body.split("\n");
        for row in rows {
            let mut r = Vec::new();

            for c in row.chars() {
                r.push(ColoredCharacter::new(c, color.clone(), level));
            }

            result.push(r);
        }

        Self {
            name,
            x,
            y,
            level,
            body: result
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn level(&self) -> usize {
        self.level.clone()
    }

    fn height(&self) -> usize {
        self.body.len()
    }

    fn width(&self, y: usize) -> usize {
        if y < self.body.len() {
            return self.body[y].len()
        }
        0
    }

    pub fn pixel(&self, y: usize, x: usize) -> Option<ColoredCharacter> {
        if y < self.height() && x < self.width(y) {
            return Some(self.body[y][x].clone())
        }
        None
    } 

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn pos(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn change_lvl(&mut self, level: usize) {
        self.level = level;
    }

    pub fn render(&self) {
        let mut pos_x = self.x;
        let mut pos_y = self.y;

        for vec in &self.body {
            for c in vec {
                c.render();

                pos_x = pos_x + 1;
            }
            pos_y = pos_y + 1;
        }
    }
}
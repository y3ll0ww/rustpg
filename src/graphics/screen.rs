use super::characters::ColoredCharacter;
use super::objects::{ActiveObjects, Object};

pub struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<Vec<ColoredCharacter>>
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::with_capacity(height);

        for _ in 0..height {
            let row = vec![ColoredCharacter::default(); width];
            pixels.push(row);
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn clear(&mut self) {
        for vec in &mut self.pixels {
            for c in vec.iter_mut() {
                c.clear();
            }
        }
    }

    pub fn destroy(&mut self, objects: ActiveObjects, name: &str) {
        for obj in objects {
            if obj.name() == name {
                // Do something
            }
        }
    }

    pub fn render(&mut self, objects: ActiveObjects) {
        // Clear the screen
        self.clear();
    
        // Collect objects for each row of pixels
        let objects_by_row: Vec<Vec<&Object>> = (0..self.height)
            .map(|index| {
                objects
                    .iter()
                    .filter(|obj| obj.y() == index)
                    .collect()
            })
            .collect();
    
        // Iterate through the vertical axis of the screen
        for (y_index, vec) in self.pixels.iter_mut().enumerate() {
            // Iterate through the objects relevant to this row
            for obj in &objects_by_row[y_index] {
                // Iterate through the horizontal axis of the screen
                for (x_index, pixel) in vec.iter_mut().enumerate() {
                    // Check if spot is occupied
                    if let Some(new_pixel) = obj.pixel(y_index, x_index) {
                        // Check if level of new pixel is higher
                        if pixel.level() < new_pixel.level() {
                            new_pixel.render();
                        }
                    }
                }
            }
        }
    }
}
use crate::tile::*;
use std::fmt;

#[derive(Debug)]
pub struct Map {
    pub tiles: Vec<Tile>,
    pub width: u32,
    pub height: u32,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        let mut tiles = Vec::<Tile>::new();

        for row in 0..height {
            for column in 0..width {
                tiles.push(Tile::new(column, row));
            }
        }

        Self {
            tiles,
            width,
            height,
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tiles = String::new();

        for row in 0..self.height {
            for column in 0..self.width {
                tiles.push_str(
                    self.tiles[((row * self.width) + column) as usize]
                        .to_string()
                        .as_str(),
                );
            }

            tiles.push('\n');
        }
        write!(f, "{tiles}")
    }
}

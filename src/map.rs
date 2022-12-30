use crate::tile::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Map {
    pub tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = Vec::<Tile>::new();

        for y in 0..height as usize {
            for x in 0..width as usize {
                let mut tile = Tile::new();

                if y > 0 {
                    tile.north = Some(((y - 1) * width) + x);

                    tiles[((y - 1) * width) + x].south = Some((y * width) + x);
                }

                if x > 0 {
                    tile.west = Some((y * width) + x - 1);

                    tiles[(y * width) + x - 1].east = Some((y * width) + x);
                }

                tiles.push(tile);
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

        for y in 0..self.height {
            for x in 0..self.width {
                tiles.push_str(self.tiles[(y * self.width) + x].to_string().as_str());
            }

            tiles.push('\n');
        }
        write!(f, "{tiles}")
    }
}

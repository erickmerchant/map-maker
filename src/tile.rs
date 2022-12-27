use std::fmt;

#[derive(Debug)]
pub struct Tile {
    pub tile_type: Option<char>,
    pub row: u32,
    pub column: u32,
}

impl Tile {
    pub fn new(column: u32, row: u32) -> Self {
        Self {
            tile_type: None,
            column,
            row,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let character = match &self.tile_type {
            Some(land) => land.to_string(),
            None => " ".to_string(),
        };
        write!(f, "{character}")
    }
}

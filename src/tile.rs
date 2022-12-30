use owo_colors::OwoColorize;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub tile_type: Option<char>,
    pub north: Option<usize>,
    pub south: Option<usize>,
    pub east: Option<usize>,
    pub west: Option<usize>,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            tile_type: None,
            north: None,
            south: None,
            east: None,
            west: None,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let character = match &self.tile_type {
            Some(land) => match land {
                'x' => 'x'.white().to_string(),
                '.' => '.'.yellow().to_string(),
                'o' => 'o'.green().to_string(),
                '~' => '~'.blue().to_string(),
                _ => " ".to_string(),
            },
            None => " ".to_string(),
        };
        write!(f, "{character}")
    }
}

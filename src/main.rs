use clap::Parser;
use owo_colors::OwoColorize;
use std::fmt;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    #[arg(short = 'H', long, default_value_t = 10)]
    height: u32,

    #[arg(short = 'W', long, default_value_t = 10)]
    width: u32,

    #[arg(short = 'S', long, default_value_t = Args::get_default_seed())]
    seed: String,
}

impl Args {
    fn get_default_seed() -> String {
        "xo.~".to_string()
    }
}

#[derive(Debug)]
enum TileType {
    // red .
    Desert,
    // white x
    Mountain,
    // green o
    Forest,
    // blue ~
    Water,
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let character = match *self {
            Self::Desert => ".".red().to_string(),
            Self::Mountain => "x".white().to_string(),
            Self::Forest => ".".green().to_string(),
            Self::Water => ".".blue().to_string(),
        };
        write!(f, "{character}")
    }
}

#[derive(Debug)]
struct Tile {
    tile_type: Option<TileType>,
    row: u32,
    column: u32,
}

impl Tile {
    fn new(column: u32, row: u32) -> Self {
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

#[derive(Debug)]
struct Map {
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
}

impl Map {
    fn new(width: u32, height: u32) -> Self {
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

fn main() {
    let args = Args::parse();

    let map = Map::new(args.width, args.height);

    print!("{map}");
}

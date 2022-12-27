pub mod args;
pub mod map;
pub mod tile;

use args::*;
use clap::Parser;
use map::*;
use rand::seq::SliceRandom;
use tile::*;

fn main() {
    let args = Args::parse();

    let mut map = Map::new(args.width, args.height);
    let mut fill_iterations = 0;

    'fill: loop {
        for character in args.seed.chars() {
            let mut filtered_tiles = map
                .tiles
                .iter_mut()
                .filter(|tile| {
                    // if fill_iterations == 0 {
                    tile.tile_type.is_none()
                    // } else {
                    //     tile.tile_type.is_none() && has_neighbor(tile, character)
                    // }
                })
                .collect::<Vec<&mut Tile>>();

            if filtered_tiles.is_empty() {
                break 'fill;
            }

            let mut tile = filtered_tiles.choose_mut(&mut rand::thread_rng()).unwrap();

            tile.tile_type = Some(character);
        }

        fill_iterations += 1;
    }

    print!("{map}");
}

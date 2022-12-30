pub mod args;
pub mod map;
pub mod tile;

use args::*;
use clap::Parser;
use map::*;
use rand::seq::SliceRandom;

fn main() {
    let args = Args::parse();

    let mut map = Map::new(args.width, args.height);
    let mut fill_iterations = 0;

    loop {
        let mut break_it_count = 0;

        for character in args.seed.chars() {
            let mut filtered_tiles = Vec::<usize>::new();

            for i in 0..map.tiles.len() {
                let tile = map.tiles[i];
                let mut push_it = false;

                if fill_iterations == 0 {
                    if tile.tile_type.is_none() {
                        push_it = true;
                    }
                } else if tile.tile_type.is_none() {
                    if let Some(north) = tile.north {
                        if Some(character) == map.tiles[north].tile_type {
                            push_it = true;
                        }
                    }

                    if let Some(south) = tile.south {
                        if Some(character) == map.tiles[south].tile_type {
                            push_it = true;
                        }
                    }

                    if let Some(east) = tile.east {
                        if Some(character) == map.tiles[east].tile_type {
                            push_it = true;
                        }
                    }

                    if let Some(west) = tile.west {
                        if Some(character) == map.tiles[west].tile_type {
                            push_it = true;
                        }
                    }
                }

                if push_it {
                    filtered_tiles.push(i);
                }
            }

            if filtered_tiles.is_empty() {
                break_it_count += 1;
            } else {
                let tile = filtered_tiles.choose_mut(&mut rand::thread_rng()).unwrap();

                map.tiles[*tile].tile_type = Some(character);
            }
        }

        if break_it_count == args.seed.chars().count() {
            break;
        }

        fill_iterations += 1;
    }

    print!("{map}");
}

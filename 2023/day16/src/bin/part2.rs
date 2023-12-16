use std::{collections::HashSet, vec};

#[derive(PartialEq, Clone, Eq, Hash, Copy, Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(PartialEq, Debug)]
enum Tile {
    Empty,               // '.'
    HorizontalSplitter,  // '-'
    VerticalSplitter,    // '|'
    RightDiagonalMirror, // '/'
    LeftDiagonalMirror,  // '\'
}

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '-' => Tile::HorizontalSplitter,
                    '|' => Tile::VerticalSplitter,
                    '/' => Tile::RightDiagonalMirror,
                    '\\' => Tile::LeftDiagonalMirror,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut starting_tiles: Vec<(u32, u32, Direction)> = vec![];
    for i in 0..grid.len() {
        starting_tiles.push((i as u32, 0, Direction::Right));
        starting_tiles.push((i as u32, (grid[0].len() - 1) as u32, Direction::Left));
    }
    for i in 0..grid[0].len() {
        starting_tiles.push((0, i as u32, Direction::Down));
        starting_tiles.push(((grid.len() - 1) as u32, i as u32, Direction::Up));
    }
    let max_count = starting_tiles
        .iter()
        .map(|tile| count_energized_tiles(&grid, *tile))
        .max()
        .unwrap();
    println!("{max_count}");
}

fn count_energized_tiles(grid: &[Vec<Tile>], starting_tile: (u32, u32, Direction)) -> usize {
    let total_rows = grid.len();
    let total_cols = grid[0].len();

    let mut beam_ends: Vec<(u32, u32, Direction)> = vec![starting_tile];
    let mut energized_tiles: HashSet<(u32, u32, Direction)> = HashSet::new();
    energized_tiles.insert(starting_tile);

    while let Some(beam_end) = beam_ends.pop() {
        match grid[beam_end.0 as usize][beam_end.1 as usize] {
            Tile::Empty => {
                if let Some(next_tile) = get_neighbor_for_direction(
                    beam_end.0, beam_end.1, beam_end.2, total_rows, total_cols,
                ) {
                    let next_tile = (next_tile.0, next_tile.1, beam_end.2);
                    if energized_tiles.insert(next_tile) {
                        beam_ends.push(next_tile);
                    }
                }
            }
            Tile::LeftDiagonalMirror => {
                let next_direction = match beam_end.2 {
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Down,
                    Direction::Left => Direction::Up,
                    Direction::Down => Direction::Right,
                };
                let next_tile = get_neighbor_for_direction(
                    beam_end.0,
                    beam_end.1,
                    next_direction,
                    total_rows,
                    total_cols,
                );
                if let Some(next_tile) = next_tile {
                    let next_tile = (next_tile.0, next_tile.1, next_direction);
                    if energized_tiles.insert(next_tile) {
                        beam_ends.push(next_tile);
                    }
                }
            }
            Tile::RightDiagonalMirror => {
                let next_direction = match beam_end.2 {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Left,
                };
                let next_tile = get_neighbor_for_direction(
                    beam_end.0,
                    beam_end.1,
                    next_direction,
                    total_rows,
                    total_cols,
                );
                if let Some(next_tile) = next_tile {
                    let next_tile = (next_tile.0, next_tile.1, next_direction);
                    if energized_tiles.insert(next_tile) {
                        beam_ends.push(next_tile);
                    }
                }
            }
            Tile::HorizontalSplitter => {
                let next_directions: Vec<Direction> = match beam_end.2 {
                    Direction::Up | Direction::Down => vec![Direction::Right, Direction::Left],
                    dir => vec![dir],
                };
                for next_direction in next_directions {
                    let next_tile = get_neighbor_for_direction(
                        beam_end.0,
                        beam_end.1,
                        next_direction,
                        total_rows,
                        total_cols,
                    );
                    if let Some(next_tile) = next_tile {
                        let next_tile = (next_tile.0, next_tile.1, next_direction);
                        if energized_tiles.insert(next_tile) {
                            beam_ends.push(next_tile);
                        }
                    }
                }
            }
            Tile::VerticalSplitter => {
                let next_directions: Vec<Direction> = match beam_end.2 {
                    Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
                    dir => vec![dir],
                };
                for next_direction in next_directions {
                    let next_tile = get_neighbor_for_direction(
                        beam_end.0,
                        beam_end.1,
                        next_direction,
                        total_rows,
                        total_cols,
                    );
                    if let Some(next_tile) = next_tile {
                        let next_tile = (next_tile.0, next_tile.1, next_direction);
                        if energized_tiles.insert(next_tile) {
                            beam_ends.push(next_tile);
                        }
                    }
                }
            }
        }
    }

    let unique_energized_tiles = energized_tiles
        .iter()
        .map(|tile| (tile.0, tile.1))
        .collect::<HashSet<(u32, u32)>>();

    unique_energized_tiles.len()
}

fn get_neighbor_for_direction(
    row: u32,
    col: u32,
    direction: Direction,
    total_rows: usize,
    total_cols: usize,
) -> Option<(u32, u32)> {
    let offset = get_neighbor_offset_for_direction(direction);
    let next_tile_row = row as i32 + offset.0;
    let next_tile_col = col as i32 + offset.1;
    if next_tile_row >= 0
        && next_tile_row < total_rows as i32
        && next_tile_col >= 0
        && next_tile_col < total_cols as i32
    {
        return Some((next_tile_row as u32, next_tile_col as u32));
    }
    None
}

fn get_neighbor_offset_for_direction(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

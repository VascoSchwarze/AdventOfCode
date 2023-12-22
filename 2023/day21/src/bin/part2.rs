use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{input}\n");

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start: (u32, u32) = (0, 0);
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == 'S' {
                start = (row_idx as u32, col_idx as u32);
            }
        }
    }

    // calculate the distance of each tile in the map from the starting tile
    let mut visited_tiles: HashMap<(u32, u32), u32> = HashMap::new();
    visited_tiles.insert(start, 0);
    let mut next_tiles_to_visit = VecDeque::new();
    next_tiles_to_visit.push_back(start);
    while let Some(tile) = next_tiles_to_visit.pop_front() {
        for neighbor in get_steppable_neighbors(tile, &grid) {
            if !visited_tiles.contains_key(&neighbor) {
                let cur_tile_distance = visited_tiles
                    .get(&tile)
                    .expect("Current tile should be contained");
                visited_tiles.insert(neighbor, cur_tile_distance + 1);
                next_tiles_to_visit.push_back(neighbor);
            }
        }
    }

    // Detailed explanation at: https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    // (This solution also is specific for the actual input as it has certain special properties. Therefore, it doesn't work for the test input.)
    let even_corners = visited_tiles
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = visited_tiles
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    let even_full = visited_tiles.values().filter(|v| **v % 2 == 0).count();
    let odd_full = visited_tiles.values().filter(|v| **v % 2 == 1).count();

    let n = (26501365 - (grid.len() / 2)) / grid.len();
    assert_eq!(n, 202300);

    let result = ((n + 1) * (n + 1)) * odd_full + (n * n) * even_full - (n + 1) * odd_corners
        + n * even_corners;

    println!("{result}");
}

fn get_steppable_neighbors(tile: (u32, u32), grid: &[Vec<char>]) -> Vec<(u32, u32)> {
    let mut neighbors = vec![];
    if tile.0 > 0 && grid[(tile.0 - 1) as usize][tile.1 as usize] != '#' {
        neighbors.push((tile.0 - 1, tile.1));
    }
    if tile.0 + 1 < grid.len() as u32 && grid[(tile.0 + 1) as usize][tile.1 as usize] != '#' {
        neighbors.push((tile.0 + 1, tile.1));
    }
    if tile.1 > 0 && grid[tile.0 as usize][(tile.1 - 1) as usize] != '#' {
        neighbors.push((tile.0, tile.1 - 1));
    }
    if tile.1 + 1 < grid[0].len() as u32 && grid[tile.0 as usize][(tile.1 + 1) as usize] != '#' {
        neighbors.push((tile.0, tile.1 + 1));
    }
    neighbors
}

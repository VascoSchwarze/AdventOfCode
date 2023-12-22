use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
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

    let step_count = 64;

    let mut current_step_reachable_tiles = HashSet::new();
    current_step_reachable_tiles.insert(start);

    let mut next_step_reachable_tiles = HashSet::new();
    for _ in 0..step_count {
        for tile in current_step_reachable_tiles {
            for neighbor in get_steppable_neighbors(tile, &grid) {
                next_step_reachable_tiles.insert(neighbor);
            }
        }
        current_step_reachable_tiles = next_step_reachable_tiles;
        next_step_reachable_tiles = HashSet::new();
    }

    let result = current_step_reachable_tiles.len();
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

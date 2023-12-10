#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Horizontal,
    Vertical,
}

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    // construct the map
    let mut tiles: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => Tile::NorthSouth,
                    '-' => Tile::EastWest,
                    'L' => Tile::NorthEast,
                    'J' => Tile::NorthWest,
                    '7' => Tile::SouthWest,
                    'F' => Tile::SouthEast,
                    '.' => Tile::Ground,
                    'S' => Tile::Start,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    // find the starting tile
    let mut loop_tiles: Vec<(u32, u32)> = vec![];
    let start_tile = tiles
        .iter()
        .enumerate()
        .find_map(|row| {
            let tile_in_row = row.1.iter().enumerate().find(|t| Tile::Start == *t.1);
            tile_in_row.map(|start_tile_in_row| (row.0 as u32, start_tile_in_row.0 as u32))
        })
        .expect("Should contain a start tile");
    loop_tiles.push(start_tile);

    let mut cur_tile = start_tile;
    // find a neighbor of the starting tile that connects to the starting tile
    for neighbor in get_neighbor_coords(&tiles, start_tile.0, start_tile.1) {
        let neighbors_of_neighbor = get_neighbor_coords(&tiles, neighbor.0, neighbor.1);
        let is_neighbor_connecting = neighbors_of_neighbor.iter().any(|n| *n == start_tile);
        if is_neighbor_connecting {
            loop_tiles.push(neighbor);
            cur_tile = neighbor;
            break;
        }
    }

    let mut prev_tile = start_tile;
    // follow the connections to the neighbors until the start tile is reached again
    while cur_tile != start_tile {
        for neighbor in get_neighbor_coords(&tiles, cur_tile.0, cur_tile.1) {
            if prev_tile == neighbor {
                continue;
            } else {
                loop_tiles.push(neighbor);
                prev_tile = cur_tile;
                cur_tile = neighbor;
                break;
            }
        }
    }

    // replace Start tile with its actual tile type
    let start_tile_neighbors = (loop_tiles[1], loop_tiles[loop_tiles.len() - 2]);
    let (neighbor1, neighbor2) = start_tile_neighbors;
    let start_tile_type: Tile = match (
        start_tile.0 as i32 - neighbor1.0 as i32,
        start_tile.1 as i32 - neighbor1.1 as i32,
    ) {
        (1, 0) => {
            match (
                start_tile.0 as i32 - neighbor2.0 as i32,
                start_tile.1 as i32 - neighbor2.1 as i32,
            ) {
                (1, 0) => panic!(),
                (-1, 0) => Tile::NorthSouth,
                (0, 1) => Tile::NorthWest,
                (0, -1) => Tile::NorthEast,
                _ => panic!(),
            }
        }
        (-1, 0) => {
            match (
                start_tile.0 as i32 - neighbor2.0 as i32,
                start_tile.1 as i32 - neighbor2.1 as i32,
            ) {
                (1, 0) => Tile::NorthSouth,
                (-1, 0) => panic!(),
                (0, 1) => Tile::SouthWest,
                (0, -1) => Tile::SouthEast,
                _ => panic!(),
            }
        }
        (0, 1) => {
            match (
                start_tile.0 as i32 - neighbor2.0 as i32,
                start_tile.1 as i32 - neighbor2.1 as i32,
            ) {
                (1, 0) => Tile::NorthWest,
                (-1, 0) => Tile::SouthWest,
                (0, 1) => panic!(),
                (0, -1) => Tile::EastWest,
                _ => panic!(),
            }
        }
        (0, -1) => {
            match (
                start_tile.0 as i32 - neighbor2.0 as i32,
                start_tile.1 as i32 - neighbor2.1 as i32,
            ) {
                (1, 0) => Tile::NorthEast,
                (-1, 0) => Tile::SouthEast,
                (0, 1) => Tile::EastWest,
                (0, -1) => panic!(),
                _ => panic!(),
            }
        }
        _ => panic!(),
    };
    tiles[start_tile.0 as usize][start_tile.1 as usize] = start_tile_type;

    let mut inside_loop_count = 0;
    // check for all tiles in the grid (except loop tiles) whether a condition is met so that they are in the loop
    for (row_idx, row) in tiles.iter().enumerate() {
        println!("{row_idx}");
        for (col_idx, _) in row.iter().enumerate() {
            if loop_tiles.contains(&(row_idx as u32, col_idx as u32)) {
                continue;
            }
            let all_tiles_in_all_directions = get_all_loop_tiles_in_all_directions(
                &tiles,
                &loop_tiles,
                row_idx as u32,
                col_idx as u32,
            );
            //condition: check that pipes cross an odd amount of times in all directions from this tile
            if all_tiles_in_all_directions
                .iter()
                .all(|all_tiles_in_one_direction| {
                    let crossings = count_crossings_in_direction(
                        &all_tiles_in_one_direction.0,
                        &all_tiles_in_one_direction.1,
                    );

                    crossings % 2 == 1
                })
            {
                inside_loop_count += 1;
            }
        }
    }

    println!("{inside_loop_count}");
}

fn count_crossings_in_direction(direction: &Direction, tiles: &[Tile]) -> u32 {
    let mut crossings = 0;

    let mut prev_half_crossing: Option<Tile> = None;

    if *direction == Direction::Horizontal {
        for tile in tiles {
            match tile {
                Tile::NorthSouth => crossings += 1,
                Tile::EastWest => continue,
                Tile::NorthEast | Tile::NorthWest => {
                    if let Some(prev_half_crossing_tile) = prev_half_crossing {
                        if prev_half_crossing_tile == Tile::SouthEast
                            || prev_half_crossing_tile == Tile::SouthWest
                        {
                            crossings += 1;
                        }
                        prev_half_crossing = None;
                    } else {
                        prev_half_crossing = Some(*tile);
                    }
                }
                Tile::SouthEast | Tile::SouthWest => {
                    if let Some(prev_half_crossing_tile) = prev_half_crossing {
                        if prev_half_crossing_tile == Tile::NorthEast
                            || prev_half_crossing_tile == Tile::NorthWest
                        {
                            crossings += 1;
                        }
                        prev_half_crossing = None;
                    } else {
                        prev_half_crossing = Some(*tile);
                    }
                }
                Tile::Ground | Tile::Start => panic!(),
            }
        }
    } else {
        for tile in tiles {
            match tile {
                Tile::NorthSouth => continue,
                Tile::EastWest => crossings += 1,
                Tile::NorthEast | Tile::SouthEast => {
                    if let Some(prev_half_crossing_tile) = prev_half_crossing {
                        if prev_half_crossing_tile == Tile::NorthWest
                            || prev_half_crossing_tile == Tile::SouthWest
                        {
                            crossings += 1;
                        }
                        prev_half_crossing = None;
                    } else {
                        prev_half_crossing = Some(*tile);
                    }
                }
                Tile::NorthWest | Tile::SouthWest => {
                    if let Some(prev_half_crossing_tile) = prev_half_crossing {
                        if prev_half_crossing_tile == Tile::NorthEast
                            || prev_half_crossing_tile == Tile::SouthEast
                        {
                            crossings += 1;
                        }
                        prev_half_crossing = None;
                    } else {
                        prev_half_crossing = Some(*tile);
                    }
                }
                Tile::Ground | Tile::Start => panic!(),
            }
        }
    }

    crossings
}

fn get_all_loop_tiles_in_all_directions(
    grid: &[Vec<Tile>],
    loop_tiles: &[(u32, u32)],
    row: u32,
    col: u32,
) -> [(Direction, Vec<Tile>); 4] {
    let mut tiles_in_directions = [
        (Direction::Vertical, vec![]),
        (Direction::Vertical, vec![]),
        (Direction::Horizontal, vec![]),
        (Direction::Horizontal, vec![]),
    ];
    for (idx, offsets) in [(1, 0), (-1 as i32, 0), (0, 1), (0, -1 as i32)]
        .iter()
        .enumerate()
    {
        let mut cur_row = (row as i32) + offsets.0;
        let mut cur_col = (col as i32) + offsets.1;
        while cur_row >= 0
            && cur_row < grid.len() as i32
            && cur_col >= 0
            && cur_col < grid[0].len() as i32
        {
            if loop_tiles.contains(&(cur_row as u32, cur_col as u32)) {
                tiles_in_directions[idx]
                    .1
                    .push(get_tile_at(grid, cur_row as u32, cur_col as u32));
            }
            cur_row = cur_row + offsets.0;
            cur_col = cur_col + offsets.1;
        }
    }
    tiles_in_directions
}

fn get_tile_at(grid: &[Vec<Tile>], row: u32, col: u32) -> Tile {
    grid[row as usize][col as usize]
}

fn get_neighbor_coords(grid: &[Vec<Tile>], row: u32, col: u32) -> Vec<(u32, u32)> {
    let tile = get_tile_at(grid, row, col);
    let north_neighbor = if row > 0 { Some((row - 1, col)) } else { None };
    let south_neighbor = if row + 1 < grid.len() as u32 {
        Some((row + 1, col))
    } else {
        None
    };
    let east_neighbor = if col + 1 < grid[0].len() as u32 {
        Some((row, col + 1))
    } else {
        None
    };
    let west_neighbor = if col > 0 { Some((row, col - 1)) } else { None };

    match tile {
        Tile::NorthSouth => vec![north_neighbor, south_neighbor],
        Tile::EastWest => vec![east_neighbor, west_neighbor],
        Tile::NorthEast => vec![north_neighbor, east_neighbor],
        Tile::NorthWest => vec![north_neighbor, west_neighbor],
        Tile::SouthEast => vec![south_neighbor, east_neighbor],
        Tile::SouthWest => vec![south_neighbor, west_neighbor],
        Tile::Ground => vec![],
        Tile::Start => vec![north_neighbor, south_neighbor, west_neighbor, east_neighbor],
    }
    .iter()
    .filter_map(|n| *n)
    .collect()
}

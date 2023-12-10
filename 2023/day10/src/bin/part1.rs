#[derive(PartialEq, Clone, Copy)]
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

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    // construct the map
    let tiles: Vec<Vec<Tile>> = input
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
    let loop_length = loop_tiles.len() - 1;
    let furthes_point_steps = loop_length / 2;
    println!("{furthes_point_steps}");
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

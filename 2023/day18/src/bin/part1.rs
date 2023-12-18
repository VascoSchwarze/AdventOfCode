use std::collections::HashSet;

#[derive(Hash, Clone, Copy, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_neighbor_offset(&self) -> (i32, i32) {
        use Direction::*;
        match &self {
            Up => (-1, 0),
            Down => (1, 0),
            Right => (0, 1),
            Left => (0, -1),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Direction::*;
        match value {
            'U' => Ok(Up),
            'D' => Ok(Down),
            'L' => Ok(Left),
            'R' => Ok(Right),
            _ => Err("Not a direction".to_string()),
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let instructions: Vec<(Direction, u32)> = input
        .lines()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<&str>>();
            let dir =
                Direction::try_from(parts[0].chars().next().unwrap()).expect("Parsing a direction");
            (dir, parts[1].parse().expect("Parsing a number"))
        })
        .collect();

    // calculate all painted edges given by the instructions
    let start_tile = (10000, 10000);
    let mut painted_tiles: HashSet<(u32, u32)> = HashSet::new();
    let mut cur_tile: (u32, u32) = start_tile;
    for instruction in instructions {
        for _ in 0..instruction.1 {
            let neighbor_offset = instruction.0.get_neighbor_offset();
            let next_tile = (
                (cur_tile.0 as i32 + neighbor_offset.0) as u32,
                (cur_tile.1 as i32 + neighbor_offset.1) as u32,
            );
            painted_tiles.insert(next_tile);
            cur_tile = next_tile;
        }
    }

    // print out the grid on the console
    // let max_row_idx = painted_tiles.iter().map(|v| v.0 .0).max().unwrap();
    // let min_row_idx = painted_tiles.iter().map(|v| v.0 .0).min().unwrap();
    // let max_col_idx = painted_tiles.iter().map(|v| v.0 .1).max().unwrap();
    // let min_col_idx = painted_tiles.iter().map(|v| v.0 .1).min().unwrap();
    // for row in min_row_idx..=max_row_idx {
    //     for col in min_col_idx..=max_col_idx {
    //         let c = if painted_tiles.contains_key(&(row, col)) {
    //             '#'
    //         } else {
    //             '.'
    //         };
    //         print!("{c}");
    //     }
    //     print!("\n");
    // }

    // flood fill
    let mut stack: Vec<(u32, u32)> = vec![];
    stack.push((start_tile.0 + 1, start_tile.1 + 1));
    while let Some(tile) = stack.pop() {
        if painted_tiles.insert(tile) {
            stack.push((tile.0 + 1, tile.1));
            stack.push((tile.0, tile.1 + 1));
            if tile.0 > 0 {
                stack.push((tile.0 - 1, tile.1));
            }
            if tile.1 > 0 {
                stack.push((tile.0, tile.1 - 1));
            }
        }
    }

    let volume = painted_tiles.len();
    println!("{volume}");
}

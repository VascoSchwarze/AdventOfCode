use itertools::Itertools;

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
            Up => (0, 1),
            Down => (0, -1),
            Right => (1, 0),
            Left => (-1, 0),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Direction::*;
        match value {
            '0' => Ok(Right),
            '1' => Ok(Down),
            '2' => Ok(Left),
            '3' => Ok(Up),
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
            let instruction = parts[2].replace(['(', ')', '#'], "");
            let number =
                u32::from_str_radix(&instruction[0..5], 16).expect("Parsing a valid hex digit");
            let dir = Direction::try_from(instruction.chars().nth(5).unwrap())
                .expect("Parsing a direction");
            (dir, number)
        })
        .collect();

    // calculate all vertices (corners) of the polygon
    let start_tile = (100_000_000, 100_000_000);
    let mut vertices: Vec<(u32, u32)> = vec![start_tile];
    let mut cur_tile: (u32, u32) = start_tile;
    for instruction in instructions {
        let neighbor_offset = instruction.0.get_neighbor_offset();
        let next_tile = (
            (cur_tile.0 as i32 + neighbor_offset.0 * instruction.1 as i32) as u32,
            (cur_tile.1 as i32 + neighbor_offset.1 * instruction.1 as i32) as u32,
        );
        vertices.push(next_tile);
        cur_tile = next_tile;
    }

    // calculate the corners of a polygon where the points from above lie between the grid points, depending on the kind of corner/direction
    // e.g. a tile (from above) with x = 3 lies now between x = 3 and x = 4
    // this polygon then includes all tiles completely or not at all, there is no confusion later whether a tile is inside or not

    // these need to be added at the end, so that the new_vertices ends with the same vertex as it starts with
    vertices.push(vertices[1]);
    vertices.push(vertices[2]);

    let mut new_vertices = vec![];
    for (p1, p2, p3) in vertices.iter().tuple_windows() {
        if p2.0 > p1.0 {
            if p3.1 > p2.1 {
                new_vertices.push((p2.0, p2.1 + 1));
            } else {
                new_vertices.push((p2.0 + 1, p2.1 + 1));
            }
        } else if p2.0 < p1.0 {
            if p3.1 > p2.1 {
                new_vertices.push((p2.0, p2.1));
            } else {
                new_vertices.push((p2.0 + 1, p2.1));
            }
        } else if p2.1 > p1.1 {
            if p3.0 > p2.0 {
                new_vertices.push((p2.0, p2.1 + 1));
            } else {
                new_vertices.push((p2.0, p2.1));
            }
        } else if p3.0 > p2.0 {
            new_vertices.push((p2.0 + 1, p2.1 + 1));
        } else {
            new_vertices.push((p2.0 + 1, p2.1));
        }
    }

    // use the shoelace formula to calculate the area
    let mut area: i128 = 0;
    for (p1, p2) in new_vertices.iter().tuple_windows() {
        area += (p2.0 as i128 - p1.0 as i128) * (p1.1 as i128 + p2.1 as i128) / 2;
    }

    println!("{area}");
}

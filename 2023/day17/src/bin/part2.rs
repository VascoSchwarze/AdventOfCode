use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

#[derive(Hash, Clone, Copy, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Node {
    coords: (u32, u32),
    direction: Direction,
    direction_streak: u32,
}

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let points: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Should be a digit"))
                .collect()
        })
        .collect();

    let start = Node {
        coords: (0, 0),
        direction: Direction::None,
        direction_streak: 1,
    };

    let mut open_nodes: PriorityQueue<Node, Reverse<u32>> = PriorityQueue::new();
    open_nodes.push(start, Reverse(0));

    let mut closed_nodes: HashSet<Node> = HashSet::new();

    let mut g: HashMap<Node, u32> = HashMap::new();
    g.insert(start, 0);

    let mut predecessors: HashMap<Node, Node> = HashMap::new();

    let target = (
        input.lines().enumerate().last().unwrap().0 as u32,
        (input.lines().last().unwrap().len() - 1) as u32,
    );

    let mut path_length = 0;
    let mut final_node: Option<Node> = None;
    while let Some((cur_node, _)) = open_nodes.pop() {
        if cur_node.coords == target && cur_node.direction_streak >= 4 {
            path_length = *g.get(&cur_node).expect("Should contain the current node");
            final_node = Some(cur_node);
            break;
        }
        closed_nodes.insert(cur_node);
        for neighbor in get_neighbors(&points, cur_node) {
            if closed_nodes.contains(&neighbor) {
                continue;
            }

            let new_g = g.get(&cur_node).expect("Should contain the current node")
                + points[neighbor.coords.0 as usize][neighbor.coords.1 as usize];
            if let Some(prev_g) = g.get(&neighbor) {
                if *prev_g <= new_g {
                    continue;
                }
            }
            predecessors.insert(neighbor, cur_node);
            g.insert(neighbor, new_g);
            let new_f = new_g + heuristic(neighbor.coords, target);
            open_nodes.push(neighbor, Reverse(new_f));
        }
    }

    // for reconstructing the path (not needed for the task)
    let mut path = vec![target];
    let mut cur = final_node.expect("A final node should exist");
    while let Some(predecessor) = predecessors.get(&cur) {
        cur = *predecessor;
        path.push(cur.coords);
    }
    path.reverse();
    // dbg!(&path);

    println!("{path_length}");
}

fn heuristic(point: (u32, u32), target: (u32, u32)) -> u32 {
    point.0.abs_diff(target.0) + point.1.abs_diff(target.1)
}

fn get_neighbors(points: &[Vec<u32>], node: Node) -> Vec<Node> {
    let mut neighbors: Vec<Node> = vec![];
    let (row, col) = node.coords;
    let does_up_neighbor_exist = row > 0 && node.direction != Direction::Down;
    let can_crucible_move_up = node.direction == Direction::Up
        || node.direction == Direction::None
        || node.direction_streak >= 4;
    if does_up_neighbor_exist && can_crucible_move_up {
        let direction_streak = if node.direction == Direction::Up {
            node.direction_streak + 1
        } else {
            1
        };
        if direction_streak <= 10 {
            neighbors.push(Node {
                coords: (row - 1, col),
                direction: Direction::Up,
                direction_streak,
            });
        }
    }
    let does_down_neighbor_exist = row + 1 < points.len() as u32 && node.direction != Direction::Up;
    let can_crucible_move_down = node.direction == Direction::Down
        || node.direction == Direction::None
        || node.direction_streak >= 4;
    if does_down_neighbor_exist && can_crucible_move_down {
        let direction_streak = if node.direction == Direction::Down {
            node.direction_streak + 1
        } else {
            1
        };
        if direction_streak <= 10 {
            neighbors.push(Node {
                coords: (row + 1, col),
                direction: Direction::Down,
                direction_streak,
            });
        }
    }
    let does_right_neighbor_exist =
        col + 1 < points[0].len() as u32 && node.direction != Direction::Left;
    let can_crucible_move_right = node.direction == Direction::Right
        || node.direction == Direction::None
        || node.direction_streak >= 4;
    if does_right_neighbor_exist && can_crucible_move_right {
        let direction_streak = if node.direction == Direction::Right {
            node.direction_streak + 1
        } else {
            1
        };
        if direction_streak <= 10 {
            neighbors.push(Node {
                coords: (row, col + 1),
                direction: Direction::Right,
                direction_streak,
            });
        }
    }
    let does_left_neighbor_exist = col > 0 && node.direction != Direction::Right;
    let can_crucible_move_left = node.direction == Direction::Left
        || node.direction == Direction::None
        || node.direction_streak >= 4;
    if does_left_neighbor_exist && can_crucible_move_left {
        let direction_streak = if node.direction == Direction::Left {
            node.direction_streak + 1
        } else {
            1
        };
        if direction_streak <= 10 {
            neighbors.push(Node {
                coords: (row, col - 1),
                direction: Direction::Left,
                direction_streak,
            });
        }
    }
    neighbors
}

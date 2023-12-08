use num::integer::lcm;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test2.txt");
    println!("{input}\n");
    let lines = input.lines().collect::<Vec<&str>>();

    let instructions = lines[0];
    let mut nodes: HashMap<&str, (String, String)> = HashMap::new();
    lines[2..].iter().for_each(|line| {
        let (node, targets) = line.split_once(" = ").unwrap();
        let targets = targets.replace(['(', ')'], "");
        let (left_target, right_target) = targets.split_once(", ").unwrap();
        nodes.insert(node, (left_target.to_owned(), right_target.to_owned()));
    });

    let nodes_to_check = nodes
        .keys()
        .cloned()
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<&str>>();
    let mut lengths: Vec<u64> = vec![];
    for node in nodes_to_check {
        let mut cur_node = node;
        let mut i = 0;
        let mut steps = 0;
        while !cur_node.ends_with('Z') {
            let (left_target, right_target) = nodes.get(cur_node).expect("Should be contained");
            if instructions.chars().nth(i).unwrap() == 'L' {
                cur_node = left_target;
            } else {
                cur_node = right_target;
            }
            i += 1;
            i %= instructions.len();
            steps += 1;
        }
        lengths.push(steps);
    }
    let result = lengths.iter().cloned().reduce(lcm).unwrap();
    println!("{result}");
}

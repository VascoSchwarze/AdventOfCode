use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
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

    let mut cur_node = "AAA";
    let mut i = 0;
    let mut steps = 0;
    while cur_node != "ZZZ" {
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
    println!("{steps}");
}

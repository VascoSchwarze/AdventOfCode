fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let result: u64 = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(springs, groups)| {
            count_valid_combinations(
                springs.chars().collect(),
                &groups
                    .split(',')
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            )
        })
        .sum();

    println!("{result}");
}

fn count_valid_combinations(springs: Vec<char>, groups: &[u32]) -> u64 {
    if springs.iter().all(|c| *c == '.' || *c == '#') {
        return if is_valid_combination(springs, groups) {
            1
        } else {
            0
        };
    }
    let unsure_idx = springs
        .iter()
        .position(|c| *c == '?')
        .expect("Should contain a ?");
    let mut new_springs1 = springs.clone();
    let mut new_springs2 = springs.clone();
    new_springs1[unsure_idx] = '.';
    new_springs2[unsure_idx] = '#';

    count_valid_combinations(new_springs1, groups) + count_valid_combinations(new_springs2, groups)
}

fn is_valid_combination(springs: Vec<char>, groups: &[u32]) -> bool {
    let actual_groups: Vec<u32> = String::from_iter(springs)
        .split('.')
        .filter_map(|s| if !s.is_empty() { Some(s.len() as u32) } else { None }).collect();
    actual_groups == groups
}

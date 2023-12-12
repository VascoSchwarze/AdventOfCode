fn main() {
    // let input = include_str!("../../input.txt");
    let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let result: u64 = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(springs, groups)| {
            // let mut new_springs = String::from(springs);
            // for _ in 0..4 {
            //     new_springs = new_springs + "?" + springs;
            // }
            // println!("{new_springs}");
            dbg!("Checking next row ");
            let r = count_possible_arrangements(
                // &new_springs.chars().collect::<Vec<char>>(),
                &springs.chars().collect::<Vec<char>>(),
                &groups
                    .split(',')
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            );
            dbg!(r);
            r
        })
        .sum();

    println!("{result}");
}

fn count_possible_arrangements(springs: &[char], groups: &[u32]) -> u64 {
    let min_needed_chars = groups.iter().sum::<u32>() as usize + groups.len() - 1;
    let first_group = groups[0] as usize;
    let mut arrangement_count = 0;
    for starting_idx in 0..=(springs.len() - min_needed_chars) {
        let can_fit_group_at_start_idx = springs[starting_idx..=(starting_idx + first_group - 1)]
            .iter()
            .all(|c| *c != '.');
            dbg!(can_fit_group_at_start_idx);
        let group_can_end_after = springs.len() <= starting_idx + first_group
            || springs[starting_idx + first_group] != '#';
            dbg!(group_can_end_after);
        if can_fit_group_at_start_idx && group_can_end_after {
            if groups.len() == 1 {
                arrangement_count += 1;
            } else {
                let reduced_springs = &springs[starting_idx + first_group + 1..];
                let reduced_groups = &groups[1..];
                dbg!("Entering recursion level");
                arrangement_count += count_possible_arrangements(reduced_springs, reduced_groups);
                dbg!("Leaving recursion level with value");
            }
        }
    }
    arrangement_count
}

use memoize::memoize;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let result: u64 = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(springs, groups)| {
            let mut new_springs = String::from(springs);
            for _ in 0..4 {
                new_springs = new_springs + "?" + springs;
            }
            let mut new_groups = String::from(groups);
            for _ in 0..4 {
                new_groups = new_groups + "," + groups;
            }
            count_possible_arrangements(
                new_springs.chars().collect::<Vec<char>>(),
                new_groups
                    .split(',')
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            )
        })
        .sum();

    println!("{result}");
}

#[memoize]
fn count_possible_arrangements(springs: Vec<char>, groups: Vec<u32>) -> u64 {
    let min_needed_chars = groups.iter().sum::<u32>() as usize + groups.len() - 1;
    let first_group = groups[0] as usize;
    let mut arrangement_count = 0;
    for starting_idx in 0..=(springs.len() as i32 - min_needed_chars as i32) {
        let starting_idx = starting_idx as usize;
        let can_fit_group_at_start_idx = springs[starting_idx..=(starting_idx + first_group - 1)]
            .iter()
            .all(|c| *c != '.');
        let group_can_end_after = springs.len() <= starting_idx + first_group
            || springs[starting_idx + first_group] != '#';
        if can_fit_group_at_start_idx && group_can_end_after {
            if groups.len() == 1 {
                let is_valid = springs.len() <= starting_idx + first_group + 1
                    || springs[starting_idx + first_group + 1..]
                        .iter()
                        .all(|c| *c != '#');

                if is_valid {
                    arrangement_count += 1;
                }
            } else {
                let reduced_springs = &springs[starting_idx + first_group + 1..];
                let reduced_groups = &groups[1..];

                let broken_springs_reduced = reduced_springs.iter().filter(|c| **c == '#').count();
                let total_group_count_reduced: u32 = reduced_groups.iter().sum();
                let can_cover_remaining_broken_springs =
                    (total_group_count_reduced as usize) >= broken_springs_reduced;
                //^ if this is false, the groups now left can not account for all confirmed broken springs that remain. This can not be a valid configuration
                if can_cover_remaining_broken_springs {
                    arrangement_count += count_possible_arrangements(
                        reduced_springs.to_vec(),
                        reduced_groups.to_vec(),
                    );
                }
            }
        }
        if springs[starting_idx] == '#' {
            // the group must start here or earlier, no need to continue searching
            break;
        }
    }
    arrangement_count
}

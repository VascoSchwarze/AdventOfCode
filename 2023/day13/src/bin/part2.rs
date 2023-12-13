use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let patterns: Vec<Vec<&str>> = input
        .lines()
        .group_by(|line| !line.is_empty())
        .into_iter()
        .filter_map(|(key, group)| {
            if key {
                return Some(group.collect_vec());
            }
            None
        })
        .collect_vec();

    let mut sum = 0;
    for pattern in patterns.iter() {
        if let Some(horizontal_line_idx) = get_reflection_idx(pattern) {
            sum += (horizontal_line_idx + 1) * 100;
            continue;
        }

        let mut columns: Vec<String> = vec![];
        for i in 0..pattern[0].len() {
            let column = String::from_iter(
                pattern
                    .iter()
                    .map(|row| row.chars().nth(i).unwrap())
                    .collect::<Vec<char>>(),
            );
            columns.push(column);
        }
        let columns: Vec<&str> = columns.iter().map(|c| c.as_str()).collect();

        if let Some(vertical_line_idx) = get_reflection_idx(&columns) {
            sum += vertical_line_idx + 1;
            continue;
        }
    }
    println!("{sum}");
}

fn get_reflection_idx(strings: &[&str]) -> Option<u32> {
    for idx in 0..(strings.len() - 1) {
        let mut is_reflection_line = true;
        let mut has_encountered_smudge = false;
        for i in (0..=idx).rev() {
            if idx + idx - i + 1 < strings.len() && strings[i] != strings[idx + idx - i + 1] {
                let smudge_count = strings[i]
                    .chars()
                    .enumerate()
                    .filter(|(char_idx, char)| {
                        *char != strings[idx + idx - i + 1].chars().nth(*char_idx).unwrap()
                    })
                    .count();
                if has_encountered_smudge || smudge_count > 1{
                    is_reflection_line = false;
                    break;
                } else {
                    has_encountered_smudge = true;
                }
            }
        }
        if is_reflection_line && has_encountered_smudge {
            return Some(idx as u32);
        }
    }
    None
}

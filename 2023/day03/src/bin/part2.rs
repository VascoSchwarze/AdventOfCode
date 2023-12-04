use std::collections::HashMap;

use regex::{Match, Regex};

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let mut schema: Vec<Vec<char>> = vec![];
    let mut numbers: Vec<(usize, Match)> = vec![];
    for (line_nr, line) in input.lines().enumerate() {
        schema.push(line.chars().collect());
        get_numbers_in_str(line).iter().for_each(|mat| {
            numbers.push((line_nr, mat.clone()));
        });
    }

    let mut gears: HashMap<(i32, i32), Vec<&str>> = HashMap::new();

    for (line_nr, mat) in numbers {
        let line_nr = i32::try_from(line_nr).expect("Should be an i32.");
        let cols = (
            i32::try_from(mat.start()).unwrap() - 1,
            i32::try_from(mat.end()).unwrap(),
        );

        for col in cols.0..=cols.1 {
            for line in (line_nr - 1)..=(line_nr + 1) {
                if is_char_at_pos_gear(&schema, line, col) {
                    gears
                        .entry((line, col))
                        .and_modify(|nums| nums.push(mat.as_str()))
                        .or_insert(vec![mat.as_str()]);
                }
            }
        }
    }

    let sum: i32 = gears
        .values()
        .filter_map(|nums| {
            if nums.len() != 2 {
                return None;
            }
            let n1 = nums[0].parse::<i32>().expect("Should be an i32");
            let n2 = nums[1].parse::<i32>().expect("Should be an i32");
            Some(n1 * n2)
        })
        .sum();

    println!("{sum}");
}

fn get_numbers_in_str(string: &str) -> Vec<Match> {
    let re = Regex::new(r"\d+").expect("Should be a valid regex");
    re.find_iter(string).collect()
}

fn is_char_at_pos_gear(schema: &Vec<Vec<char>>, line: i32, col: i32) -> bool {
    if line < 0
        || line > i32::try_from(schema.len()).unwrap() - 1
        || col < 0
        || col > i32::try_from(schema[0].len()).unwrap() - 1
    {
        return false;
    }
    let line: usize = line.try_into().unwrap();
    let col: usize = col.try_into().unwrap();

    schema[line][col] == '*'
}

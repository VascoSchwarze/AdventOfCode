use regex::{Match, Regex};

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let mut sum = 0;
    let mut schema: Vec<Vec<char>> = vec![];
    let mut numbers: Vec<(usize, Match)> = vec![];
    for (line_nr, line) in input.lines().enumerate() {
        schema.push(line.chars().collect());
        get_numbers_in_str(line).iter().for_each(|mat| {
            numbers.push((line_nr, mat.clone()));
        });
    }

    for (line_nr, mat) in numbers {
        let line_nr = i32::try_from(line_nr).expect("Should be an i32.");
        let cols = (
            i32::try_from(mat.start()).unwrap() - 1,
            i32::try_from(mat.end()).unwrap(),
        );
        let mut is_part = false;
        for col in cols.0..=cols.1 {
            for line in (line_nr - 1)..=(line_nr + 1) {
                if is_char_at_pos_symbol(&schema, line, col) {
                    let num = mat
                        .as_str()
                        .parse::<u32>()
                        .expect("Match should be a number.");
                    sum += num;
                    is_part = true;
                    break;
                }
            }
            if is_part {
                break;
            }
        }
        if is_part {
            let str = mat.as_str();
            println!("Adding {str}");
        }
    }

    println!("{sum}");
}

fn get_numbers_in_str(string: &str) -> Vec<Match> {
    let re = Regex::new(r"\d+").expect("Should be a valid regex");
    re.find_iter(string).collect()
}

fn is_char_at_pos_symbol(schema: &Vec<Vec<char>>, line: i32, col: i32) -> bool {
    if line < 0
        || line > i32::try_from(schema.len()).unwrap() - 1
        || col < 0
        || col > i32::try_from(schema[0].len()).unwrap() - 1
    {
        return false;
    }
    let line: usize = line.try_into().unwrap();
    let col: usize = col.try_into().unwrap();

    !(schema[line][col].is_digit(10) || schema[line][col] == '.')
}

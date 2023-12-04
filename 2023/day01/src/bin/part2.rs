use std::cmp::min;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let number_patterns: [(&str, char); 9] = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let mut sum = 0;
    input.lines().for_each(|line| {
        let chars: Vec<char> = line.chars().collect();
        let char_count = chars.len();
        let mut first_number_char: Option<char> = Option::None;
        for i in 0..char_count {
            let char = chars[i];
            match char.to_digit(10) {
                None => {}
                Some(_) => {
                    first_number_char = Option::Some(char);
                    break;
                }
            }
            let slice = &line[i..(i + min(char_count - i, 5))];
            for pattern in number_patterns {
                if slice.starts_with(pattern.0) {
                    first_number_char = Option::Some(pattern.1);
                    break;
                }
            }
            if first_number_char.is_some() {
                break;
            }
        }

        let mut second_number_char: Option<char> = Option::None;
        for i in (0..char_count).rev() {
            let char = chars[i];
            match char.to_digit(10) {
                None => {}
                Some(_) => {
                    second_number_char = Option::Some(char);
                    break;
                }
            }
            let slice = &line[i..(i + min(char_count - i, 5))];
            for pattern in number_patterns {
                if slice.starts_with(pattern.0) {
                    second_number_char = Option::Some(pattern.1);
                    break;
                }
            }
            if second_number_char.is_some() {
                break;
            }
        }

        let first_number_char = first_number_char.unwrap();
        let second_number_char = second_number_char.unwrap();

        let mut number = String::from(first_number_char);
        number.push(second_number_char);
        println!("{line}: {number}");
        let number: u32 = number.parse().expect("Not a number");
        sum += number;
    });

    println!("{sum}");
}

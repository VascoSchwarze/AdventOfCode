fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../input_test.txt");
    println!("{input}\n");

    let mut sum = 0;
    input.lines().for_each(|line| {
        let chars: Vec<char> = line.chars().collect();
        let char_count = chars.len();
        let mut first_number_char = chars[0];
        for i in 0..char_count {
            let char = chars[i];
            match char.to_digit(10) {
                None => continue,
                Some(_) => {
                    first_number_char = char;
                    break;
                }
            }
        }
        let mut second_number_char = chars[char_count - 1];
        for i in (0..char_count).rev() {
            let char = chars[i];
            match char.to_digit(10) {
                None => continue,
                Some(_) => {
                    second_number_char = char;
                    break;
                }
            }
        }

        let mut number = String::from(first_number_char);
        number.push(second_number_char);
        let number: u32 = number.parse().expect("Not a number");
        sum += number;
    });

    println!("{sum}");
}

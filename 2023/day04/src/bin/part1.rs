use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let mut sum = 0;
    input.lines().for_each(|line| {
        let whitespace_re = Regex::new(" +").unwrap();
        let line = whitespace_re.replace_all(line, " ");
        let winning_numbers: Vec<u32> = line
            .split(":")
            .nth(1)
            .unwrap()
            .split("|")
            .nth(0)
            .unwrap()
            .trim()
            .split(" ")
            .map(|str| str.parse::<u32>().expect("Should be a number"))
            .collect();

        let card_numbers: Vec<u32> = line
            .split(":")
            .nth(1)
            .unwrap()
            .split("|")
            .nth(1)
            .unwrap()
            .trim()
            .split(" ")
            .map(|str| str.parse::<u32>().expect("Should be a number"))
            .collect();

        let matching_count = card_numbers
            .iter()
            .filter(|card_number| winning_numbers.contains(&card_number))
            .count();
        if matching_count > 0 {
            sum += (2 as u32).pow(u32::try_from(matching_count).unwrap() - 1);
        }
    });

    println!("{sum}");
}

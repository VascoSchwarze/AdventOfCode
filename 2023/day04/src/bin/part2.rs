use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let mut card_counts: Vec<u32> = vec![1; input.lines().count()];
    for (line_nr, line) in input.lines().enumerate() {
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

        for i in (line_nr+1)..=(line_nr+matching_count) {
            card_counts[i] += card_counts[line_nr];
        }
    }

    let total_card_count: u32 = card_counts.iter().sum();
    println!("{total_card_count}");
}

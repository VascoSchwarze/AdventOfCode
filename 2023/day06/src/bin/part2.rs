use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let lines: Vec<&str> = input.lines().collect();
    let number_re = Regex::new(r"\d+").unwrap();
    let time = number_re
        .find_iter(lines[0])
        .fold(String::from(""), |acc, mat| acc + mat.as_str())
        .parse::<u64>()
        .expect("Shoul be a number");
    let distance = number_re
        .find_iter(lines[1])
        .fold(String::from(""), |acc, mat| acc + mat.as_str())
        .parse::<u64>()
        .expect("Shoul be a number");

    let mut num_ways = 0;
    for i in 1..(time / 2) {
        if i * (time - i) > distance {
            num_ways = time - (i - 1) - i;
            break;
        }
    }
    println!("{num_ways}");
}

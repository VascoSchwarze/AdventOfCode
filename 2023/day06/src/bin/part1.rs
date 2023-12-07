use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let lines: Vec<&str> = input.lines().collect();
    let number_re = Regex::new(r"\d+").unwrap();
    let times: Vec<u32> = number_re
        .find_iter(lines[0])
        .map(|mat| mat.as_str().parse::<u32>().expect("Should be a number"))
        .collect();
    let distances: Vec<u32> = number_re
        .find_iter(lines[1])
        .map(|mat| mat.as_str().parse::<u32>().expect("Should be a number"))
        .collect();

    let mut product = 1;
    for (idx, time) in times.iter().enumerate() {
        let mut num_ways = 0;
        let distance = distances[idx];
        for i in 1..(time / 2) {
            if i * (time - i) > distance {
                num_ways = time - (i - 1) - i;
                break;
            }
        }
        product *= num_ways;
    }
    println!("{product}");
}

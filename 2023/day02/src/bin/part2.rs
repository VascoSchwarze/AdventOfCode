use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let patterns: [&str; 3] = [r"(\d+) red", r"(\d+) green", r"(\d+) blue"];

    let mut sum = 0;
    input.lines().for_each(|line| {
        let line_parts: Vec<&str> = line.split(":").collect();
        let sets: Vec<&str> = line_parts[1].split(";").map(|set| set.trim()).collect();

        let mut power = 1;
        for pattern in patterns {
            let min_num = sets.iter().map(|set| {
                let re = Regex::new(pattern).unwrap();
                match re.captures(set) {
                    None => 0,
                    Some(capture) => {
                        let group = capture.extract::<1>().1[0];
                        group.parse::<i32>().expect("Should be a number")
                    }
                }
            }).max().unwrap();
            power *= min_num;
        }

        sum += power;
    });

    println!("{sum}");
}

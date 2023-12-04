use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let patterns: [(&str, i32); 3] = [
        (r"(\d+) red", 12),
        (r"(\d+) green", 13),
        (r"(\d+) blue", 14),
    ];

    let mut sum = 0;
    input.lines().for_each(|line| {
        let line_parts: Vec<&str> = line.split(":").collect();
        let game_id = line_parts[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .expect("Game ID should be a number");
        let sets: Vec<&str> = line_parts[1].split(";").map(|set| set.trim()).collect();

        let mut possible = true;
        for set in sets {
            for pattern in patterns {
                let re = Regex::new(pattern.0).unwrap();
                match re.captures(set) {
                    None => {}
                    Some(capture) => {
                        let group = capture.extract::<1>().1[0];
                        let count = group.parse::<i32>().expect("Should be a number");
                        if count > pattern.1 {
                            possible = false;
                        }
                    }
                }
            }
        }

        if possible {
            sum += game_id;
        }
    });

    println!("{sum}");
}

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let steps: Vec<&str> = input.lines().flat_map(|line| line.split(',')).collect();

    let mut sum: u64 = 0;
    for step in steps {
        sum += step.chars().fold(0, |acc, c| {
            let val = c as u64;
            ((acc + val) * 17) % 256
        })
    }
    println!("{sum}");
}

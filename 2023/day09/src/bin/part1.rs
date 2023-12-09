fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let sequences: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|nr| nr.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let mut sum = 0;
    for series in sequences {
        let mut diff_series: Vec<Vec<i64>> = vec![];
        while !diff_series
            .last()
            .is_some_and(|diffs| diffs.iter().all(|d| *d == 0))
        {
            let last_series = diff_series.last().unwrap_or(&series);
            let mut new_diff_series: Vec<i64> = vec![];
            for i in 0..(last_series.len() - 1) {
                new_diff_series.push(last_series[i + 1] - last_series[i]);
            }
            diff_series.push(new_diff_series);
        }
        diff_series.reverse();
        diff_series.push(series);
        let value = diff_series
            .iter()
            .fold(0, |acc, diffs| diffs.last().unwrap() + acc);

        sum += value;
    }
    println!("{sum}");
}

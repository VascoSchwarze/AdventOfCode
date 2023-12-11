use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let grid: Vec<Vec<char>> = input.lines().map(|c| c.chars().collect()).collect();

    //find empty rows and columns
    let mut row_idcs_to_insert = vec![];
    let mut col_idcs_to_insert = vec![];
    for (row_idx, row) in grid.iter().enumerate() {
        if row.iter().all(|c| *c == '.') {
            row_idcs_to_insert.push(row_idx);
        }
    }
    for col_idx in 0..grid[0].len() {
        if grid.iter().all(|row| row[col_idx] == '.') {
            col_idcs_to_insert.push(col_idx);
        }
    }

    //find galaxies (taking expanded cols and rows into account)
    let mut galaxies: Vec<(u64, u64)> = vec![];
    for (row_idx, row) in grid.iter().enumerate() {
        let add_rows = row_idcs_to_insert
            .iter()
            .filter(|idx| **idx < row_idx)
            .count();
        for (col_idx, char) in row.iter().enumerate() {
            if *char == '#' {
                let add_cols = col_idcs_to_insert
                    .iter()
                    .filter(|idx| **idx < col_idx)
                    .count();
                galaxies.push(((row_idx + add_rows * 999_999) as u64, (col_idx + add_cols * 999_999) as u64));
            }
        }
    }

    // calculate distances between galaxies
    let sum: u64 = galaxies
        .iter()
        .combinations(2)
        .map(|pair| {
            let galaxy1 = pair[0];
            let galaxy2 = pair[1];
            galaxy2.0.abs_diff(galaxy1.0) + galaxy2.1.abs_diff(galaxy1.1)
        })
        .sum();

    println!("{sum}");
}

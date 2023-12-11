use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let mut grid: Vec<Vec<char>> = input.lines().map(|c| c.chars().collect()).collect();

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

    //expand those rows and columns
    for (count, row_idx) in row_idcs_to_insert.iter().enumerate() {
        grid.insert(row_idx + count, vec!['.'; grid.len()])
    }
    for (count, col_idx) in col_idcs_to_insert.iter().enumerate() {
        for row in &mut grid {
            row.insert(col_idx+count, '.');
        }
    }

    //find galaxies
    let mut galaxies: Vec<(u32, u32)> = vec![];
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, char) in row.iter().enumerate() {
            if *char == '#' {
                galaxies.push((row_idx as u32, col_idx as u32));
            }
        }
    }

    // calculate distances between galaxies
    let sum: u32 = galaxies.iter().combinations(2).map(|pair| {
        let galaxy1 = pair[0];
        let galaxy2 = pair[1];
        galaxy2.0.abs_diff(galaxy1.0) + galaxy2.1.abs_diff(galaxy1.1)
    }).sum();

    println!("{sum}");
}

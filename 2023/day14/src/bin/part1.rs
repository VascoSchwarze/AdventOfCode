fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for row_idx in 0..grid.len() {
        for col_idx in 0..grid[row_idx].len() {
            if grid[row_idx][col_idx] == '.' {
                for i in (row_idx + 1)..grid.len() {
                    match grid[i][col_idx] {
                        'O' => {
                            grid[i][col_idx] = '.';
                            grid[row_idx][col_idx] = 'O';
                            break;
                        }
                        '#' => break,
                        _ => {}
                    }
                }
            }
        }
    }
    
    let mut sum: u32 = 0;
    for (row_idx, row) in grid.iter().rev().enumerate() {
        row.iter().for_each(|c| {
            if *c == 'O' {
                sum += row_idx as u32 + 1;
            }
        })
    }

    println!("{sum}");
}

use memoize::memoize;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    grid = spin_cycle(grid, 1_000_000_000);

    // for r in &grid {
    //     let s = String::from_iter(r);
    //     println!("{s}");
    // }

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

#[memoize]
fn spin_cycle(grid: Vec<Vec<char>>, amount: u32) -> Vec<Vec<char>> {
    if amount > 1 {
        let mut grid = grid;
        for _ in 0..10 {
            grid = spin_cycle(grid, amount / 10);
        }
        return grid;
    }

    let mut grid = grid.clone();

    tilt_north(&mut grid);
    tilt_west(&mut grid);
    tilt_south(&mut grid);
    tilt_east(&mut grid);
    grid
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
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
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    for row_idx in (1..grid.len()).rev() {
        for col_idx in 0..grid[row_idx].len() {
            if grid[row_idx][col_idx] == '.' {
                for i in (0..=(row_idx - 1)).rev() {
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
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
    for col_idx in (1..grid[0].len()).rev() {
        for row_idx in 0..grid.len() {
            if grid[row_idx][col_idx] == '.' {
                for i in (0..=(col_idx - 1)).rev() {
                    match grid[row_idx][i] {
                        'O' => {
                            grid[row_idx][i] = '.';
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
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    for col_idx in 0..grid[0].len() {
        for row_idx in 0..grid.len() {
            if grid[row_idx][col_idx] == '.' {
                for i in (col_idx + 1)..grid[0].len() {
                    match grid[row_idx][i] {
                        'O' => {
                            grid[row_idx][i] = '.';
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
}

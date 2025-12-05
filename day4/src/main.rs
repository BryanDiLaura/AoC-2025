use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn file_handle(path: &str) -> io::Result<BufReader<File>> {
    let file_path = Path::new(path);

    let file = File::open(file_path)?;

    Ok(BufReader::new(file))
}

pub fn count_accessible(grid: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    for (row, r) in grid.iter().enumerate() {
        for (col, v) in r.iter().enumerate() {
            if *v > 0 {
                let row = row as i32;
                let col = col as i32;
                let mut index_count = 0;
                for x in -1..=1 {
                    for y in -1..=1 {
                        if row + x < 0
                            || row + x >= grid.len() as i32
                            || col + y < 0
                            || col + y >= r.len() as i32
                        {
                            continue;
                        }

                        if x == 0 && y == 0 {
                            continue;
                        }

                        // we're at a valid one to check
                        if grid[(row + x) as usize][(col + y) as usize] >= 0 {
                            index_count += 1;
                        }
                    }
                }
                if index_count < 4 {
                    result.push((row as usize, col as usize));
                }
            }
        }
    }

    println!("COUNT: {}", result.len());

    result
}

fn main() -> io::Result<()> {
    // let reader = file_handle("assets/example.txt")?;
    let reader = file_handle("assets/input.txt")?;

    let mut grid: Vec<Vec<i32>> = reader
        .lines()
        .map(|lin| {
            lin.unwrap()
                .chars()
                .into_iter()
                .map(|c| if c == '.' { -1 } else { 8 })
                .collect()
        })
        .collect();

    //pt1
    // count_accessible(&mut grid);

    //pt2
    let mut total = 0;
    while let count = count_accessible(&grid) {
        if count.len() <= 0 {
            break;
        }
        total += count.len();
        for (x, y) in count {
            grid[x][y] = -1;
        }
    }

    println!("ANSWER: {total}");

    Ok(())
}

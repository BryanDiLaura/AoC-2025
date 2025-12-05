use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn file_handle(path: &str) -> io::Result<BufReader<File>> {
    let file_path = Path::new(path);

    let file = File::open(file_path)?;

    Ok(BufReader::new(file))
}

pub fn count_accessible(grid: &Vec<Vec<i32>>) -> u32 {
    let mut count = 0;

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
                    count += 1;
                }
            }
        }
    }

    for a in grid {
        println!("{:?}", a);
    }

    println!("COUNT: {count}");

    count
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

    count_accessible(&mut grid);
    //pt1
    // let total: u32 = banks.iter().map(|b| calculate_max_power_2(&b)).sum();

    //pt2
    // let total: u64 = banks.iter().map(|b| calculate_max_power(&b, 12)).sum();

    // println!("ANSWER: {total}");

    Ok(())
}

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn file_handle(path: &str) -> io::Result<BufReader<File>> {
    let file_path = Path::new(path);

    let file = File::open(file_path)?;

    Ok(BufReader::new(file))
}

pub fn get_first_max_index(arr: &[u32], offset: usize) -> usize {
    let mut max_index = 0;
    let mut max_val: u32 = 0;
    for (i, v) in arr.iter().enumerate() {
        if *v > max_val {
            max_index = i + offset;
            max_val = *v;
        }
    }
    max_index
}

pub fn calculate_max_power_2(bank: &[u32]) -> u32 {
    let first_index = get_first_max_index(&bank[..bank.len() - 1], 0);
    let second_index = get_first_max_index(&bank[first_index + 1..], first_index + 1);
    bank[first_index] * 10 + bank[second_index]
}

pub fn calculate_max_power(bank: &[u32], digits: usize) -> u64 {
    //idea is to split the available array into smaller sub-arrays and just calculate the max
    //of that subarray (maximizing the value for that digit).
    //
    //Need to keep track of "slack" to know how to size that subarray (based
    //on the required number of digits compared to the size of the bank)
    let mut indexes: Vec<usize> = vec![0; digits];

    let mut slack = bank.len() - digits;

    for i in 0..digits {
        let start = {
            match i {
                0 => 0,
                _ => indexes[i - 1] + 1,
            }
        };

        let end = start + slack;

        indexes[i] = get_first_max_index(&bank[start..=end], start);

        slack -= indexes[i] - start;
    }

    let mut sum: u64 = 0;
    for (i, index) in indexes.iter().enumerate() {
        sum += bank[*index] as u64 * 10_u64.pow((digits - 1 - i) as u32);
    }
    sum
}

fn main() -> io::Result<()> {
    // let reader = file_handle("assets/example.txt")?;
    let reader = file_handle("assets/input.txt")?;

    let banks: Vec<Vec<u32>> = reader
        .lines()
        .map(|lin| {
            lin.unwrap()
                .chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    //pt1
    // let total: u32 = banks.iter().map(|b| calculate_max_power_2(&b)).sum();

    //pt2
    let total: u64 = banks.iter().map(|b| calculate_max_power(&b, 12)).sum();

    println!("ANSWER: {total}");

    Ok(())
}

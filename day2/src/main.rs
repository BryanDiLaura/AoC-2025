use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn file_handle(path: &str) -> io::Result<BufReader<File>> {
    let file_path = Path::new(path);

    let file = File::open(file_path)?;

    Ok(BufReader::new(file))
}

pub fn check_simple_repeat(s: &str) -> bool {
    if s.len() % 2 == 0 {
        let (left, right) = s.split_at(s.len() / 2);
        if left == right {
            return true;
        }
    }
    false
}

pub fn check_any_repeat(s: &str) -> bool {
    // I ended up looking up an elegant solution. This is similar to:
    // https://leetcode.com/problems/repeated-substring-pattern

    let repeated = s.to_owned() + s;
    let repeated = repeated[1..repeated.len() - 1].to_string();
    repeated.find(s).is_some()
}

fn main() -> io::Result<()> {
    let reader = file_handle("assets/input.txt")?;

    let ranges = reader.lines().next().unwrap()?;

    let ranges: Vec<Vec<i64>> = ranges
        .split(",")
        .into_iter()
        .map(|r| r.split('-').map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut ret = Vec::new();
    for r in ranges {
        // println!("{:?}", r);
        for num in r[0]..=r[1] {
            let num_str = num.to_string();
            //pt1
            // if check_simple_repeat(&num_str) {
            //pt2
            if check_any_repeat(&num_str) {
                ret.push(num);
            }
        }
    }

    println!("ANSWER: {}", ret.iter().sum::<i64>());

    Ok(())
}

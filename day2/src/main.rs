use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn file_handle(path: &str) -> io::Result<BufReader<File>> {
    let file_path = Path::new(path);

    let file = File::open(file_path)?;

    Ok(BufReader::new(file))
}
fn main() -> io::Result<()> {
    let reader = file_handle("assets/example.txt")?;

    let ranges = reader.lines().next().unwrap()?;

    // let ranges: Vec<Vec<i64>> = ranges
    //     .split(",")
    //     .into_iter()
    //     .map(|r| r.split('-').map(|n| n.parse().unwrap()).collect())
    //     .collect();
    let ranges: Vec<Vec<&str>> = ranges.split(",").map(|r| r.split('-').collect()).collect();

    for r in ranges {
        println!("{:?}", r)
    }

    Ok(())
}

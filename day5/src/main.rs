use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn file_handle(path: &str) -> io::Result<BufReader<File>> {
    let file_path = Path::new(path);

    let file = File::open(file_path)?;

    Ok(BufReader::new(file))
}

pub fn combine_ranges(ranges: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut ret: Vec<Vec<u64>> = Vec::new();

    let mut cur = 0;
    for (i, r) in ranges.iter().enumerate() {
        if cur != 0 && r[0] <= ret[cur - 1][1] {
            if r[1] > ret[cur - 1][1] {
                ret[cur - 1][1] = r[1];
            }
        } else {
            ret.push(r.clone());
            cur += 1;
        }
    }
    ret
}

pub fn check_fresh_items(ranges: &Vec<Vec<u64>>, items: &Vec<u64>) -> u32 {
    let mut count = 0;
    for i in items {
        //this likely could be more efficient...
        for r in ranges {
            if *i >= r[0] && *i <= r[1] {
                count += 1;
                break;
            }
        }
    }
    count
}

pub fn count_fresh_ids(ranges: &Vec<Vec<u64>>) -> u64 {
    let mut count = 0;
    for r in ranges {
        count += r[1] - r[0] + 1;
    }
    count
}

fn main() -> io::Result<()> {
    // let reader = file_handle("assets/example2.txt")?;
    let reader = file_handle("assets/input.txt")?;

    let database: Vec<String> = reader.lines().map(|lin| lin.unwrap().to_owned()).collect();

    let pivot = database.iter().position(|s| s.is_empty()).unwrap();

    let mut ranges: Vec<Vec<u64>> = database[..pivot]
        .iter()
        .map(|s| {
            s.split("-")
                .into_iter()
                .map(|v| v.parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    let mut items: Vec<u64> = database[pivot + 1..]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Ranges:");
    for r in &ranges {
        println!("{:?}", r);
    }
    ranges.sort_by(|a, b| a[0].cmp(&b[0]));
    let combined = combine_ranges(&mut ranges);

    println!("Combined:");
    for r in &combined {
        println!("{:?}", r);
    }

    items.sort();

    // println!("Items:");
    // for i in &items {
    //     println!("{i}");
    // }

    let total = check_fresh_items(&combined, &items);

    println!("Fresh: {total}");

    let count = count_fresh_ids(&combined);

    println!("Count: {count}");

    Ok(())
}

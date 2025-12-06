use core::num;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn file_handle(path: &str) -> io::Result<BufReader<File>> {
    let file_path = Path::new(path);

    let file = File::open(file_path)?;

    Ok(BufReader::new(file))
}

//this could certainly be faster if you change row/col major...
pub fn calculate(problems: &Vec<Vec<String>>) -> u64 {
    let operation_row = problems.len() - 1;
    let num_problems = problems[0].len();

    let mut total = 0;
    for i in 0..num_problems {
        total += match problems[operation_row][i].as_str() {
            "*" => {
                let mut inner: u64 = 1;
                for num in 0..operation_row {
                    inner *= problems[num][i].parse::<u64>().unwrap();
                }
                inner
            }
            "+" => {
                let mut inner: u64 = 0;
                for num in 0..operation_row {
                    inner += problems[num][i].parse::<u64>().unwrap();
                }
                inner
            }
            _ => panic!(
                "invalid operation '{}'",
                problems[operation_row][i].as_str()
            ),
        }
    }

    total
}

enum Operation {
    Add,
    Multiply,
}

//I realized after the fact that I did this in a dumb way. I really should just reverse iterate
//making numbers along the way, and use anchors to do the operation and clear the numbers
//vector. Ah well.
pub fn calculate_annoying(problems: &Vec<String>) -> u64 {
    let operation_row_idx = problems.len() - 1;
    let op_row = &problems[operation_row_idx];

    let mut total = 0;

    let mut index = 0;
    while index < op_row.len() {
        let operation = match op_row.chars().nth(index) {
            Some('*') => Operation::Multiply,
            Some('+') => Operation::Add,
            _ => panic!("invalid operation '{}'", op_row.chars().nth(index).unwrap()),
        };

        // Idea is to use the operation as an "anchor" and then make numbers from this anchor
        // until the next anchor we detect

        let mut numbers = Vec::new();
        loop {
            let mut num = "".to_owned();
            for row in 0..operation_row_idx {
                if problems[row].chars().nth(index).unwrap() != ' ' {
                    num.push(problems[row].chars().nth(index).unwrap());
                }
            }
            if num.len() > 0 {
                numbers.push(num.parse::<u64>().unwrap());
            }
            index += 1;
            if index >= op_row.len() || op_row.chars().nth(index).unwrap() != ' ' {
                break;
            }
        }

        // println!("Got: {:?}", numbers);
        total += match operation {
            Operation::Multiply => numbers.iter().product::<u64>(),
            Operation::Add => numbers.iter().sum(),
        };
    }
    total
}

fn main() -> io::Result<()> {
    //pt1
    // // let reader = file_handle("assets/example.txt")?;
    // let reader = file_handle("assets/input.txt")?;

    // let problems: Vec<Vec<String>> = reader
    //     .lines()
    //     .map(|lin| {
    //         lin.unwrap()
    //             .split_whitespace()
    //             .map(|s| s.to_string())
    //             .collect()
    //     })
    //     .collect();

    // let answer = calculate(&problems);
    // println!("ANSWER: {answer}");

    //pt2
    // let reader = file_handle("assets/example.txt")?;
    let reader = file_handle("assets/input.txt")?;
    let problems: Vec<String> = reader.lines().map(|lin| lin.unwrap()).collect();

    let answer = calculate_annoying(&problems);
    println!("ANSWER: {answer}");

    Ok(())
}

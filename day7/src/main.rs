use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn file_handle(path: &str) -> io::Result<BufReader<File>> {
    let file_path = Path::new(path);

    let file = File::open(file_path)?;

    Ok(BufReader::new(file))
}

fn main() -> io::Result<()> {
    // let reader = file_handle("assets/example.txt")?;
    let reader = file_handle("assets/input.txt")?;
    let mut manifold: Vec<String> = reader.lines().map(|lin| lin.unwrap()).collect();

    // for m in &manifold {
    //     println!("{m}");
    // }

    //start the propagation
    let start_idx = manifold[0]
        .chars()
        .into_iter()
        .position(|c| c == 'S')
        .unwrap();
    manifold[1].replace_range(start_idx..start_idx + 1, "|");

    let mut answer = 0;

    for i in 1..manifold.len() - 1 {
        // println!("LOOKING: {:?}", manifold[i]);
        let idxs: Vec<usize> = manifold[i].match_indices("|").map(|(idx, _)| idx).collect();
        let next_row = &mut manifold[i + 1];
        for idx in idxs {
            match next_row.chars().nth(idx) {
                Some('^') => {
                    // println!("SPLIT");
                    answer += 1;
                    next_row.replace_range(idx - 1..idx, "|");
                    next_row.replace_range(idx + 1..idx + 2, "|");
                }
                _ => next_row.replace_range(idx..idx + 1, "|"),
            }
        }
    }

    println!("ANSWER: {answer}");
    // for m in &manifold {
    //     println!("{m}");
    // }

    Ok(())
}

use std::fs::File;
use std::io::{self,BufRead,BufReader};
use std::path::Path;

pub fn file_handle(path: &str) -> io::Result<BufReader<File>>{
    let file_path = Path::new(path);

    let file = File::open(file_path)?;

    Ok(BufReader::new(file))
}


pub fn pt1() -> io::Result<()>{
    let reader = file_handle("assets/input.txt")?;

    let mut dial = 50;

    let mut zero_count = 0;

    for line_res in reader.lines() {
        let line = line_res?;

        let mut num: i32 = line[1..].parse().expect("Couldn't parse value");
        num = match line.chars().nth(0){
            Some('L') => num * -1,
            Some('R') => num * 1,
            _ => panic!("invalid")
        };

        dial += num;
        while dial > 99 || dial < 0{
            match dial {
                n if n>99 => dial -= 100,
                n if n<0 => dial += 100,
                _ => (),
            }
        }

        if dial == 0{
            zero_count += 1;
        }

        println!("{line} -> {dial}");
    }

    //Answer: 1086
    println!("ZERO COUNT: {zero_count}");

    Ok(())
}

pub fn pt2() -> io::Result<()>{
    let reader = file_handle("assets/input.txt")?;

    let mut dial = 50;

    let mut zero_count = 0;

    for line_res in reader.lines() {
        let line = line_res?;

        let mut mag: i32 = line[1..].parse().expect("Couldn't parse value");
        let num = match line.chars().nth(0){
            Some('L') => -1,
            Some('R') => 1,
            _ => panic!("invalid")
        };

        //unga bunga
        while mag != 0{
            dial += num;
            if dial == -1{
                dial = 99;
            }
            if dial == 100{
                dial = 0;
            }
            if dial == 0{
                zero_count += 1;
            }
            mag -= 1;
        }

        println!("{line} -> {dial}");
    }

    println!("ZERO COUNT: {zero_count}");

    Ok(())
}

fn main() -> io::Result<()>{

    // pt1()
    pt2()
}

use std::{
    env,
    error::{self},
    fs::File,
    io::{BufRead, BufReader},
};

use solution::solution::{solve, solve2};

mod solution;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("<FILE_PATH> argument missing");
        return Ok(());
    }

    let file_path = args[1].clone();

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];

    for line in reader.lines() {
        if line.is_ok() {
            let l = line.unwrap();
            let numbers: Vec<&str> = l.split("   ").collect();
            if numbers.len() == 2 {
                let n1: u32 = numbers[0].parse()?;
                let n2: u32 = numbers[1].parse()?;
                list1.push(n1);
                list2.push(n2);
            }
        }
    }

    let sum_distances = solve(&list1, &list2);

    if sum_distances.is_ok() {
        println!("sum_distances = {}", sum_distances.unwrap());
    } else {
        return Err(sum_distances.unwrap_err().into());
    }

    let sim_score = solve2(&list1, &list2);
    println!("sim_score = {}", sim_score);
    Ok(())
}

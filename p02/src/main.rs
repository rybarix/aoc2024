use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use solution::solution::{is_safe, is_safe_double_check};

mod solution;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("<FILE_PATH> argument missing");
        return;
    }

    let file_path = args[1].clone();

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut number_of_safe_lists = 0;
    let mut number_of_safe_lists_damperner = 0;

    for line in reader.lines() {
        if line.is_ok() {
            let l = line.unwrap();
            let numbers_str: Vec<&str> = l.split(" ").collect();
            let iter = numbers_str.iter();
            let numbers_it: Vec<u32> = iter.map(|x| x.parse().unwrap_or_default()).collect();

            let safety = is_safe(&numbers_it);
            number_of_safe_lists += safety as u32;

            if safety == false {
                number_of_safe_lists_damperner += is_safe_double_check(&numbers_it) as u32;
            }
        }
    }

    println!("number_of_safe_lists={}", number_of_safe_lists);
    println!(
        "number_of_safe_lists_thumper={}",
        number_of_safe_lists_damperner
    );
    println!(
        "total={}",
        number_of_safe_lists_damperner + number_of_safe_lists
    );
}

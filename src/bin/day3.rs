use std::{env::args, fs};

use regex::Regex;

fn main() {
    let data = fs::read_to_string(args().nth(1).unwrap_or("input/day3.txt".into())).unwrap();

    let mul_regex = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum_of_products: u32 = 0;
    let mut is_enabled = true;
    for capture in mul_regex.captures_iter(&data) {
        match &capture[0] {
            "do()" => {
                is_enabled = true;
            }
            "don't()" => {
                is_enabled = false;
            }
            _ if is_enabled => {
                let a: u32 = capture[1].parse().expect("integer");
                let b: u32 = capture[2].parse().expect("integer");
                sum_of_products += a * b;
            }
            _ => {}
        }
    }

    println!("Sum or products: {sum_of_products}");
}

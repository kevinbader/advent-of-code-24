use std::{env::args, fs};

use regex::Regex;

fn main() {
    let data = fs::read_to_string(args().nth(1).unwrap_or("input/day3.txt".into())).unwrap();

    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let sum_of_products: u32 = mul_regex
        .captures_iter(&data)
        .map(|c| {
            let a: u32 = c[1].parse().expect("integer");
            let b: u32 = c[2].parse().expect("integer");
            a * b
        })
        .sum();

    println!("Sum or products: {sum_of_products}");
}

use std::{env::args, fs};

use itertools::Itertools;

fn main() {
    let data = fs::read_to_string(args().nth(1).unwrap()).unwrap();

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in data.lines() {
        let [a, b] = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("number"))
            .collect::<Vec<_>>()[..]
        else {
            panic!("expected exactly two numbers per row");
        };
        list1.push(a);
        list2.push(b);
    }
    assert_eq!(list1.len(), list2.len());

    list1.sort();
    list2.sort();

    let diff: usize = list1.iter().zip(&list2).map(|(a, b)| a.abs_diff(*b)).sum();

    println!("total distance between both lists: {diff}");

    // PART 2

    let list2_counts = list2.iter().counts();

    let similarity_score: usize = list1
        .iter()
        .map(|n| n * list2_counts.get(&n).unwrap_or(&0))
        .sum();

    println!("similarity score: {similarity_score}");
}

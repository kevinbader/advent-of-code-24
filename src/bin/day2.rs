use std::{env::args, fs};

fn main() {
    let data = fs::read_to_string(args().nth(1).unwrap_or("input/day2.txt".into())).unwrap();

    let mut n_safe_reports = 0;
    for line in data.lines() {
        let levels = line
            .split_whitespace()
            .map(|col| col.parse::<i32>().expect("level is number"))
            .collect::<Vec<_>>();

        if is_report_safe(&levels) {
            n_safe_reports += 1;
        } else {
            // Is it safe using the Problem Dampener?
            for i in 0..levels.len() {
                let mut levels = levels.clone();
                levels.remove(i);
                if is_report_safe(&levels) {
                    n_safe_reports += 1;
                    break;
                }
            }
        }
    }

    println!("Number of safe reports: {n_safe_reports}");
}

/// A report only counts as safe if both of the following are true:
/// 1. The levels are either all increasing or all decreasing.
/// 2. Any two adjacent levels differ by at least one and at most three.
fn is_report_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let diffs: Vec<i32> = levels.windows(2).map(|w| w[1] - w[0]).collect();

    let all_increasing_within_range = diffs.iter().all(|&d| d > 0 && d <= 3);
    let all_decreasing_within_range = diffs.iter().all(|&d| d < 0 && d >= -3);

    all_increasing_within_range || all_decreasing_within_range
}

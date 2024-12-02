use std::{env::args, fs};

fn main() {
    let data = fs::read_to_string(args().nth(1).unwrap_or("input/day2.txt".into())).unwrap();

    let reports: Vec<Report> = data
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|col| col.parse::<i32>().expect("level is number"))
                .map(Level)
                .collect::<Vec<_>>()
        })
        .map(Report)
        .collect();

    let n_safe_reports = reports
        .iter()
        .filter(|report| safety_system::is_safe(report))
        .count();

    println!("Number of safe reports: {n_safe_reports}");
}

#[derive(Debug, Clone, Copy)]
struct Level(i32);

#[derive(Debug)]
struct Report(Vec<Level>);

mod safety_system {
    pub fn is_safe(report: &super::Report) -> bool {
        // A report only counts as safe if both of the following are true:
        // 1. The levels are either all increasing or all decreasing.
        // 2. Any two adjacent levels differ by at least one and at most three.
        #[derive(Debug, Clone, Copy)]
        enum Direction {
            Increasing,
            Decreasing,
        }

        let mut last_level = None;
        let mut direction = None;
        for level in &report.0 {
            // Calculate diff and update last_level to current level.
            let diff = match last_level {
                Some(&super::Level(last_value)) => {
                    let diff = level.0 - last_value;
                    last_level = Some(level);
                    diff
                }
                None => {
                    // If this is the first level, we can't determine safety yet.
                    last_level = Some(level);
                    continue;
                }
            };

            // If distance is not in [1, 3], we know it's _unsafe_.
            if !(1..=3).contains(&diff.abs()) {
                return false;
            }

            // If we don't know the direction yet, we accept it and continue.
            let Some(direction) = direction else {
                direction = Some(if diff > 0 {
                    Direction::Increasing
                } else {
                    Direction::Decreasing
                });
                continue;
            };

            // If the level does not match expected direction, it's _unsafe_.
            if matches!(direction, Direction::Increasing) && diff < 0 {
                return false;
            }
            if matches!(direction, Direction::Decreasing) && diff > 0 {
                return false;
            }
        }

        // We passed all checks, so this report is _safe_.
        true
    }
}

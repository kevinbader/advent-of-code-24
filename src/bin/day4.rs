use core::fmt;
use std::{env::args, fs};

fn main() {
    let data = fs::read_to_string(args().nth(1).unwrap_or("input/day4.txt".into())).unwrap();

    let space = Space::new(data.lines().map(|l| l.chars().collect()).collect());
    let count = space
        .regions_3x3()
        .filter_map(|mut region| {
            if matches_x_mas_pattern(&mut region) {
                Some(region)
            } else {
                None
            }
        })
        .count();

    println!("X-MAS count: {count}");
}

struct Space {
    rows: Vec<Vec<char>>,
    n_rows: usize,
    n_cols: usize,
}

impl Space {
    fn new(rows: Vec<Vec<char>>) -> Self {
        let n_rows = rows.len();
        assert!(n_rows > 0);
        let n_cols = rows[0].len();
        assert!(rows.iter().all(|cols| cols.len() == n_cols));
        Self {
            rows,
            n_rows,
            n_cols,
        }
    }
    fn regions_3x3(&self) -> Region3x3Iter {
        assert!(self.n_rows >= 3);
        assert!(self.n_cols >= 3);
        Region3x3Iter {
            space: self,
            x: 1,
            y: 1,
        }
    }
}

struct Region3x3Iter<'a> {
    space: &'a Space,
    x: usize,
    y: usize,
}

impl<'a> Iterator for Region3x3Iter<'a> {
    type Item = Region3x3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= (self.space.n_rows - 1) {
            return None;
        }

        let Region3x3Iter { space, x, y } = *self;
        let region = Region3x3::new(
            &space.rows[y - 1][(x - 1)..=(x + 1)],
            &space.rows[y][(x - 1)..=(x + 1)],
            &space.rows[y + 1][(x - 1)..=(x + 1)],
        );

        // Advance cursor..
        if x < (self.space.n_cols - 2) {
            // ..to the right.
            self.x += 1;
        } else {
            // ..downwards.
            self.x = 1;
            self.y += 1;
        }

        Some(region)
    }
}

struct Region3x3 {
    chars: [char; 9],
}

impl Region3x3 {
    fn new(top_row: &[char], mid_row: &[char], bottom_row: &[char]) -> Self {
        let mut chars = [' '; 9];
        chars[0..3].clone_from_slice(top_row);
        chars[3..6].clone_from_slice(mid_row);
        chars[6..9].clone_from_slice(bottom_row);
        Self { chars }
    }

    fn get(&self, x: usize, y: usize) -> char {
        assert!(x < 3);
        assert!(y < 3);
        self.chars[y * 3 + x]
    }

    fn rotate_left(&mut self) {
        // We rotate left by first transposing, then reversing the columns.
        let chars = &mut self.chars;

        // Transpose:
        for row in 0..2 {
            for col in (row + 1)..3 {
                chars.swap(row * 3 + col, col * 3 + row);
            }
        }

        // Reverse columns:
        for col in 0..3 {
            #[allow(clippy::identity_op)]
            #[allow(clippy::erasing_op)]
            chars.swap(0 * 3 + col, 2 * 3 + col);
        }
    }
}

impl fmt::Display for Region3x3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = self.chars;
        write!(
            f,
            "{}\n{}\n{}\n",
            &chars[0..3].iter().collect::<String>(),
            &chars[3..6].iter().collect::<String>(),
            &chars[6..9].iter().collect::<String>(),
        )
    }
}

fn matches_x_mas_pattern(region: &mut Region3x3) -> bool {
    let mid = region.get(1, 1);
    if mid != 'A' {
        return false;
    }

    // We check and rotate 4 times.
    for _ in 0..4 {
        if let ('M', 'M', 'S', 'S') = (
            region.get(0, 0),
            region.get(2, 0),
            region.get(0, 2),
            region.get(2, 2),
        ) {
            return true;
        };
        region.rotate_left();
    }

    false
}

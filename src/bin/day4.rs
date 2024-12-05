use std::{env::args, fs};

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
}

fn main() {
    let data = fs::read_to_string(args().nth(1).unwrap_or("input/day4.txt".into())).unwrap();

    let space = Space::new(data.lines().map(|l| l.chars().collect()).collect());

    let mut count = 0;
    for x in 0..space.n_rows {
        for y in 0..space.n_cols {
            count += number_of_xmas_at_x(&space, x, y);
        }
    }

    println!("XMAS count: {count}");
}

fn number_of_xmas_at_x(
    Space {
        rows,
        n_rows,
        n_cols,
    }: &Space,
    x: usize,
    y: usize,
) -> usize {
    if rows[x][y] != 'X' {
        return 0;
    }
    let mut count = 0;
    // horizontal, forward
    if x + 3 < *n_cols {
        if rows[x + 1][y] == 'M' && rows[x + 2][y] == 'A' && rows[x + 3][y] == 'S' {
            count += 1;
        }
    }
    // horizontal, backward
    if x >= 3 {
        if rows[x - 1][y] == 'M' && rows[x - 2][y] == 'A' && rows[x - 3][y] == 'S' {
            count += 1;
        }
    }
    // vertical, downward
    if y + 3 < *n_rows {
        if rows[x][y + 1] == 'M' && rows[x][y + 2] == 'A' && rows[x][y + 3] == 'S' {
            count += 1;
        }
    }
    // vertical, upward
    if y >= 3 {
        if rows[x][y - 1] == 'M' && rows[x][y - 2] == 'A' && rows[x][y - 3] == 'S' {
            count += 1;
        }
    }
    // diagonal, up-right
    if y >= 3 && x + 3 < *n_cols {
        if rows[x + 1][y - 1] == 'M' && rows[x + 2][y - 2] == 'A' && rows[x + 3][y - 3] == 'S' {
            count += 1;
        }
    }
    // diagonal, up-left
    if y >= 3 && x >= 3 {
        if rows[x - 1][y - 1] == 'M' && rows[x - 2][y - 2] == 'A' && rows[x - 3][y - 3] == 'S' {
            count += 1;
        }
    }
    // diagonal, down-left
    if y + 3 < *n_rows && x >= 3 {
        if rows[x - 1][y + 1] == 'M' && rows[x - 2][y + 2] == 'A' && rows[x - 3][y + 3] == 'S' {
            count += 1;
        }
    }
    // diagonal, down-right
    if y + 3 < *n_rows && x + 3 < *n_cols {
        if rows[x + 1][y + 1] == 'M' && rows[x + 2][y + 2] == 'A' && rows[x + 3][y + 3] == 'S' {
            count += 1;
        }
    }
    count
}

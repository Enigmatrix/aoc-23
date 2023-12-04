#![cfg(test)]

use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, Error},
};

#[test]
fn part1() -> Result<(), Error> {
    let inp = crate::input(3);

    let grid: Vec<Vec<_>> = inp
        .lines()
        .flat_map(|line| line.ok().map(|line| line.chars().collect()))
        .collect();

    let mut num = None;
    let mut has_symbol = false;
    let mut sum = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let c = grid[row][col];
            if c.is_digit(10) {
                num = Some(num.unwrap_or(0) * 10 + c.to_digit(10).unwrap());
                if !has_symbol {
                    for dx in [1i32, 0, -1] {
                        for dy in [1i32, 0, -1] {
                            if dx == 0 && dy == 0 {
                                continue;
                            }

                            let nrow = row as i32 + dx;
                            let ncol = col as i32 + dy;
                            if nrow < 0 || ncol < 0 {
                                continue;
                            }

                            if grid
                                .get(nrow as usize)
                                .map(|row| row.get(ncol as usize))
                                .is_some_and(|c| c.is_some_and(|c| *c != '.' && !c.is_digit(10)))
                            {
                                has_symbol = true;
                            }
                        }
                    }
                }
            } else {
                if let Some(val) = num {
                    if has_symbol {
                        sum += val;
                    }
                }
                num = None;
                has_symbol = false;
            }
        }
        if let Some(val) = num {
            if has_symbol {
                sum += val;
            }
        }
        num = None;
        has_symbol = false;
    }

    println!("part1: {sum}");

    Ok(())
}

#[test]
fn part2() -> Result<(), Error> {
    let inp = crate::input(3);

    let grid: Vec<Vec<_>> = inp
        .lines()
        .flat_map(|line| line.ok().map(|line| line.chars().collect()))
        .collect();

    let mut num = None;
    let mut fidx = 0;
    let mut sum = 0;
    let mut gears: HashMap<(i32, i32), Vec<u32>> = HashMap::new();

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let c = grid[row][col];
            if c.is_digit(10) {
                if num.is_none() {
                    fidx = col;
                }
                num = Some(num.unwrap_or(0) * 10 + c.to_digit(10).unwrap());
            } else {
                let mut seen = HashSet::new();
                if let Some(val) = num {
                    for itercol in fidx..col {
                        for dx in [1i32, 0, -1] {
                            for dy in [1i32, 0, -1] {
                                if dx == 0 && dy == 0 {
                                    continue;
                                }

                                let nrow = row as i32 + dx;
                                let ncol = itercol as i32 + dy;
                                if nrow < 0 || ncol < 0 {
                                    continue;
                                }

                                if grid
                                    .get(nrow as usize)
                                    .map(|row| row.get(ncol as usize))
                                    .is_some_and(|c| c.is_some_and(|c| *c == '*'))
                                {
                                    if seen.contains(&(nrow, ncol)) {
                                        continue;
                                    }
                                    gears
                                        .entry((nrow, ncol))
                                        .or_insert_with(|| Vec::new())
                                        .push(val);
                                    seen.insert((nrow, ncol));
                                }
                            }
                        }
                    }
                }
                fidx = 0;
                num = None;
            }
        }
        let mut seen = HashSet::new();
        if let Some(val) = num {
            for itercol in fidx..grid[row].len() {
                for dx in [1i32, 0, -1] {
                    for dy in [1i32, 0, -1] {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nrow = row as i32 + dx;
                        let ncol = itercol as i32 + dy;
                        if nrow < 0 || ncol < 0 {
                            continue;
                        }

                        if grid
                            .get(nrow as usize)
                            .map(|row| row.get(ncol as usize))
                            .is_some_and(|c| c.is_some_and(|c| *c == '*'))
                        {
                            if seen.contains(&(nrow, ncol)) {
                                continue;
                            }
                            gears
                                .entry((nrow, ncol))
                                .or_insert_with(|| Vec::new())
                                .push(val);
                            seen.insert((nrow, ncol));
                        }
                    }
                }
            }
        }
        fidx = 0;
        num = None;
    }

    for (_, ratios) in gears {
        if ratios.len() != 2 {
            continue;
        }
        sum += ratios[0] * ratios[1];
    }

    println!("part2: {sum}");

    Ok(())
}

#![cfg(test)]

use std::{
    collections::HashSet,
    io::{repeat, Error},
    str::FromStr,
};

#[test]
fn part1() -> Result<(), Error> {
    let inp = include_str!("../inputs/11.txt");
    let mut grid: Vec<Vec<_>> = inp.lines().map(|s| s.chars().collect()).collect();
    let mut row_inserts = Vec::new();
    for row in 0..grid.len() {
        if grid[row].iter().all(|&c| c == '.') {
            row_inserts.push(row)
        }
    }
    let mut col_inserts = Vec::new();
    for col in 0..grid[0].len() {
        let mut all = true;
        for row in 0..grid.len() {
            if grid[row][col] != '.' {
                all = false;
                break;
            }
        }
        if all {
            col_inserts.push(col);
        }
    }

    let mut inserted = 0;
    for insert in row_inserts {
        grid.insert(insert + inserted, grid[insert + inserted].clone());
        inserted += 1;
    }
    inserted = 0;
    for insert in col_inserts {
        for row in 0..grid.len() {
            grid[row].insert(insert + inserted, '.')
        }
        inserted += 1;
    }

    let mut galaxies = Vec::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                galaxies.push((row, col));
            }
        }
    }

    let mut ans = 0;

    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            ans += galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
        }
    }

    println!("part1: {ans}");
    Ok(())
}

#[test]
fn part2() -> Result<(), Error> {
    let inp = include_str!("../inputs/11.txt");
    let mut grid: Vec<Vec<_>> = inp.lines().map(|s| s.chars().collect()).collect();
    let mut row_inserts = HashSet::new();
    for row in 0..grid.len() {
        if grid[row].iter().all(|&c| c == '.') {
            row_inserts.insert(row);
        }
    }
    let mut col_inserts = HashSet::new();
    for col in 0..grid[0].len() {
        let mut all = true;
        for row in 0..grid.len() {
            if grid[row][col] != '.' {
                all = false;
                break;
            }
        }
        if all {
            col_inserts.insert(col);
        }
    }

    let mut galaxies = Vec::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                galaxies.push((row, col));
            }
        }
    }

    let growth = 1000000;

    let mut ans = 0;

    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let (irow, icol) = galaxies[i];
            let (jrow, jcol) = galaxies[j];
            let mut rcost = 0i64;
            let mut ccost = 0i64;
            for row in irow.min(jrow)..irow.max(jrow) {
                if row_inserts.contains(&row) {
                    rcost += growth;
                } else {
                    rcost += 1;
                }
            }

            for col in icol.min(jcol)..icol.max(jcol) {
                if col_inserts.contains(&col) {
                    ccost += growth;
                } else {
                    ccost += 1;
                }
            }

            ans += rcost + ccost;
        }
    }

    println!("part2: {ans}");
    Ok(())
}

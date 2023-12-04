#![cfg(test)]

use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, Error},
};

fn parse_nums(s: &str) -> impl Iterator<Item = i32> + '_ {
    s.split_whitespace().map(|s| s.parse().unwrap())
}

#[test]
fn part1() -> Result<(), Error> {
    let inp = crate::input(4);
    let sum: u32 = inp
        .lines()
        .filter_map(|line| {
            line.ok().and_then(|line| {
                let mut game = line.split(":");
                let _ = game.next();

                let mut nums = game.next().unwrap().split(" | ");
                let winning: HashSet<_> = parse_nums(nums.next().unwrap()).collect();
                let actual = parse_nums(nums.next().unwrap());
                let count = actual.filter(|v| winning.contains(v)).count();
                Some(if count == 0 { 0 } else { 1 << (count - 1) })
            })
        })
        .sum();
    println!("{sum}");
    Ok(())
}

#[test]
fn part2() -> Result<(), Error> {
    let inp = crate::input(4);
    // (game-1, count)
    let counts: HashMap<_, _> = inp
        .lines()
        .filter_map(|line| {
            line.ok().and_then(|line| {
                //line.ok().and_then(|line| {
                let mut game = line.split(":");
                let _ = game.next();

                let mut nums = game.next().unwrap().split(" | ");
                let winning: HashSet<_> = parse_nums(nums.next().unwrap()).collect();
                let actual = parse_nums(nums.next().unwrap());
                let count = actual.filter(|v| winning.contains(v)).count();
                Some(count)
            })
        })
        .enumerate()
        .collect();
    let mut idx = 0;
    let mut sum = 0;

    let mut scratches: HashMap<_, _> = (0..counts.len()).map(|_| 1).enumerate().collect();

    loop {
        if let Some(v) = scratches.get(&idx).cloned() {
            sum += v;
            println!("{idx} = count({}), v({v})", counts[&idx]);
            for nidx in idx + 1..=idx + counts[&idx] {
                if let Some(r) = scratches.get_mut(&nidx) {
                    *r += v;
                }
            }
            idx += 1;
        } else {
            break;
        }
    }

    println!("{sum}");
    Ok(())
}

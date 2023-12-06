#![cfg(test)]

use std::io::Error;
use std::str::FromStr;

#[test]
fn part1() -> Result<(), Error> {
    let inp = include_str!("../inputs/06.txt");
    let mut lines = inp.lines();
    let times = lines.next().expect("time line").split_once(": ").expect("split :").1.split_whitespace().map(|s| s.parse().expect("parse num"));
    let dists = lines.next().expect("time line").split_once(": ").expect("split :").1.split_whitespace().map(|s| s.parse().expect("parse num"));
    let ways = times.zip(dists).map(|(time, dist): (u32, u32)| {
        let mut ways = 0;
        for t in 1..time {
            if (time - t) * t > dist {
                ways += 1
            }
        }
        ways
    }).fold(1, |acc, elem| acc * elem);

    println!("part1: {ways}");
    Ok(())
}

#[test]
fn part2() -> Result<(), Error> {
    let inp = include_str!("../inputs/06.txt");
    let mut lines = inp.lines();
    let time: u64 = lines.next().expect("time line").split_once(":").expect("line").1.replace(" ", "").parse().expect("parse num");
    let dist: u64 = lines.next().expect("time line").split_once(":").expect("line").1.replace(" ", "").parse().expect("parse num");
    let mut ways = 0;
    for t in 1..time {
        if (time - t) * t > dist {
            ways += 1
        }
    }

    println!("part2: {ways}");
    Ok(())
}

#![cfg(test)]

use std::{io::Error, str::FromStr};

struct History {
    inner: Vec<Vec<i64>>,
}

impl FromStr for History {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s
            .split_whitespace()
            .map(|s| s.parse().expect("num"))
            .collect();
        let mut inner = Vec::new();
        inner.push(line);
        Ok(History { inner })
    }
}

impl History {
    fn create_diffs(&mut self) {
        loop {
            let v = self.inner.last().expect("last");
            let mut r = Vec::new();
            let mut all_zero = true;
            for i in 1..v.len() {
                let b = v[i] - v[i - 1];
                r.push(b);
                if b != 0 {
                    all_zero = false;
                }
            }
            self.inner.push(r);
            if all_zero {
                break;
            }
        }
    }

    pub fn extrapolate_forwards(&mut self) -> i64 {
        for i in (0..self.inner.len() - 1).rev() {
            let diff = self.inner[i + 1].last().unwrap().clone();
            let prev = self.inner[i].last().unwrap().clone();
            self.inner[i].push(diff + prev);
        }
        self.inner[0].last().unwrap().clone()
    }

    pub fn extrapolate_backwards(&mut self) -> i64 {
        for i in (0..self.inner.len() - 1).rev() {
            let diff = self.inner[i + 1].first().unwrap().clone();
            let prev = self.inner[i].first().unwrap().clone();
            self.inner[i].insert(0, prev - diff);
        }
        self.inner[0].first().unwrap().clone()
    }
}

#[test]
fn part1() -> Result<(), Error> {
    let inp = include_str!("../inputs/09.txt");
    let ans: i64 = inp
        .lines()
        .map(|s| s.parse::<History>().expect("parse history line"))
        .map(|mut h| {
            h.create_diffs();
            h.extrapolate_forwards()
        })
        .sum();
    println!("part1: {ans}");
    Ok(())
}

#[test]
fn part2() -> Result<(), Error> {
    let inp = include_str!("../inputs/09.txt");
    let ans: i64 = inp
        .lines()
        .map(|s| s.parse::<History>().expect("parse history line"))
        .map(|mut h| {
            h.create_diffs();
            h.extrapolate_backwards()
        })
        .sum();
    println!("part2: {ans}");
    Ok(())
}

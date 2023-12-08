#![cfg(test)]

use std::{collections::HashMap, io::Error, iter::repeat};

#[test]
fn part1() -> Result<(), Error> {
    let inp = include_str!("../inputs/08.txt");
    let mut lines = inp.lines();
    let dirs = lines.next().expect("directions");
    let mapping: HashMap<_, _> = lines
        .skip(1)
        .map(|s| s.split_once(" = ").expect("equal line"))
        .map(|(src, dests)| {
            let split = dests.replace("(", "").replace(")", "");
            let (dest1, dest2) = split.split_once(", ").expect("two dests");
            (src, (dest1.to_owned(), dest2.to_owned()))
        })
        .collect();

    let mut cur = "AAA";
    let mut steps = 0;
    for dir in dirs.chars().cycle() {
        if dir == 'L' {
            cur = mapping[cur].0.as_ref()
        } else if dir == 'R' {
            cur = mapping[cur].1.as_ref()
        } else {
            panic!("what")
        }
        steps += 1;
        if cur == "ZZZ" {
            break;
        }
    }

    println!("part1: {steps}");
    Ok(())
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

#[test]
fn part2() -> Result<(), Error> {
    let inp = include_str!("../inputs/08.txt");
    let mut lines = inp.lines();
    let dirs = lines.next().expect("directions");
    let mapping: HashMap<_, _> = lines
        .skip(1)
        .map(|s| s.split_once(" = ").expect("equal line"))
        .map(|(src, dests)| {
            let split = dests.replace("(", "").replace(")", "");
            let (dest1, dest2) = split.split_once(", ").expect("two dests");
            (src, (dest1.to_owned(), dest2.to_owned()))
        })
        .collect();

    let starts: Vec<_> = mapping
        .keys()
        .filter(|s| s.ends_with('A'))
        .cloned()
        .collect();
    let mut curs = starts.clone();
    let mut repeat: Vec<_> = repeat(Vec::new()).take(starts.len()).collect();
    let mut steps = 0;
    for dir in dirs.chars().cycle().take(100000) {
        steps += 1;
        for (idx, cur) in &mut curs.iter_mut().enumerate() {
            if dir == 'L' {
                *cur = mapping[*cur].0.as_ref();
            } else {
                *cur = mapping[*cur].1.as_ref();
            }
            if cur.ends_with('Z') {
                repeat[idx].push(steps);
            }
        }
    }
    let r = repeat.into_iter().map(|v| v[1] - v[0]).collect::<Vec<_>>();
    let steps = lcm(&r);
    println!("part2: {steps}");
    Ok(())
}

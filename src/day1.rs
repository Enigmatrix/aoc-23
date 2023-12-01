#![cfg(test)]

use std::io::{BufRead, Error};

#[test]
fn part1() -> Result<(), Error> {
    let inp = crate::input(1);
    let sum: u32 = inp
        .lines()
        .filter_map(|line| {
            line.ok().and_then(|line| {
                let numbers: Vec<_> = line
                    .chars()
                    .filter(|c| c.is_numeric())
                    .flat_map(|c| c.to_digit(10))
                    .collect();
                let first = numbers.first();
                let second = numbers.last();
                let num = first.zip(second).map(|(first, second)| first * 10 + second);
                num
            })
        })
        .sum();
    println!("{sum}");
    Ok(())
}

fn string_to_number(s: &str) -> Option<usize> {
    let word_digits = [
        "_", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let digit_matches: Vec<_> = digits
        .iter()
        .enumerate()
        .chain(word_digits.iter().enumerate())
        .map(|(idx, sub)| {
            let matches: Vec<_> = s.match_indices(sub).collect();
            let first = matches.first().map(|(m, _)| (idx, *m));
            let last = matches.last().map(|(m, _)| (idx, *m));
            (first, last)
        })
        .collect();
    let first = digit_matches
        .iter()
        .flat_map(|(first, _)| first)
        .min_by_key(|(_, idx)| *idx)
        .map(|(v, _)| v);
    let last = digit_matches
        .iter()
        .flat_map(|(_, last)| last)
        .max_by_key(|(_, idx)| *idx)
        .map(|(v, _)| v);
    let num = first.zip(last).map(|(first, second)| first * 10 + second);
    num
}

#[test]
fn part2() -> Result<(), Error> {
    let inp = crate::input(1);
    let sum: usize = inp
        .lines()
        .filter_map(|line| line.ok().and_then(|line| string_to_number(&line)))
        .sum();
    println!("{sum}");
    Ok(())
}

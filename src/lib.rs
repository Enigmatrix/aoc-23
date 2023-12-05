use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub fn input(day: u32) -> impl Seek + BufRead {
    let file = File::open(format!("inputs/{day:02}.txt")).expect("input not found");
    BufReader::new(file)
}

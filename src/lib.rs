#![feature(iter_next_chunk)]
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn input(day: u32) -> impl Seek + BufRead {
    let file = File::open(format!("inputs/{day:02}.txt")).expect("input not found");
    BufReader::new(file)
}

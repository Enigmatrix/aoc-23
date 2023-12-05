#![cfg(test)]

use std::io::Error;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Range {
    start: u64,
    len: u64,
}

#[derive(Clone)]
struct RangeMap {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

impl FromStr for RangeMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s
            .split_whitespace()
            .map(|s| s.parse().expect("always a number"));
        let dst_start = nums.next().expect("dst_start");
        let src_start = nums.next().expect("src_start");
        let len = nums.next().expect("len");
        Ok(RangeMap {
            dst_start,
            src_start,
            len,
        })
    }
}

struct RangeChunks {
    start: Option<Range>,
    end: Option<Range>,
    matched: Option<Range>,
}

impl RangeMap {
    pub fn chunk(&self, range: &Range) -> Option<RangeChunks> {
        let mut chunk = RangeChunks {
            start: None,
            end: None,
            matched: None,
        };
        if range.start >= self.src_start + self.len || range.start + range.len <= self.src_start {
            return None;
        }

        if range.start < self.src_start {
            chunk.start = Some(Range {
                start: range.start,
                len: self.src_start - range.start,
            });
        }
        if range.start + range.len > self.src_start + self.len {
            chunk.end = Some(Range {
                start: self.src_start + self.len,
                len: range.start + range.len - self.src_start - self.len,
            });
        }

        let start = self.src_start.max(range.start);
        chunk.matched = Some(Range {
            start: self.dst_start + start - self.src_start,
            len: (self.src_start + self.len).min(range.start + range.len) - start,
        });

        Some(chunk)
    }
}

struct RangeMaps {
    ranges: Vec<RangeMap>,
}

impl RangeMaps {
    pub fn find(&self, key: u64) -> u64 {
        let range = self
            .ranges
            .iter()
            .find(|range| range.src_start <= key && key < range.src_start + range.len)
            .cloned()
            .unwrap_or_else(|| RangeMap {
                dst_start: key,
                src_start: key,
                len: 0,
            });
        range.dst_start + (key - range.src_start)
    }

    pub fn chunk_all(&self, ranges: &Vec<Range>) -> Vec<Range> {
        let mut ret = Vec::new();
        for range in ranges {
            ret.append(&mut self.chunk(range));
        }
        ret
    }

    pub fn chunk(&self, range: &Range) -> Vec<Range> {
        let mut ranges = Vec::new();
        ranges.push(range.clone());
        let mut out_ranges = Vec::new();

        for range_map in &self.ranges {
            let mut nranges = Vec::new();
            for range in ranges {
                if let Some(chunks) = range_map.chunk(&range) {
                    let RangeChunks {
                        start,
                        end,
                        matched,
                    } = chunks;
                    if let Some(matched) = matched {
                        out_ranges.push(matched);
                    }
                    if let Some(start) = start {
                        nranges.push(start);
                    }
                    if let Some(end) = end {
                        nranges.push(end);
                    }
                } else {
                    nranges.push(range);
                }
            }
            ranges = nranges
        }

        out_ranges.append(&mut ranges);

        out_ranges
    }
}

impl FromStr for RangeMaps {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges: Vec<RangeMap> = s
            .lines()
            .skip(1)
            .map(|s| s.parse().expect("range line parse"))
            .collect();
        Ok(RangeMaps { ranges })
    }
}

#[test]
fn part1() -> Result<(), Error> {
    let inp = include_str!("../inputs/05.txt");
    let mut parts = inp.split("\n\n");
    let seeds: Vec<u64> = parts
        .next()
        .expect("seeds line")
        .split_once(':')
        .expect("seeds")
        .1
        .split_whitespace()
        .map(|s| s.parse().expect("seed number"))
        .collect();

    let range_maps: Vec<RangeMaps> = parts
        .map(|s| s.parse().expect("ranges map parse"))
        .collect();

    let min_loc = seeds
        .into_iter()
        .map(|seed| {
            let mut cur = seed;
            for range_map in range_maps.iter() {
                cur = range_map.find(cur);
            }
            cur
        })
        .min()
        .expect("min_loc");

    println!("part1: {min_loc}");
    Ok(())
}

#[test]
fn part2() -> Result<(), Error> {
    let inp = include_str!("../inputs/05.txt");

    let mut parts = inp.split("\n\n");

    let seeds: Vec<u64> = parts
        .next()
        .expect("seeds line")
        .split_once(':')
        .expect("seeds")
        .1
        .split_whitespace()
        .map(|s| s.parse().expect("seed number"))
        .collect();

    let range_maps: Vec<RangeMaps> = parts
        .map(|s| s.parse().expect("ranges map parse"))
        .collect();

    let mut ranges: Vec<_> = seeds
        .chunks(2)
        .into_iter()
        .map(|seed| {
            let start = seed[0];
            let len = seed[1];
            Range { start, len }
        })
        .collect();

    for range_map in range_maps {
        ranges = range_map.chunk_all(&ranges);
    }

    let min_loc = ranges.iter().map(|s| s.start).min().expect("min_loc");

    println!("part2: {min_loc}");
    Ok(())
}

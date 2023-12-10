#![cfg(test)]
use colored::Colorize;

use core::panic;
use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

struct Grid {
    inner: Vec<Vec<char>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Grid {
    pub fn get(&self, pos: (isize, isize)) -> Option<char> {
        let (row, col) = pos;
        if row < 0
            || col < 0
            || row as usize >= self.inner.len()
            || col as usize >= self.inner[0].len()
        {
            return None;
        }
        return Some(self.inner[row as usize][col as usize]);
    }

    fn nextfordir(pos: (isize, isize), dir: &Dir) -> (isize, isize) {
        let (row, col) = pos;
        match dir {
            Dir::North => (row - 1, col),
            Dir::South => (row + 1, col),
            Dir::East => (row, col + 1),
            Dir::West => (row, col - 1),
        }
    }
    pub fn nextinner(
        &self,
        mut innerdir: Dir,
        inners: &mut HashSet<(isize, isize)>,
        innerdirs: &mut HashMap<(isize, isize), Dir>,
        mut pos: (isize, isize),
        mut dir: Dir,
        mut count: usize,
        path: &mut Vec<(isize, isize)>,
    ) -> Option<Vec<(isize, isize)>> {
        loop {
            // validate position
            if let Some(c) = self.get(pos) {
                if c == 'S' {
                    return Some(path.clone());
                }
                // validate direction first
                match dir.clone() {
                    Dir::North if c != '|' && c != '7' && c != 'F' => return None,
                    Dir::South if c != '|' && c != 'J' && c != 'L' => return None,
                    Dir::East if c != '-' && c != 'J' && c != '7' => return None,
                    Dir::West if c != '-' && c != 'L' && c != 'F' => return None,
                    _ => (),
                }
                let innerpos = Self::nextfordir(pos, &innerdir.clone());
                if !path.contains(&innerpos) {
                    inners.insert(innerpos);
                }

                inners.remove(&pos);
                path.push(pos);
                innerdirs.insert(pos, innerdir.clone());
                let (ndir, ninnerdir) = if c == '|' || c == '-' {
                    (dir, innerdir)
                } else if c == 'L' && dir == Dir::South {
                    (
                        Dir::East,
                        if innerdir == Dir::East {
                            Dir::North
                        } else if innerdir == Dir::West {
                            Dir::South
                        } else {
                            panic!()
                        },
                    )
                } else if c == 'L' && dir == Dir::West {
                    (
                        Dir::North,
                        if innerdir == Dir::North {
                            Dir::East
                        } else if innerdir == Dir::South {
                            Dir::West
                        } else {
                            panic!()
                        },
                    )
                } else if c == '7' && dir == Dir::North {
                    (
                        Dir::West,
                        if innerdir == Dir::East {
                            Dir::North
                        } else if innerdir == Dir::West {
                            Dir::South
                        } else {
                            panic!()
                        },
                    )
                } else if c == '7' && dir == Dir::East {
                    (
                        Dir::South,
                        if innerdir == Dir::North {
                            Dir::East
                        } else if innerdir == Dir::South {
                            Dir::West
                        } else {
                            panic!()
                        },
                    )
                } else if c == 'J' && dir == Dir::South {
                    (
                        Dir::West,
                        if innerdir == Dir::East {
                            Dir::South
                        } else if innerdir == Dir::West {
                            Dir::North
                        } else {
                            panic!()
                        },
                    )
                } else if c == 'J' && dir == Dir::East {
                    (
                        Dir::North,
                        if innerdir == Dir::South {
                            Dir::East
                        } else if innerdir == Dir::North {
                            Dir::West
                        } else {
                            panic!()
                        },
                    )
                } else if c == 'F' && dir == Dir::North {
                    (
                        Dir::East,
                        if innerdir == Dir::West {
                            Dir::North
                        } else if innerdir == Dir::East {
                            Dir::South
                        } else {
                            panic!()
                        },
                    )
                } else if c == 'F' && dir == Dir::West {
                    (
                        Dir::South,
                        if innerdir == Dir::North {
                            Dir::West
                        } else if innerdir == Dir::South {
                            Dir::East
                        } else {
                            panic!();
                        },
                    )
                } else {
                    unreachable!()
                };

                let innerpos = Self::nextfordir(pos, &ninnerdir.clone());
                if !path.contains(&innerpos) {
                    inners.insert(innerpos);
                }

                inners.remove(&pos);

                pos = Self::nextfordir(pos, &ndir);
                dir = ndir;
                count += 1;
                innerdir = ninnerdir;
            } else {
                return None;
            }
        }
    }

    pub fn next(
        &self,
        mut pos: (isize, isize),
        mut dir: Dir,
        mut count: usize,
        path: &mut Vec<(isize, isize)>,
    ) -> Option<Vec<(isize, isize)>> {
        loop {
            if path.contains(&pos) {
                return None;
            }
            // validate position
            if let Some(c) = self.get(pos) {
                if c == 'S' {
                    return Some(path.clone());
                }
                // validate direction first
                match dir.clone() {
                    Dir::North if c != '|' && c != '7' && c != 'F' => return None,
                    Dir::South if c != '|' && c != 'J' && c != 'L' => return None,
                    Dir::East if c != '-' && c != 'J' && c != '7' => return None,
                    Dir::West if c != '-' && c != 'L' && c != 'F' => return None,
                    _ => (),
                }
                path.push(pos);
                let ndir = if c == '|' || c == '-' {
                    dir
                } else if c == 'L' && dir == Dir::South {
                    Dir::East
                } else if c == 'L' && dir == Dir::West {
                    Dir::North
                } else if c == '7' && dir == Dir::North {
                    Dir::West
                } else if c == '7' && dir == Dir::East {
                    Dir::South
                } else if c == 'J' && dir == Dir::South {
                    Dir::West
                } else if c == 'J' && dir == Dir::East {
                    Dir::North
                } else if c == 'F' && dir == Dir::North {
                    Dir::East
                } else if c == 'F' && dir == Dir::West {
                    Dir::South
                } else {
                    unreachable!()
                };

                pos = Self::nextfordir(pos, &ndir);
                dir = ndir;
                count += 1;
            } else {
                return None;
            }
        }
    }
}

#[test]
fn part1() -> Result<(), Error> {
    let inp = include_str!("../inputs/10.txt");
    let grid: Vec<Vec<_>> = inp.lines().map(|s| s.chars().collect()).collect();
    let mut start = (0, 0);
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'S' {
                start = (row as isize, col as isize);
                break;
            }
        }
    }
    let grid = Grid { inner: grid };
    for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
        let mut path = Vec::new();
        if let Some(path) = grid.next(Grid::nextfordir(start, &dir), dir, 1, &mut path) {
    println!("part1: {}", path.len() / 2 + 1);
            break;
        }
    }

    Ok(())
}

#[test]
fn part2() -> Result<(), Error> {
    let inp = include_str!("../inputs/10.txt");
    let grid: Vec<Vec<_>> = inp.lines().map(|s| s.chars().collect()).collect();
    let mut start = (0, 0);
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'S' {
                start = (row as isize, col as isize);
                break;
            }
        }
    }
    let grid = Grid { inner: grid };
    let dir = Dir::South; // We know South works, and that West is inside the loop (east is outside)
    let mut inners = HashSet::new();

    let mut opath = Vec::new();
    grid.next(Grid::nextfordir(start, &dir), dir.clone(), 1, &mut opath);
    opath.push(start);

    let mut innerdirs = HashMap::new();

    if let Some(mut path) = grid.nextinner(
        Dir::West,
        &mut inners,
        &mut innerdirs,
        Grid::nextfordir(start, &dir),
        dir,
        1,
        &mut opath,
    ) {
        path.push(start);
        let pathpart: HashSet<_> = path.iter().collect();
        inners.remove(&start);
        let mut q: Vec<_> = inners.iter().cloned().collect();

        while let Some(v) = q.pop() {
            for dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
                let npos = Grid::nextfordir(v, &dir);
                if grid.get(npos).is_some() && !pathpart.contains(&npos) {
                    if !inners.contains(&npos) {
                        inners.insert(npos);
                        q.push(npos);
                    }
                }
            }
        }

        innerdirs.insert(start, Dir::West);

        for row in 0..grid.inner.len() {
            for col in 0..grid.inner[0].len() {
                if inners.contains(&(row as isize, col as isize)) {
                    print!("I");
                } else if pathpart.contains(&(row as isize, col as isize)) {
                    print!("{}", grid.inner[row][col].to_string().green());
                } else {
                    print!("{}", grid.inner[row][col].to_string().blue());
                }
            }
            println!();
        }
    }

    println!("part2: {}", inners.len());
    Ok(())
}

#![cfg(test)]

use std::error::Error;
use std::io::BufRead;
use std::str::FromStr;

#[derive(PartialEq, Eq)]
enum Color {
    Red,
    Blue,
    Green,
}

struct Draw {
    count: u32,
    color: Color,
}

struct Game {
    id: u32,
    draws: Vec<Vec<Draw>>,
}

impl FromStr for Draw {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let part: Vec<_> = s.split_whitespace().collect();
        let count: u32 = part[0].parse()?;
        let color = match part[1] {
            "blue" => Color::Blue,
            "red" => Color::Red,
            "green" => Color::Green,
            _ => return Err("invalid color".into()),
        };
        Ok(Draw { count, color })
    }
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game_parts = s.split(":");
        let game_desc = game_parts.next().ok_or("no game desc")?;
        let id: u32 = game_desc
            .split_whitespace()
            .next_back()
            .ok_or("no game id")?
            .parse()?;

        let draws = game_parts.next().ok_or("no draws")?;
        let draws: Result<Vec<Vec<_>>, _> = draws
            .split(";")
            .map(|draws| draws.split(", ").map(Draw::from_str).collect())
            .collect();

        Ok(Game { id, draws: draws? })
    }
}

#[test]
fn part1() -> Result<(), Box<dyn Error>> {
    let lines = crate::input(2).lines();

    let mut sum = 0;
    for line in lines {
        let game: Game = line?.parse()?;
        let valid = game.draws.iter().all(|draws| {
            draws.iter().all(|draw| match draw.color {
                Color::Red => draw.count <= 12,
                Color::Green => draw.count <= 13,
                Color::Blue => draw.count <= 14,
            })
        });
        if valid {
            sum += game.id;
        }
    }
    println!("part1: {sum}");
    Ok(())
}

fn get_needed_draws(game: &Game, color: Color) -> u32 {
    game.draws
        .iter()
        .map(|draws| {
            draws
                .iter()
                .find(|draw| draw.color == color)
                .map(|draw| draw.count)
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0)
}

#[test]
fn part2() -> Result<(), Box<dyn Error>> {
    let lines = crate::input(2).lines();

    let mut sum = 0;
    for line in lines {
        let game: Game = line?.parse()?;
        let min_red = get_needed_draws(&game, Color::Red);
        let min_blue = get_needed_draws(&game, Color::Blue);
        let min_green = get_needed_draws(&game, Color::Green);
        sum += min_red * min_blue * min_green;
    }
    println!("part2: {sum}");
    Ok(())
}

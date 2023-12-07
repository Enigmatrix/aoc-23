#![cfg(test)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Error;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone)]
struct Hand {
    cards: [char; 5],
}

impl Hand {
    pub fn new(s: &str) -> Hand {
        Hand {
            cards: s.chars().next_chunk().expect("hand cards chunk size 5"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let strengths = "AKQJT98765432";
        let seccond_rule = || {
            let self_idxs = self
                .cards
                .iter()
                .map(|c| strengths.find(|c1| *c == c1).expect("find card"));
            let other_idxs = other
                .cards
                .iter()
                .map(|c| strengths.find(|c1| *c == c1).expect("find card"));
            self_idxs
                .zip(other_idxs)
                .find(|(s, o)| s != o)
                .map(|(s, o)| o.cmp(&s))
                .expect("second rule never gives equal")
        };

        let first_rule_type = |hand: &Hand| {
            let mut freq = HashMap::new();
            for card in hand.cards {
                *freq.entry(card).or_insert(0) += 1;
            }
            let freqcount = *freq.iter().map(|(_, count)| count).max().unwrap();
            if freqcount == 5 {
                return 7;
            } else if freqcount == 4 {
                return 6;
            } else if freqcount == 3 {
                if freq.iter().any(|(c, count)| *count == 2) {
                    return 5;
                } else {
                    return 4;
                }
            } else if freqcount == 2 {
                if freq.iter().filter(|(c, count)| **count == 2).count() == 2 {
                    return 3;
                } else {
                    return 2;
                }
            }
            return 1;
        };

        Some(
            first_rule_type(self)
                .cmp(&first_rule_type(other))
                .then_with(seccond_rule),
        )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[test]
fn part1() -> Result<(), Error> {
    let inp = include_str!("../inputs/07.txt");
    let mut bids: Vec<_> = inp
        .lines()
        .map(|s| s.split_once(" ").expect("split line"))
        .map(|(cards, value)| (Hand::new(cards), value.parse::<i64>().expect("valid bid")))
        .collect();
    bids.sort_by_key(|(hand, _)| hand.clone());
    let ans: i64 = bids
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| bid * (idx + 1) as i64)
        .sum();

    println!("part1: {ans}");
    Ok(())
}

#[derive(PartialEq, Eq, Clone)]
struct Hand2 {
    cards: [char; 5],
}

impl Hand2 {
    pub fn new(s: &str) -> Hand2 {
        Hand2 {
            cards: s.chars().next_chunk().expect("hand cards chunk size 5"),
        }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let strengths = "AKQT98765432J";
        let seccond_rule = || {
            let self_idxs = self
                .cards
                .iter()
                .map(|c| strengths.find(|c1| *c == c1).expect("find card"));
            let other_idxs = other
                .cards
                .iter()
                .map(|c| strengths.find(|c1| *c == c1).expect("find card"));
            self_idxs
                .zip(other_idxs)
                .find(|(s, o)| s != o)
                .map(|(s, o)| o.cmp(&s))
                .expect("second rule never gives equal")
        };

        let first_rule_type = |hand: &Hand2| {
            let mut freq = HashMap::new();
            for card in hand.cards {
                *freq.entry(card).or_insert(0) += 1;
            }
            let jfreq = *freq.get(&'J').unwrap_or(&0);
            let (freqchar, freqcount) = freq
                .iter()
                .filter(|(c, _)| **c != 'J')
                .max_by_key(|(_, count)| *count)
                .unwrap_or((&'J', &0));
            if freqcount + jfreq == 5 {
                return 7;
            } else if freqcount + jfreq == 4 {
                return 6;
            } else if freqcount + jfreq == 3 {
                // 0, 1, 2 J
                // 0 J = same logic as before
                // 1 J = A A J B B or A A J B C
                // 2 J = A B J J C
                if freq
                    .iter()
                    .filter(|(c, _)| *c != freqchar && **c != 'J')
                    .any(|(_, count)| *count == 2)
                {
                    return 5;
                } else {
                    return 4;
                }
            } else if freqcount + jfreq == 2 {
                // only 0 or 1 J
                // 0 J = same as before
                // 1 J = A J B C D, can't be A J B B D else 3 pair
                if freq.iter().filter(|(_, count)| **count == 2).count() == 2 {
                    return 3;
                } else {
                    return 2;
                }
            }
            return 1;
        };

        Some(
            first_rule_type(self)
                .cmp(&first_rule_type(other))
                .then_with(seccond_rule),
        )
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[test]
fn part2() -> Result<(), Error> {
    let inp = include_str!("../inputs/07.txt");
    let mut bids: Vec<_> = inp
        .lines()
        .map(|s| s.split_once(" ").expect("split line"))
        .map(|(cards, value)| (Hand2::new(cards), value.parse::<i64>().expect("valid bid")))
        .collect();
    bids.sort_by_key(|(hand, _)| hand.clone());
    let ans: i64 = bids
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| bid * (idx + 1) as i64)
        .sum();

    println!("part2: {ans}");
    Ok(())
}

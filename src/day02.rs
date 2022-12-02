use crate::day02::Outcome::{Draw, Loss, Win};
use crate::day02::Play::{Paper, Rock, Scissors};
use aoc_runner_derive::aoc;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Play {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => unreachable!(),
        }
    }
}

impl Play {
    fn win_move(&self) -> Play {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn loss_move(&self) -> Self {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

impl Play {
    fn score(&self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Win => 6,
            Loss => 0,
            Draw => 3,
        }
    }
}

struct Round {
    opponent: Play,
    you: Play,
}

impl From<(u8, &str, &str)> for Round {
    fn from((p, o, y): (u8, &str, &str)) -> Self {
        let o = o.into();

        Self {
            opponent: o,
            you: match (p, y.into()) {
                (1, y) => y,
                (_, Paper) => o,
                (_, Rock) => o.loss_move(),
                (_, Scissors) => o.win_move(),
            },
        }
    }
}

impl From<(u8, &str)> for Round {
    fn from((p, value): (u8, &str)) -> Self {
        value
            .rsplit_once(' ')
            .map(|l| Round::from((p, l.0, l.1)))
            .unwrap()
    }
}

impl Round {
    fn score(&self) -> usize {
        self.you.score() + self.outcome().score()
    }

    fn outcome(&self) -> Outcome {
        match self {
            Round {
                opponent: Rock,
                you: Paper,
            } => Win,
            Round {
                opponent: Paper,
                you: Scissors,
            } => Win,
            Round {
                opponent: Scissors,
                you: Rock,
            } => Win,
            Round {
                opponent: x,
                you: y,
            } if x == y => Draw,
            _ => Loss,
        }
    }
}

struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn score(&self) -> usize {
        self.rounds.iter().map(Round::score).sum()
    }
}

fn parse_input_day2(p: u8, input: &str) -> Game {
    Game {
        rounds: input.lines().map(|l| Round::from((p, l))).collect(),
    }
}

#[aoc(day2, part1)]
fn day2_part1(input: &str) -> usize {
    parse_input_day2(1, input).score()
}

#[aoc(day2, part2)]
fn day2_part2(input: &str) -> usize {
    parse_input_day2(2, input).score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "A Y
B X
C Z";
        assert_eq!(day2_part1(&input), 15);
    }

    #[test]
    fn test_part2() {
        let input = "A Y
B X
C Z";
        assert_eq!(day2_part2(&input), 12);
    }
}

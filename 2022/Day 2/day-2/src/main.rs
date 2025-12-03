#![feature(iter_array_chunks)]

use std::fs;
use std::str::FromStr;
use std::time::SystemTime;

enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Lose,
    Tie,
}

impl FromStr for Result {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "X" => Ok(Result::Lose),
            "Y" => Ok(Result::Tie),
            "Z" => Ok(Result::Win),
            _ => Err(()),
        }
    }
}

impl FromStr for Choice {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

trait Compare<T, U> {
    fn compare(&self, b: &T) -> U;
}

trait HasScore {
    fn score(&self) -> u32;
}

impl HasScore for Result {
    fn score(&self) -> u32 {
        match self {
            Result::Win => 6,
            Result::Lose => 0,
            Result::Tie => 3,
        }
    }
}

impl HasScore for Choice {
    fn score(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

impl Compare<Choice, Result> for Choice {
    fn compare(&self, b: &Choice) -> Result {
        return match self {
            Choice::Rock => match b {
                Choice::Rock => Result::Tie,
                Choice::Paper => Result::Lose,
                Choice::Scissors => Result::Win,
            },
            Choice::Paper => match b {
                Choice::Rock => Result::Win,
                Choice::Paper => Result::Tie,
                Choice::Scissors => Result::Lose,
            },
            Choice::Scissors => match b {
                Choice::Rock => Result::Lose,
                Choice::Paper => Result::Win,
                Choice::Scissors => Result::Tie,
            },
        };
    }
}

impl Compare<Result, Choice> for Choice {
    fn compare(&self, b: &Result) -> Choice {
        match self {
            Choice::Rock => {
                match b {
                    Result::Win => {Choice::Paper}
                    Result::Lose => {Choice::Scissors}
                    Result::Tie => {Choice::Rock}
                }
            }
            Choice::Paper => {
                match b {
                    Result::Win => {Choice::Scissors}
                    Result::Lose => {Choice::Rock}
                    Result::Tie => {Choice::Paper}
                }
            }
            Choice::Scissors => {
                match b {
                    Result::Win => {Choice::Rock}
                    Result::Lose => {Choice::Paper}
                    Result::Tie => {Choice::Scissors}
                }
            }
        }
    }
}

fn main() {
    let start = SystemTime::now();

    let input = fs::read_to_string("./strategy_guide.txt").unwrap();
    let mut out: u32 = 0;
    for pair in input.split_whitespace().into_iter().array_chunks::<2>() {
        let opp = Choice::from_str(pair[0]).unwrap();
        let wanted = Result::from_str(pair[1]).unwrap();
        out += wanted.score() + opp.compare(&wanted).score();
    }
    println!("{out}");

    println!(
        "Time taken: {:?}",
        SystemTime::now().duration_since(start).unwrap()
    );
}

use std::str::FromStr;

enum Turn {
    Left(u64),
    Right(u64),
}

impl FromStr for Turn {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_at(1);
        let distance = dist.parse::<u64>().map_err(|_| "Invalid distance")?;
        match dir {
            "L" => Ok(Turn::Left(distance)),
            "R" => Ok(Turn::Right(distance)),
            _ => Err("Invalid direction"),
        }
    }
}

const INPUT: &str = include_str!("input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut current = 50;
    let mut out = 0;
    for line in INPUT.lines() {
        let turn: Turn = line.parse()?;
        match turn {
            Turn::Left(dist) => {
                // Count how many times we wrap past 0 going left
                // We wrap when current - dist < 0, and each 100 units is another wrap
                if dist > current {
                    out += (dist - current - 1) / 100 + 1;
                }
                let temp = (current + 100) as i64 - (dist as i64);
                current = temp.rem_euclid(100) as u64;
            }
            Turn::Right(dist) => {
                // Count how many times we wrap past 0 going right
                // We wrap when current + dist >= 100, and each 100 units is another wrap
                out += (current + dist) / 100;
                current = (current + dist) % 100;
            }
        }
    }
    println!("Number of times at position 0: {}", out);
    Ok(())
}

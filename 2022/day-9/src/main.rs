use std::collections::HashSet;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
struct Knot(i32, i32);

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

impl Direction {
    fn plus_n(&self, n: i32) -> Direction {
        match self {
            Direction::Left(amt) => Direction::Left(amt + n),
            Direction::Right(amt) => Direction::Right(amt + n),
            Direction::Up(amt) => Direction::Up(amt + n),
            Direction::Down(amt) => Direction::Down(amt + n),
        }
    }
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        let mut it = value.split_whitespace();
        match (it.next(), it.next()) {
            (Some("L"), Some(amt)) => Self::Left(amt.parse().expect("Couldn't Parse number")),
            (Some("R"), Some(amt)) => Self::Right(amt.parse().expect("Couldn't Parse number")),
            (Some("U"), Some(amt)) => Self::Up(amt.parse().expect("Couldn't Parse number")),
            (Some("D"), Some(amt)) => Self::Down(amt.parse().expect("Couldn't Parse number")),
            (_, _) => panic!("WHATTTTT"),
        }
    }
}
impl Knot {
    fn movement(&mut self, direction: &Direction) {
        match direction {
            &Direction::Left(amt) => self.0 -= amt as i32,
            &Direction::Right(amt) => self.0 += amt as i32,
            &Direction::Up(amt) => self.1 += amt as i32,
            &Direction::Down(amt) => self.1 -= amt as i32,
        }
    }

    fn follow_movement(&mut self, head: &mut Knot, tail_locations: &mut HashSet<Knot>, direction: Direction) {
        use Direction::*;
        // let curr_head = head.clone();
        if head == self {
            head.movement(&direction);
            self.movement(&direction.plus_n(-1))
        } else if head.0 != self.0 && head.1 != self.1 {
            let left_right = head.0 > self.0;
            let up_down = head.1 < self.0;
            match (left_right, up_down) {
                (true, true) if matches!(direction, Left(1) | Up(1)) => head.movement(&direction),
                (true, false) if matches!(direction, Left(1) | Down(1)) => head.movement(&direction),
                (false, true) if matches!(direction, Right(1) | Up(1)) => head.movement(&direction),
                (false, false) if matches!(direction, Right(1) | Down(1)) => head.movement(&direction),
                _ => {
                    head.movement(&direction);
                    self.movement(&direction.plus_n(-1))
                }
            }
        } else if head.0 == self.0 {
            match (head.1 > self.1, direction) {
                (_, Left(amt) | Right(amt)) if amt == 1 => head.movement(&direction),
                (true, Up(_)) | (false, Down(_)) | (_, _) => {
                    head.movement(&direction);
                    self.movement(&direction.plus_n(-1))
                },
            }
        } else {
            match (head.0 > self.0, direction) {
                (_, Up(amt) | Down(amt)) if amt == 1 => head.movement(&direction),
                (true, Right(_)) | (false, Left(_)) | (_, _) => {
                    head.movement(&direction);
                    self.movement(&direction.plus_n(-1))
                },
            }
        }
        tail_locations.insert(self.clone());
    }
}

fn main() {
    let mut head = Knot(0, 0);
    let mut tail = Knot(0, 0);
    let mut tail_locations = HashSet::new();

    let input = std::fs::read_to_string("input.txt").expect("Couldnt read file");
    let directions: Vec<Direction> = input.lines().map(Into::into).collect();

    dbg!(&directions);

    for direction in directions.into_iter() {
        tail.follow_movement(&mut head, &mut tail_locations, direction);
    }

    println!(
        "Total locations the tail has been on: {}",
        tail_locations.len()
    );
}

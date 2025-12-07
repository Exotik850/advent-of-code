use std::str::FromStr;

enum Slot {
    Roll,
    Empty,
}

impl FromStr for Slot {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..1] {
            "@" => Ok(Slot::Roll),
            "." => Ok(Slot::Empty),
            _ => Err(()),
        }
    }
}

struct Wall {
    slots: Vec<Slot>,
    width: usize,
    height: usize,
}

fn for_each_x_y<F>(mut f: F, width: usize, height: usize)
where
    F: FnMut(usize, usize),
{
    for y in 0..height {
        for x in 0..width {
            f(x, y);
        }
    }
}

impl Wall {
    fn get(&self, x: usize, y: usize) -> Option<&Slot> {
        if x < self.width && y < self.height {
            Some(&self.slots[y * self.width + x])
        } else {
            None
        }
    }

    fn num_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in [-1isize, 0, 1].iter() {
            for dx in [-1isize, 0, 1].iter() {
                if *dx == 0 && *dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 {
                    if let Some(Slot::Roll) = self.get(nx as usize, ny as usize) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn remove_valid_once(&mut self) -> u64 {
        let mut removed = 0;
        self.for_each_valid(|wall, x, y| {
            wall.slots[y * wall.width + x] = Slot::Empty;
            removed += 1;
        });
        removed
    }

    fn remove_valid(&mut self) -> u64 {
        let mut total_removed = 0;
        loop {
            let removed = self.remove_valid_once();
            if removed == 0 {
                break;
            }
            total_removed += removed;
        }
        total_removed
    }

    fn for_each_valid<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Self, usize, usize),
    {
        let width = self.width;
        let height = self.height;
        for_each_x_y(
            |x, y| {
                if let Slot::Roll = self.get(x, y).unwrap() {
                    let neighbors = self.num_neighbors(x, y);
                    if neighbors < 4 {
                        f(self, x, y);
                    }
                }
            },
            width,
            height,
        );
    }

    fn sum_valid(&mut self) -> u64 {
        let mut sum = 0;
        self.for_each_valid(|_, _, _| {
            sum += 1;
        });
        sum
    }
}

impl FromStr for Wall {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let height = lines.len();
        let width = lines.first().map_or(0, |line| line.len());
        let mut slots = Vec::with_capacity(width * height);

        for line in lines {
            let mut rest = line;
            while !rest.is_empty() {
                let Ok(slot) = rest.parse() else {
                    eprintln!("Failed to parse slot in line: {}", line);
                    break;
                };
                slots.push(slot);
                rest = &rest[1..];
            }
        }

        Ok(Wall {
            slots,
            width,
            height,
        })
    }
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut wall: Wall = INPUT.parse().expect("Failed to parse wall");
    println!("Wall dimensions: {}x{}", wall.width, wall.height);
    let valid_rolls = wall.sum_valid();
    println!("Number of valid rolls initially: {}", valid_rolls);
    let removed = wall.remove_valid();
    println!("Total rolls removed: {}", removed);
    let remaining_valid_rolls = wall.sum_valid();
    println!("Number of valid rolls remaining: {}", remaining_valid_rolls);
}

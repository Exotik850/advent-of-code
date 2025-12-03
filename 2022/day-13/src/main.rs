use std::{fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Num(i32),
}

impl Default for Packet {
    fn default() -> Self {
        Self::List(vec![])
    }
}

#[derive(Debug)]
struct BracketGroup<'a> {
    orig: &'a str,
    start: usize,
    finish: usize,
}

impl<'a> BracketGroup<'a> {
    fn as_str(&self) -> &'a str {
        &self.orig[self.start + 1..self.finish]
    }

    fn holds(&self, rhs: &Self) -> bool {
        self.orig.contains(rhs.orig)
    }

    // fn remainder(&self, rhs: &Self) -> Option<Self> {
    //   if !self.holds(rhs) {
    //     return None;
    //   }

    // }
}

impl<'a> Display for BracketGroup<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

trait GetGroup<'a> {
    fn get_group(&self) -> Option<BracketGroup<'a>>;
}

trait HasGroup {
    fn has_group(&self) -> bool;
}

impl HasGroup for &str {
    fn has_group(&self) -> bool {
        self.contains(|e| e == '[' || e == ']')
    }
}

impl<'a> HasGroup for BracketGroup<'a> {
    fn has_group(&self) -> bool {
        self.as_str().has_group()
    }
}

impl<'a> GetGroup<'a> for BracketGroup<'a> {
    fn get_group(&self) -> Option<BracketGroup<'a>> {
        (&self.orig[self.start + 1..self.finish]).get_group()
    }
}

impl<'a> GetGroup<'a> for &'a str {
    fn get_group(&self) -> Option<BracketGroup<'a>> {
        if !self.has_group() {
            return None;
        }

        let mut start = 0;
        let mut group = 0;
        let Some(finish) = self.chars().enumerate().position(|(i, e)| {
            match e {
                '[' if group == 0 => {
                    start = i;
                    group += 1
                }
                '[' => group += 1,
                ']' if group == 1 => return true,
                ']' => group -= 1,
                _ => (),
            }
            false
        }) else {
            return None;
        };

        Some(BracketGroup {
            orig: *self,
            start,
            finish,
        })
    }
}

impl FromStr for Packet {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.is_empty() {
      return Ok(Packet::default());
    }

    let mut stack = vec![];
    let mut current = vec![];

    for c in s.chars() {
      match c {
        '[' => {
          stack.push(current);
          current = vec![];
        }
        ']' => {
          if let Some(last) = stack.pop() {
            let val = Packet::List(current);
            current = last;
            current.push(val);
          }
        }
        ',' => {}
        c if c.is_digit(10) => {
          let num = c.to_string().parse::<i32>()?;
          current.push(Packet::Num(num));
        }
        _ => return Err("a".parse::<i32>().err().unwrap()),
      }
    }
    if current.len() == 1 {
      return Ok(current.pop().unwrap())
    }
    Ok(Packet::List(current))
  }
}

use anyhow::Result;
fn main() -> Result<()> {
    let input = std::fs::read_to_string("./input.txt")?;
    let start = std::time::Instant::now();
    for line in input.lines() .nth(4).iter() {
        // dbg!(line);
        if line.len() > 10 {
            // println!("{:?}", line.get_group().expect("!"));
            // let l = line.get_group().expect("!");
            // let l2 = (&line[1..]).get_group().expect("!2");
            // println!("{l}\n\t{l2}");
            // let mut l = line.get_group().unwrap();
            // println!("{l}");
            // while let Some(la) = l.get_group() {
            //     l = la;
            //     println!("{l}");
            // }
            let pack: Packet = line.parse()?;
            println!("{:?}", pack);
        }
    }

    println!("Time taken: {:?}", start.elapsed());
    Ok(())
}

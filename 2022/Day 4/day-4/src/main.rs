use std::fmt::{Debug, Formatter};
use std::fs;
use std::str::FromStr;
use std::time::SystemTime;

struct Range(u32, u32);

trait Compare {
    fn contains(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl Compare for Range {
    fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Self) -> bool {
        todo!()
    }
}

impl FromStr for Range {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .replace('\r', "")
            .split("-")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        assert_eq!(nums.len(), 2);
        Ok(Range(nums[0], nums[1]))
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("{}-{}", self.0, self.1))
    }
}

fn main() {
    let input = fs::read_to_string("pairs.txt").unwrap();
    let start = SystemTime::now();
    let pairs = input.split('\n').collect::<Vec<&str>>();
    let pairs = pairs
        .iter()
        .map(|&x| x.split(','))
        .collect::<Vec<Vec<&str>>>();

    pairs.iter().for_each(|x| println!("{:?}", x)

    let pairs = pairs
        .iter()
        .map(|x| {
            x.iter()
                .map(|&x| Range::from_str(x).unwrap())
                .collect::<Vec<Range>>()
        })
        .collect::<Vec<Vec<Range>>>();
    let mut sum = 0;
    for pair in pairs.iter() {
        if pair[0].contains(&pair[1]) || pair[1].contains(&pair[0]) {
            sum += 1;
        }
    }
    println!("{pairs:?}");
    println!("{sum}");
    println!(
        "Time taken: {:?}",
        SystemTime::now().duration_since(start).unwrap()
    );
}

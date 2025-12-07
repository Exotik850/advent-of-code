use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let mut leftt = vec![];
    let mut rightt = vec![];
    for (left, right) in input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(left, right)| {
            (
                left.trim().parse::<u32>().unwrap(),
                right.trim().parse::<u32>().unwrap(),
            )
        })
    {
        leftt.push(left);
        rightt.push(right);
    }
    leftt.sort_unstable();
    rightt.sort_unstable();

    let mut count = HashMap::new();

    for r in rightt.iter() {
        *count.entry(r).or_insert(0) += 1;
    }

    let out: u32 = leftt
        .iter()
        // .zip(rightt.iter())
        .map(|l| l * count.get(l).unwrap_or(&0))
        .sum();
    println!("{}", out);
}

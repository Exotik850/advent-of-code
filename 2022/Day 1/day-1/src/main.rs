use std::fs;
use std::time::SystemTime;
use rayon::prelude::*;

fn main() {
    let start = SystemTime::now();
    let input = fs::read_to_string("./src/elf_food.txt").expect("No file found");
    let nums: Vec<&str> = input.split("\r").collect();
    let num = nums.iter().filter(|&x| x == &"\n").count() + 1;
    let mut iter = nums.iter();
    let mut elves = vec![0u32; num];
    elves[0] += iter.next().unwrap().parse::<u32>().expect("Not a number!");
    let mut idx = 0usize;
    while let Some(&i) = iter.next() {
        match i {
            "\n" => idx += 1,
            calories => {
                let num = calories[1..].parse::<u32>().expect("Not a number!");
                elves[idx] += num;
            }
        }
    }
    elves.sort();
    elves.reverse();
    println!("{}", elves[..3].iter().sum::<u32>());
    println!("Time taken: {:?}", SystemTime::now().duration_since(start).unwrap())
}
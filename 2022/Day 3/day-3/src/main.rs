#![feature(iter_array_chunks)]

use std::fs;
use std::time::SystemTime;

trait Priority {
    fn priority(&self) -> u8;
}

impl Priority for char {
    fn priority(&self) -> u8 {
        match self {
            'a'..='z' => (*self as u8) - 96,
            'A'..='Z' => (*self as u8) - 38,
            a => panic!("Not a valid character! {}", a)
        }
    }
}

fn main() {
    let input = fs::read_to_string("rucksacks.txt").unwrap();

    let start = SystemTime::now();
    let rucksacks: Vec<[&str; 2]> = input.split('\n').map(|x| [&x[..x.len() / 2], &x[x.len() / 2..x.len() - 1]]).collect();
    let num = rucksacks.len();
    let mut chars: Vec<char> = vec!['a'; num];
    for (c, sack) in chars.iter_mut().zip(rucksacks) {
        for i in sack[0].chars() {
            if sack[1].contains(i) {
                *c = i;
                break;
            }
        }
    }
    let out = chars.iter().map(|x| x.priority() as u32).sum::<u32>();
    println!("{out}");
    println!("Time taken: {:?}", SystemTime::now().duration_since(start).unwrap());

    let start = SystemTime::now();
    let iter = input.split('\n').array_chunks::<3>();
    let mut chars = vec!['a'; num / 3];
    for (c, chunk) in chars.iter_mut().zip(iter) {
        for i in chunk[0].chars() {
            if chunk[1].contains(i) && chunk[2].contains(i) {
                *c = i;
                break;
            }
        }
    }
    let bind = chars.iter().map(|x| x.priority() as u32).sum::<u32>();
    println!("{bind}");
    println!("Time taken: {:?}", SystemTime::now().duration_since(start).unwrap());

}

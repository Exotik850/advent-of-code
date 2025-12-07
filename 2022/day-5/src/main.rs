use std::ops::IndexMut;

/*
    [T] [V]                     [W]
    [V] [C] [P] [D]             [B]
    [J] [P] [R] [N] [B]         [Z]
    [W] [Q] [D] [M] [T]     [L] [T]
    [N] [J] [H] [B] [P] [T] [P] [L]
    [R] [D] [F] [P] [R] [P] [R] [S] [G]
    [M] [W] [J] [R] [V] [B] [J] [C] [S]
    [S] [B] [B] [F] [H] [C] [B] [N] [L]
    1   2   3   4   5   6   7   8   9
*/
use anyhow::Result;

fn main() -> Result<()> {
    let lines = std::fs::read_to_string("input.txt")?;
    let (mut towers, skip) = get_towers(&lines)?;
    let instructions = get_instructions(&lines, skip)?;

    for (num, from, to) in instructions.iter() {
        #[cfg(feature="part-one")] 
        {
            for _ in 0..*num {
                match towers[*from].pop() {
                    Some(b) => towers.index_mut(*to).push(b),
                    None => (),
                };
            }
        }

        #[cfg(not(feature="part-one"))]
        {
            let num = *num as usize;
            let l = towers[*from].len();
            let last: Vec<_> = towers[*from].drain(l - num..).collect();
            towers[*to].extend(last);
        }
    }

    let result: String = towers
        .into_iter()
        .map(|mut a| a.pop().unwrap_or(' '))
        .collect();
    println!("{result}");

    Ok(())
}

fn get_towers(line: &str) -> Result<(Vec<Vec<char>>, usize)> {
    let (skip, bins) = line
        .lines()
        .enumerate()
        .find(|(i, l)| l.trim().chars().next().unwrap_or('[') != '[')
        .expect("Couldn't find the number of lines till the final input of crates");

    let max = bins.trim().chars().last().unwrap().to_digit(10).unwrap() as usize;

    let mut out = vec![vec![]; max];

    let mut input: Vec<&str> = line.lines().take(skip).collect();
    input.reverse();

    for line in input.into_iter() {
        for (i, c) in line
            .chars()
            .enumerate()
            .filter_map(|(i, c)| match matches!(c, 'A'..='Z') {
                true => Some((i, c)),
                false => None,
            })
        {
            out.index_mut((i - 1) / 4).push(c);
        }
    }

    Ok((out, skip + 1))
}

fn get_instructions(lines: &str, skip: usize) -> Result<Vec<(u32, usize, usize)>> {
    lines
        .lines()
        .skip(skip)
        .map(|line| -> Result<(_, _, _)> {
            let words: Vec<&str> = line.split_whitespace().collect();
            Ok((
                words[1].parse()?,
                words[3].parse::<usize>()?.saturating_sub(1),
                words[5].parse::<usize>()?.saturating_sub(1),
            ))
        })
        .collect()
}

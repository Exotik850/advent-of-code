use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_grid(path: &Path) -> io::Result<Vec<Vec<u32>>> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row: Vec<u32> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        grid.push(row);
    }

    Ok(grid)
}

fn is_visible(grid: &[Vec<u32>], x: usize, y: usize) -> bool {
    let number = grid[y][x];

    let up = (0..y).rev().all(|i| grid[i][x] < number);
    let down = ((y + 1)..grid.len()).all(|i| grid[i][x] < number);
    let left = (0..x).rev().all(|i| grid[y][i] < number);
    let right = ((x + 1)..grid[0].len()).all(|i| grid[y][i] < number);

    up || down || left || right
}

fn count_visible(grid: &[Vec<u32>]) -> u32 {
    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if is_visible(grid, x, y) {
                count += 1;
            }
        }
    }

    count
}

fn main() -> io::Result<()> {
    let grid = read_grid(Path::new("input.txt"))?;
    let visible = count_visible(&grid);

    println!("Visible: {}", visible);

    Ok(())
}
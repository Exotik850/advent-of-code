fn find_all_x_mas(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // For each possible center point of the X
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            // Check all four possible combinations:
            // 1. Both MAS forward
            // 2. Both MAS backward
            // 3. Upper-right forward, lower-left forward
            // 4. Upper-right backward, lower-left backward

            let upper_left = check_mas(grid, i - 1, j - 1, 1, 1);
            let upper_right = check_mas(grid, i - 1, j + 1, 1, -1);
            let lower_left = check_mas(grid, i + 1, j - 1, -1, 1);
            let lower_right = check_mas(grid, i + 1, j + 1, -1, -1);

            let reverse_upper_left = check_mas_reverse(grid, i - 1, j - 1, 1, 1);
            let reverse_upper_right = check_mas_reverse(grid, i - 1, j + 1, 1, -1);
            let reverse_lower_left = check_mas_reverse(grid, i + 1, j - 1, -1, 1);
            let reverse_lower_right = check_mas_reverse(grid, i + 1, j + 1, -1, -1);

            // Check all valid combinations
            if (upper_left && lower_right)
                || (reverse_upper_left && reverse_lower_right)
                || (upper_right && lower_left)
                || (reverse_upper_right && reverse_lower_left)
            {
                count += 1;
            }
        }
    }

    count
}

// Check for "MAS" starting from the given position in the given direction
fn check_mas(grid: &[Vec<char>], start_i: usize, start_j: usize, di: i32, dj: i32) -> bool {
    let pattern = ['M', 'A', 'S'];
    let mut i = start_i as i32;
    let mut j = start_j as i32;

    for &c in &pattern {
        if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
            return false;
        }
        if grid[i as usize][j as usize] != c {
            return false;
        }
        i += di;
        j += dj;
    }
    true
}

// Check for "SAM" (MAS backwards) starting from the given position
fn check_mas_reverse(grid: &[Vec<char>], start_i: usize, start_j: usize, di: i32, dj: i32) -> bool {
    let pattern = ['S', 'A', 'M'];
    let mut i = start_i as i32;
    let mut j = start_j as i32;

    for &c in &pattern {
        if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
            return false;
        }
        if grid[i as usize][j as usize] != c {
            return false;
        }
        i += di;
        j += dj;
    }
    true
}

// Optional: Function to create a grid showing only the X-MAS patterns
fn create_marked_grid(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut marked = vec![vec!['.'; cols]; rows];

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let upper_left = check_mas(grid, i - 1, j - 1, 1, 1);
            let upper_right = check_mas(grid, i - 1, j + 1, 1, -1);
            let lower_left = check_mas(grid, i + 1, j - 1, -1, 1);
            let lower_right = check_mas(grid, i + 1, j + 1, -1, -1);

            let reverse_upper_left = check_mas_reverse(grid, i - 1, j - 1, 1, 1);
            let reverse_upper_right = check_mas_reverse(grid, i - 1, j + 1, 1, -1);
            let reverse_lower_left = check_mas_reverse(grid, i + 1, j - 1, -1, 1);
            let reverse_lower_right = check_mas_reverse(grid, i + 1, j + 1, -1, -1);

            if (upper_left && lower_right)
                || (reverse_upper_left && reverse_lower_right)
                || (upper_right && lower_left)
                || (reverse_upper_right && reverse_lower_left)
            {
                // Mark the X pattern
                if upper_left && lower_right {
                    mark_mas(&mut marked, i - 1, j - 1, 1, 1);
                    mark_mas(&mut marked, i + 1, j + 1, -1, -1);
                } else if reverse_upper_left && reverse_lower_right {
                    mark_mas_reverse(&mut marked, i - 1, j - 1, 1, 1);
                    mark_mas_reverse(&mut marked, i + 1, j + 1, -1, -1);
                } else if upper_right && lower_left {
                    mark_mas(&mut marked, i - 1, j + 1, 1, -1);
                    mark_mas(&mut marked, i + 1, j - 1, -1, 1);
                } else if reverse_upper_right && reverse_lower_left {
                    mark_mas_reverse(&mut marked, i - 1, j + 1, 1, -1);
                    mark_mas_reverse(&mut marked, i + 1, j - 1, -1, 1);
                }
            }
        }
    }

    marked
}

fn mark_mas(grid: &mut Vec<Vec<char>>, start_i: usize, start_j: usize, di: i32, dj: i32) {
    let pattern = ['M', 'A', 'S'];
    let mut i = start_i as i32;
    let mut j = start_j as i32;

    for &c in &pattern {
        grid[i as usize][j as usize] = c;
        i += di;
        j += dj;
    }
}

fn mark_mas_reverse(grid: &mut Vec<Vec<char>>, start_i: usize, start_j: usize, di: i32, dj: i32) {
    let pattern = ['S', 'A', 'M'];
    let mut i = start_i as i32;
    let mut j = start_j as i32;

    for &c in &pattern {
        grid[i as usize][j as usize] = c;
        i += di;
        j += dj;
    }
}

// Example usage:
fn main() {
    let grid = include_str!("grid.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let count = find_all_x_mas(&grid);
    println!("Found {} occurrences of XMAS", count);

    // Print marked grid
    let marked = create_marked_grid(&grid);
    for row in marked {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

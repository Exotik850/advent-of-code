use std::io::Write;

const INPUT: &str = include_str!("sample.txt");

struct Homework {
    nums: Vec<u64>,
    width: usize,
    height: usize,
    operators: Vec<char>,
}

fn nth_digit(num: u64, n: usize) -> Option<u64> {
    let mut temp = num;
    for _ in 0..n {
        temp /= 10;
        if temp == 0 {
            return None;
        }
    }
    Some(temp % 10)
}

impl Homework {
    fn for_column<F>(&self, idx: usize, mut f: F)
    where
        F: FnMut(u64),
    {
        for row in 0..self.height {
            let index = row * self.width + idx;
            f(self.nums[index]);
        }
    }

    fn longest_digit(&self, idx: usize) -> usize {
        let mut max_len = 0;
        self.for_column(idx, |num| {
            let len = num.ilog10() as usize + 1;
            if len > max_len {
                max_len = len;
            }
        });
        max_len
    }

    fn nth_colwise_number(&self, col: usize, n: usize) -> Option<u64> {
        let mut result = None;
        self.for_column(col, |num| {
            if let Some(digit) = nth_digit(num, n) {
                match result {
                    None => result = Some(digit),
                    Some(acc) => result = Some(acc * 10 + digit),
                }
            }
        });
        result
    }

    fn colwise_numbers(&self, col: usize) -> Vec<u64> {
        let mut result = Vec::new();
        let max_len = self.longest_digit(col);
        for n in 0..max_len {
            if let Some(num) = self.nth_colwise_number(col, n) {
                result.push(num);
            }
        }
        result
    }

    fn eval(&self) -> u64 {
        let mut result = 0;
        for (i, op) in self.operators.iter().enumerate() {
            print!("Evaluating column {} with operator '{}'..", i, op);
            std::io::stdout().flush().unwrap();
            let mut col_sum = None;
            self.for_column(i, |num| {
                col_sum = Some(match (col_sum, op) {
                    (None, _) => num,
                    (Some(acc), '+') => acc + num,
                    (Some(acc), '*') => acc * num,
                    _ => panic!("Unknown operator"),
                });
            });
            let col_sum = col_sum.expect("Column should have at least one number");
            // std::thread::sleep(std::time::Duration::from_millis(500));
            println!("Column {} result: {}", i, col_sum);
            result += col_sum;
        }
        result
    }
}

impl std::str::FromStr for Homework {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = Vec::new();
        let mut height = 0;
        let mut width = 0;

        let mut lines = s.lines().peekable();

        // read all lines with numbers first
        while let Some(&line) = lines.peek() {
            if !line.chars().next().map_or(false, |c| c.is_digit(10)) {
                break;
            }
            let row_nums: Vec<u64> = line
                .split_whitespace()
                .map(|num| num.parse::<u64>().map_err(|_| "Invalid number"))
                .collect::<Result<_, _>>()?;
            if row_nums.iter().any(|n| *n == 0) {
                return Err("Numbers must be positive non-zero");
            }
            if width == 0 {
                width = row_nums.len();
            } else if row_nums.len() != width {
                return Err("Inconsistent row width");
            }
            nums.extend(row_nums);
            height += 1;
            lines.next(); // consume the line
        }
        if height == 0 {
            return Err("No numbers found");
        }

        // after the numbers, there should as many operators as width
        let operators: Vec<char> = lines
            .flat_map(|line| line.chars().filter(|c| !c.is_whitespace()))
            .take(width)
            .collect();
        if operators.len() != width {
            return Err("Not enough operators");
        }
        Ok(Homework {
            nums,
            width,
            height,
            operators,
        })
    }
}

fn main() {
    let homework: Homework = INPUT.parse().expect("Failed to parse homework");
    println!(
        "Parsed homework with {} rows and {} columns",
        homework.height, homework.width
    );
    let result = homework.eval();
    println!("Evaluation result: {}", result);
}

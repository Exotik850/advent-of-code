enum Token {
    Noop,
    Addx(i32),
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        use Token::*;
        let mut tokens = value.split_whitespace();
        match (tokens.next(), tokens.next()) {
            (Some("addx"), Some(num)) => Addx(num.parse().expect("Couldn't parse number")),
            (Some("noop"), None) => Noop,
            (_, _) => panic!("WHAT T EE"),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Could not read input file");
    let tokens: Vec<Token> = input.lines().map(Into::into).collect();

    let sample_points = [20, 60, 100, 140, 180, 220];

    let mut cycle = 0;
    let mut total = 0;
    let mut sum = 0;
    for token in tokens.into_iter() {
        match token {
            Token::Noop => cycle += 1,
            Token::Addx(val) => {
                if let Some(amt) = sample_points.iter().find(|&c| *c >= cycle && *c <= cycle + 2) {
                    sum += total * amt;
                } 
                cycle += 2;
                total += val
            }
        }
    }
    dbg!(sum);
}

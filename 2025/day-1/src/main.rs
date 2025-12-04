const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut current = 50;
    let mut out = 0;
    for line in INPUT.lines() {
        if line.starts_with("R") {
            current += line.trim_start_matches("R").parse::<i32>().unwrap();
        } else if line.starts_with("L") {
            current -= line.trim_start_matches("L").parse::<i32>().unwrap();
        }
        while current > 99 {
            current -= 100;
            out += 1;
        }
        while current < 0 {
            current += 100;
            out += 1;
        }
        if current == 0 {
            out += 1;
        }
    }
    println!("{}", out);
}

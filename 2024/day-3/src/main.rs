use regex_lite::Regex;

fn main() {
    let re = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();
    let input = include_str!("input.txt");

    let mut sum = 0;
    let mut enabled = true;
    for op in re.find_iter(input).map(|o| o.as_str()) {
        let op = match op {
            "don't()" => {
                enabled = false;
                continue;
            }
            "do()" => {
                enabled = true;
                continue;
            }
            _ => {
                if !enabled {
                    continue;
                }
                op
            }
        };

        let nums = op.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap();
        let (left, right) = nums.split_at(nums.find(',').unwrap());
        let left = left.parse::<i32>().unwrap();
        let right = right[1..].parse::<i32>().unwrap();
        println!("{} * {} = {}", left, right, left * right);
        sum += left * right;
    }
    println!("Sum: {}", sum);
}

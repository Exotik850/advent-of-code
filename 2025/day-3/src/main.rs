const INPUT: &str = include_str!("input.txt");

// Finds the largest n-digit in the input string,
// where n is the max number of digits to search for.
//
// Digits can be chosen non-contiguously, but must preserve order.
// E.g. if input is "abc123def45gh6" and n is 2, it should return 56.
// if the input is "911122334", it should return 94, as 9 and 4 combined make 94.
//
fn largest_n_digits(input: &str, n: usize) -> u64 {
    if input.is_empty() || n == 0 {
        return 0;
    }

    // Collect all digits in order
    let digits: Vec<u64> = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| (c as u8 - b'0') as u64)
        .collect();

    if digits.is_empty() {
        return 0;
    }

    let len = digits.len();
    let max_len = n.min(len);

    // DP approach: dp[j] = largest value using exactly j digits from digits seen so far
    // We use None to represent "impossible" states
    let mut dp: Vec<Option<u64>> = vec![None; max_len + 1];
    dp[0] = Some(0); // Base case: selecting 0 digits gives value 0

    // Process each digit
    for &digit in &digits {
        // Iterate j from high to low to avoid using updated values in the same iteration
        for j in (1..=max_len).rev() {
            if let Some(prev_val) = dp[j - 1] {
                let new_val = prev_val * 10 + digit;
                dp[j] = Some(dp[j].map_or(new_val, |curr| curr.max(new_val)));
            }
        }
    }

    // Find the maximum value across all lengths from 1 to max_len
    (1..=max_len).filter_map(|j| dp[j]).max().unwrap_or(0)
}

fn main() {
    let num: u64 = INPUT.lines().map(|line| largest_n_digits(line, 12)).sum();
    println!("Sum of largest 12-digit numbers: {}", num);
}

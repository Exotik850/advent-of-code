fn main() {
    let input = include_str!("input.txt");
    let mut safe = 0;
    for line in input.lines().map(|l| l.split(" ")) {
        let line = line
            .map(|l| l.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
            .unwrap();
        if verify_report_with_dampener(&line) {
            safe += 1;
        }
    }
    if safe > input.lines().count() {
        println!("Safe: {} > Total: {} !!", safe, input.lines().count());
    } else {
        println!("{}", safe);
    }
}

fn verify_report_with_dampener(report: &[i32]) -> bool {
    // First check if report is safe without removing any level
    if verify_report(report) {
        return true;
    }

    // If not safe, try removing each level one at a time
    for skip_idx in 0..report.len() {
        let filtered_report: Vec<i32> = report
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != skip_idx)
            .map(|(_, &x)| x)
            .collect();

        if verify_report(&filtered_report) {
            return true;
        }
    }

    false
}

fn verify_report(report: &[i32]) -> bool {
    if report.len() < 2 {
        return false;
    }

    // Get the first difference to establish direction
    let mut prev_diff = report[1] - report[0];

    // Check if first difference is valid
    if prev_diff.abs() < 1 || prev_diff.abs() > 3 {
        return false;
    }

    // Check remaining differences
    for i in 1..report.len() - 1 {
        let curr_diff = report[i + 1] - report[i];

        // Check if difference maintains same direction
        if curr_diff.signum() != prev_diff.signum() {
            return false;
        }

        // Check if difference is within valid range
        if curr_diff.abs() < 1 || curr_diff.abs() > 3 {
            return false;
        }

        prev_diff = curr_diff;
    }

    true
}

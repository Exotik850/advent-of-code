use std::ops::RangeInclusive;

struct IdRange {
    range: RangeInclusive<u64>,
}

impl std::str::FromStr for IdRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err("Invalid format. Expected 'start-end'.".to_string());
        }

        let start: u64 = parts[0]
            .parse()
            .map_err(|_| "Invalid start value.".to_string())?;
        let end: u64 = parts[1]
            .parse()
            .map_err(|_| "Invalid end value.".to_string())?;
        Ok(IdRange { range: start..=end })
    }
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = INPUT.trim();
    let mut lines = input.lines();
    let mut ranges = Vec::new();
    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        match line.parse::<IdRange>() {
            Ok(id_range) => ranges.push(id_range),
            Err(e) => eprintln!("Error parsing line '{}': {}", line, e),
        }
    }
    // parse the ids from the remaining lines
    let ids: Vec<u64> = lines.filter_map(|line| line.trim().parse().ok()).collect();
    let mut found_count = 0;
    for id in ids {
        let mut found = false;
        for id_range in &ranges {
            if id_range.range.contains(&id) {
                found = true;
                break;
            }
        }
        if found {
            found_count += 1;
        }
    }
    println!("Found {} IDs within the ranges.", found_count);

    // sum of (unique) range lengths
    // Combine overlapping ranges first
    ranges.sort_unstable_by_key(|r| *r.range.start());
    let mut combined_ranges: Vec<IdRange> = Vec::new();
    for id_range in ranges {
        if let Some(last) = combined_ranges.last_mut() {
            if id_range.range.start() <= last.range.end() {
                last.range = *last.range.start()..=*last.range.end().max(id_range.range.end());
                continue;
            }
        }
        combined_ranges.push(id_range);
    }
    let total_range_length: u64 = combined_ranges
        .iter()
        .map(|r| r.range.end() - r.range.start() + 1)
        .sum();
    println!("Total range length: {}", total_range_length);
}

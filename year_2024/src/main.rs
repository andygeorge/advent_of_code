use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Get the input file from command-line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    let file_name = &args[1];

    // Read and parse the file
    let reports = match read_file(file_name) {
        Ok(reports) => reports,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    // Count the safe reports
    let safe_count = reports.iter().filter(|report| is_safe(report)).count();

    println!("Number of safe reports: {}", safe_count);
}

fn read_file(file_name: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(file_name)?;
    let reports = io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect();
    Ok(reports)
}

fn is_safe(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return false;
    }

    // Check if the report is increasing or decreasing
    let is_increasing = report.windows(2).all(|pair| pair[1] > pair[0]);
    let is_decreasing = report.windows(2).all(|pair| pair[1] < pair[0]);

    // Check the differences between adjacent levels
    let valid_differences = report.windows(2).all(|pair| {
        let diff = (pair[1] - pair[0]).abs();
        diff >= 1 && diff <= 3
    });

    // Return true if both conditions are satisfied
    (is_increasing || is_decreasing) && valid_differences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        let left_list: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
        let right_list: Vec<i32> = vec![4, 3, 5, 3, 9, 3];

        let mut left_sorted: Vec<i32> = left_list.clone();
        let mut right_sorted: Vec<i32> = right_list.clone();
        left_sorted.sort();
        right_sorted.sort();

        let total_distance: i32 = left_sorted.iter()
                                            .zip(right_sorted.iter())
                                            .map(|(l, r)| (l - r).abs())
                                            .sum();

        // Assert the result is as expected
        assert_eq!(total_distance, 11);
    }

    #[test]
    fn day2() {
        let reports = vec![
            vec![7, 6, 4, 2, 1], // Safe
            vec![1, 2, 7, 8, 9], // Unsafe (2 -> 7 difference is 5)
            vec![9, 7, 6, 2, 1], // Unsafe (6 -> 2 difference is 4)
            vec![1, 3, 2, 4, 5], // Unsafe (mix of increasing and decreasing)
            vec![8, 6, 4, 4, 1], // Unsafe (4 -> 4 is no change)
            vec![1, 3, 6, 7, 9], // Safe
        ];

        let expected_results = vec![true, false, false, false, false, true];

        for (i, report) in reports.iter().enumerate() {
            assert_eq!(
                is_safe(report),
                expected_results[i],
                "Failed on report {:?}",
                report
            );
        }
    }
}

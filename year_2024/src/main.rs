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

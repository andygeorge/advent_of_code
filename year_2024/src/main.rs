use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Get the file name from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    let file_name = &args[1];

    // Read and parse the file
    let (left_list, right_list) = match read_file(file_name) {
        Ok((left, right)) => (left, right),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    // Ensure both lists are of equal length
    if left_list.len() != right_list.len() {
        eprintln!("Error: The two lists must have the same number of elements.");
        std::process::exit(1);
    }

    // Sort both lists
    let mut sorted_left = left_list.clone();
    let mut sorted_right = right_list.clone();
    sorted_left.sort_unstable();
    sorted_right.sort_unstable();

    // Compute the total distance
    let total_distance: i32 = sorted_left
        .iter()
        .zip(sorted_right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("The total distance between the two lists is: {}", total_distance);
}

fn read_file(file_name: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    let file = File::open(file_name)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let columns: Vec<&str> = line.split_whitespace().collect();
        if columns.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Each line must have exactly two columns",
            ));
        }

        // Parse the numbers from each column
        let left_num = columns[0].parse::<i32>().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Invalid number in left column")
        })?;
        let right_num = columns[1].parse::<i32>().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Invalid number in right column")
        })?;

        left_list.push(left_num);
        right_list.push(right_num);
    }

    Ok((left_list, right_list))
}

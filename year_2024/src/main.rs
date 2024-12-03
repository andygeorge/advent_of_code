use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Get the input file from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    let file_name = &args[1];

    // Read the input from the file
    let input = match read_file(file_name) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    // Parse the input and compute the result
    let result = parse_and_compute(&input);

    println!("The sum of all valid mul results is: {}", result);
}

fn read_file(file_name: &str) -> io::Result<String> {
    let file = File::open(file_name)?;
    let mut content = String::new();
    for line in io::BufReader::new(file).lines() {
        content.push_str(&line?);
    }
    Ok(content)
}

fn parse_and_compute(input: &str) -> i32 {
    // Regex to match valid `mul(X,Y)` instructions
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Sum the results of all valid multiplications
    re.captures_iter(input)
        .map(|cap| {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            x * y
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_compute() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(parse_and_compute(input), 161);

        let input = "mul(1,1)mul(2,2)mul(3,3)";
        assert_eq!(parse_and_compute(input), 14);

        let input = "no_valid_instructions_here";
        assert_eq!(parse_and_compute(input), 0);

        let input = "mul(12,34) and mul(5,6)";
        assert_eq!(parse_and_compute(input), 462);
    }
}

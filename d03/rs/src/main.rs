use regex::Regex;
use std::{fs::read_to_string, io::Error, time::Instant};

static FILE_PATH: &str = "../sample_2.txt";

fn main() -> Result<(), Error> {
    let start = Instant::now();
    let content = read_to_string(FILE_PATH)?;
    let result_one = parse_part_one(&content);
    let result_two = parse_part_two(&content);

    println!("Result: {}", result_one - result_two);
    println!("Time elapsed: {:?}", start.elapsed());
    Ok(())
}

#[allow(dead_code)]
fn parse_part_one(content: &str) -> i64 {
    calculate_mul_pattern(content)
}

#[allow(dead_code)]
fn parse_part_two(content: &str) -> i64 {
    let control_pattern: Regex = Regex::new(r"(?:don't|do)\(\)").unwrap();

    let mut disabled_sum = 0;
    let mut is_disabled = false;
    let mut last_pos = 0;

    // Find all don't() and do() positions
    for control_match in control_pattern.find_iter(content) {
        let control = control_match.as_str();
        let current_pos = control_match.start();

        // If we're in a disabled section, process multiplications up to this point
        if is_disabled {
            let section = &content[last_pos..current_pos];
            disabled_sum += calculate_mul_pattern(section);
        }

        is_disabled = control == "don't()";
        last_pos = control_match.end();
    }

    // Handle the last section if we're still disabled
    if is_disabled {
        disabled_sum += calculate_mul_pattern(&content[last_pos..]);
    }

    disabled_sum
}

#[allow(dead_code)]
fn calculate_mul_pattern(content: &str) -> i64 {
    let mul_pattern: Regex = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();

    mul_pattern
        .captures_iter(content)
        .map(|cap| {
            if cap.name("a").is_none() || cap.name("b").is_none() {
                return 0;
            }
            let a = cap.name("a").unwrap().as_str().parse::<i64>().unwrap();
            let b = cap.name("b").unwrap().as_str().parse::<i64>().unwrap();
            a * b
        })
        .sum::<i64>()
}

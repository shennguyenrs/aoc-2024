use rayon::prelude::*;
use std::{fs::read_to_string, io::Error, time::Instant};

static FILE_PATH: &str = "../input.txt";
static IS_INCREASING: fn(i64, i64) -> bool = |a, b| a < b;
static IS_DECREASING: fn(i64, i64) -> bool = |a, b| a > b;

fn main() -> Result<(), Error> {
    let start = Instant::now();
    let content = read_to_string(FILE_PATH)?;
    let result = parse_part_two(&content);

    println!("Result: {}", result);
    println!("Elapsed times: {:?}", start.elapsed());
    Ok(())
}

#[allow(dead_code)]
fn parse_part_one(content: &str) -> i64 {
    content
        .lines()
        .filter_map(analyze_line)
        .filter(|(is_safe, _)| *is_safe)
        .count() as i64
}

#[allow(dead_code)]
fn parse_part_two(content: &str) -> i64 {
    let (safe_lines, unsafe_lines): (Vec<_>, Vec<_>) = content
        .lines()
        .filter_map(analyze_line)
        .partition(|(is_safe, _)| *is_safe);
    let mut safe_lines_count = safe_lines.len() as i64;

    for (_, line) in unsafe_lines {
        let is_safe_after_removal = (0..line.len()).any(|i| {
            let mut temp = line.clone();
            temp.remove(i);
            (is_monotonic(&temp, IS_INCREASING) || is_monotonic(&temp, IS_DECREASING))
                && is_valid_diff(&temp)
        });

        if is_safe_after_removal {
            safe_lines_count += 1
        }
    }

    safe_lines_count
}

fn analyze_line(line: &str) -> Option<(bool, Vec<i64>)> {
    let converted_vec: Vec<i64> = line
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    if converted_vec.len() == 0 {
        return None;
    }

    let is_safe = (is_monotonic(&converted_vec, IS_INCREASING)
        || is_monotonic(&converted_vec, IS_DECREASING))
        && is_valid_diff(&converted_vec);

    Some((is_safe, converted_vec))
}

fn is_valid_diff(data: &Vec<i64>) -> bool {
    data.par_windows(2).all(|w| {
        let diff = (w[0] - w[1]).abs();
        diff >= 1 && diff <= 3
    })
}

fn is_monotonic(data: &Vec<i64>, compare: fn(i64, i64) -> bool) -> bool {
    data.par_windows(2).all(|w| compare(w[0], w[1]))
}

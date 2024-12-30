use std::{collections::HashMap, fs::read_to_string, io::Error, time::Instant};

const FILE_PATH: &str = "../input.txt";
const SEPERATOR: &str = "   ";

fn main() -> Result<(), Error> {
    let start = Instant::now();
    let content = read_to_string(FILE_PATH)?;
    let sum = parse_part_two(&content);

    println!("Result: {:?}", sum);
    println!("Eslaped time: {:?}", start.elapsed());
    Ok(())
}

#[allow(dead_code)]
fn parse_part_one(content: &str) -> i64 {
    let mut left_arr: Vec<i64> = Vec::new();
    let mut right_arr: Vec<i64> = Vec::new();
    for line in content.lines() {
        if let Some((a_str, b_str)) = line.split_once(SEPERATOR) {
            if let (Ok(a), Ok(b)) = (a_str.parse::<i64>(), b_str.parse::<i64>()) {
                left_arr.push(a);
                right_arr.push(b);
            }
        }
    }

    left_arr.sort();
    right_arr.sort();

    let sum: i64 = left_arr
        .iter()
        .zip(right_arr.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    sum
}

#[allow(dead_code)]
fn parse_part_two(content: &str) -> i64 {
    let mut left_arr: Vec<i64> = Vec::new();
    let mut right_dict = HashMap::<i64, i64>::new();
    for line in content.lines() {
        if let Some((a_str, b_str)) = line.split_once(SEPERATOR) {
            if let (Ok(a), Ok(b)) = (a_str.parse::<i64>(), b_str.parse::<i64>()) {
                left_arr.push(a);
                right_dict.entry(b).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }

    let sum: i64 = left_arr
        .iter()
        .map(|i| i * right_dict.get(i).unwrap_or(&0))
        .sum();

    sum
}

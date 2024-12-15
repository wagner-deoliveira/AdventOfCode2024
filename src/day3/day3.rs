use crate::utils::utils::read_lines;
use regex::Regex;
use std::io::{self, Error};

#[derive(Debug)]
enum Operation {
    Multiply(i32, i32),
    Do,
    Dont,
}

fn parse_mul(capture: &str) -> Option<(i32, i32)> {
    let nums: Vec<i32> = capture[4..capture.len() - 1] // Remove "mul(" and ")"
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect();

    match nums[..] {
        [a, b] => Some((a, b)),
        _ => None,
    }
}

pub fn main() -> io::Result<()> {
    // Compile regex patterns once
    let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();

    let mut total_mul = 0;
    let mut should_multiply = true;

    for line in read_lines("../inputs/input3.txt")?
        .into_iter()
        .filter_map(|lines| Result::<String, Error>::ok(Ok(lines)))
    {
        // Find all operations in one pass
        let operations: Vec<(usize, Operation)> = re
            .find_iter(&line)
            .filter_map(|m| {
                let op = match m.as_str() {
                    "do()" => Some(Operation::Do),
                    "don't()" => Some(Operation::Dont),
                    mul if mul.starts_with("mul(") => {
                        parse_mul(mul).map(|(a, b)| Operation::Multiply(a, b))
                    }
                    _ => None,
                };
                op.map(|op| (m.start(), op))
            })
            .collect();

        // Process operations
        for (pos, op) in operations {
            match op {
                Operation::Dont => {
                    should_multiply = false;
                    println!("Found don't() at {}", pos);
                }
                Operation::Do => {
                    should_multiply = true;
                    println!("Found do() at {}", pos);
                }
                Operation::Multiply(a, b) if should_multiply => {
                    let result = a * b;
                    println!("Multiplying at {}: {} * {} = {}", pos, a, b, result);
                    total_mul += result;
                }
                Operation::Multiply(a, b) => {
                    println!("Skipping multiplication at {}: {} * {}", pos, a, b);
                }
            }
        }
    }

    println!("Final result: {}", total_mul);
    Ok(())
}

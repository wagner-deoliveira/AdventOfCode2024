use crate::utils::utils::read_lines;
use std::io;

pub fn main() -> io::Result<()> {
    // Part one: Find the number of valid lines

    let lines = read_lines("../inputs/input2.txt").unwrap();

    let max_diff: i32 = 3;
    let mut accumulated_diff: i32 = 0;

    'outer: for line in lines {
        let values: Vec<&str> = line.split_whitespace().collect();
        let line_numbers: Vec<i32> = values.iter().map(|&v| v.parse::<i32>().unwrap()).collect();

        // First check if the original line is valid
        if is_valid_sequence(&line_numbers)
            && line_numbers
                .windows(2)
                .all(|pair| (pair[1] - pair[0]).abs() <= max_diff)
        {
            println!("Originally valid line: {:?}", line_numbers);
            accumulated_diff += 1;
            continue;
        }

        // Part two: Check if the line can be made valid by removing any index
        let mut modified_numbers = line_numbers.clone();
        let lenght = modified_numbers.len();
        for i in 0..lenght {
            let removed_number = modified_numbers.remove(i);

            if is_valid_sequence(&modified_numbers)
                && modified_numbers
                    .windows(2)
                    .all(|pair| (pair[1] - pair[0]).abs() <= max_diff)
            {
                println!("Original invalid line: {:?}", line_numbers);
                println!(
                    "Can be made valid by removing index {}: {:?}",
                    i, modified_numbers
                );
                println!("All differences are <= {}\n", max_diff);
                accumulated_diff += 1;
                continue 'outer;
            }
            modified_numbers.insert(i, removed_number);
        }
    }

    println!(
        "Total number of valid lines (including fixed ones): {}",
        accumulated_diff
    );
    Ok(())
}

fn is_valid_sequence(numbers: &[i32]) -> bool {
    let is_increasing = numbers.windows(2).all(|pair| pair[1] > pair[0]);
    let is_decreasing = numbers.windows(2).all(|pair| pair[1] < pair[0]);
    is_increasing || is_decreasing
}

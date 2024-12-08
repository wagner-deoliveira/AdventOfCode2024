use crate::utils::utils::read_lines;
use std::io::BufRead;

pub fn main() {
    let file = read_lines("../inputs/input5.txt").expect("Failed to read input file");
    let split_index = file
        .iter()
        .position(|line| line.is_empty())
        .unwrap_or(file.len());

    let first_part: Vec<String> = file[..split_index].to_vec();
    let second_part: Vec<String> = file[split_index + 1..].to_vec();
    let mut middle_elements: Vec<i32> = vec![];

    for entry in &second_part {
        let values: Vec<&str> = entry.split(',').collect();
        let mut skip_entry = false;

        'outer: for (index, current_value) in values.iter().enumerate() {
            let first_part_vec = check_if_line_contains_value(&first_part, current_value);
            let first_part_vec_value: Vec<(&str, &str)> = first_part_vec
                .iter()
                .map(|el| {
                    let parts: Vec<&str> = el.split('|').collect();
                    (parts[0], parts[1])
                })
                .collect();

            // println!("First part vector: {:?}", first_part_vec_value);

            let vector_to_compare = &values[index + 1..];

            for &compare_value in vector_to_compare {
                if compare_value.is_empty() {
                    continue;
                }

                // Check if compare_value exists as second element in any tuple
                if first_part_vec_value
                    .iter()
                    .any(|(_, b)| b == &compare_value)
                {
                    // Check if the first element equals current_value
                    if !first_part_vec_value.iter().any(|(a, _)| a == current_value) {
                        skip_entry = true;
                        break 'outer;
                    }
                }
                // Check if compare_value exists as first element in any tuple
                else if first_part_vec_value
                    .iter()
                    .any(|(a, _)| a == &compare_value)
                {
                    skip_entry = true;
                    break 'outer;
                }
            }
        }

        if !skip_entry {
            let valid_entry: Vec<i32> = entry.split(',').into_iter().collect::<Vec<&str>>().iter().map(|&s| s.parse::<i32>().unwrap()).collect();
            let middle_index = valid_entry.len() / 2;
            middle_elements.push(valid_entry[middle_index]);
        }
    }
    let sum_middle_elements: i32 = middle_elements.iter().sum();
    println!("Sum of middle elements: {}", sum_middle_elements);
}

fn check_if_line_contains_value(line: &Vec<String>, value: &str) -> Vec<String> {
    line.iter()
        .filter(|line| line.contains(value))
        .cloned()
        .collect()
}

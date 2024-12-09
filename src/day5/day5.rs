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
        let (is_valid, problem_index) = is_valid_arrangement(&first_part, &values);
        let mut skip_entry = !is_valid;

        if skip_entry {
            let mut swapped_entry: Vec<&str> = values.clone();
            if let Some(start_index) = problem_index {
                // Start swapping from the problematic index
                for i in start_index..values.len() - 1 {
                    // println!("Index to swap {:?}", i);
                    // println!("Swapping entries in {:?}", swapped_entry);
                    swapped_entry.swap(i, i + 1);
                    // println!("Swapped entry string {:?}", swapped_entry);
                    let (is_valid_after_swap, _) =
                        is_valid_arrangement(&first_part, &swapped_entry);
                    if is_valid_after_swap {
                        skip_entry = false;
                        let valid_entry: Vec<i32> = swapped_entry
                            .iter()
                            .map(|&s| s.parse::<i32>().unwrap())
                            .collect();
                        // println!("Valid entry: {:?}", valid_entry);
                        let middle_index = swapped_entry.len() / 2;
                        middle_elements.push(valid_entry[middle_index]);
                        break;
                    }

                    if i+1 == values.len() - 1 {
                        // println!("Index to swap {:?} and swapping entries in {:?} plus value length {:?}", i, swapped_entry, values.len() - 1);
                        // println!("Starting index minus value length {:?}", start_index - values.len() - 1);
                        for i in (0..values.len() - 1).rev() {
                            // println!("Index to swap {:?}", i);
                            if i > start_index {
                                swapped_entry.swap(i, i - 1);
                                // println!("Swapped entry string {:?}", swapped_entry);
                                let (is_valid_after_swap, _) =
                                    is_valid_arrangement(&first_part, &swapped_entry);
                                if is_valid_after_swap {
                                    skip_entry = false;
                                    let valid_entry: Vec<i32> = swapped_entry
                                        .iter()
                                        .map(|&s| s.parse::<i32>().unwrap())
                                        .collect();
                                    // println!("Valid entry: {:?}", valid_entry);
                                    let middle_index = swapped_entry.len() / 2;
                                    middle_elements.push(valid_entry[middle_index]);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        if !skip_entry && problem_index.is_none() {
            // let valid_entry: Vec<i32> = values.iter().map(|&s| s.parse::<i32>().unwrap()).collect();
            // println!("Valid entry: {:?}", valid_entry);
            // let middle_index = valid_entry.len() / 2;
            // middle_elements.push(valid_entry[middle_index]);
        }
    }

    println!("Middle elements: {:?}", middle_elements);
    let sum_middle_elements: i32 = middle_elements.iter().sum();
    println!("Sum of middle elements: {}", sum_middle_elements);
}

fn check_if_line_contains_value(line: &Vec<String>, value: &str) -> Vec<String> {
    line.iter()
        .filter(|line| line.contains(value))
        .cloned()
        .collect()
}

fn is_valid_arrangement(first_part: &Vec<String>, values: &[&str]) -> (bool, Option<usize>) {
    for (index, current_value) in values.iter().enumerate() {
        let first_part_vec = check_if_line_contains_value(first_part, current_value);
        let first_part_vec_value: Vec<(&str, &str)> = first_part_vec
            .iter()
            .map(|el| {
                let parts: Vec<&str> = el.split('|').collect();
                (parts[0], parts[1])
            })
            .collect();

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
                    return (false, Some(index));
                }
            }
            // Check if compare_value exists as first element in any tuple
            else if first_part_vec_value
                .iter()
                .any(|(a, _)| a == &compare_value)
            {
                return (false, Some(index));
            }
        }
    }
    (true, None)
}

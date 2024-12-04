use crate::utils::utils::read_lines;
use std::collections::HashMap;
use std::io;

pub fn main() -> io::Result<()> {
    // Part one: Summ the distances between list1 and list2

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let lines = read_lines("../inputs/input1.txt").expect("Something went wrong");

    for line in lines {
        let values: Vec<&str> = line.split_whitespace().collect();
        if values.len() == 2 {
            if let (Ok(val1), Ok(val2)) = (values[0].parse::<i32>(), values[1].parse::<i32>()) {
                list1.push(val1);
                list2.push(val2);
            }
        }
    }

    list1.sort();
    list2.sort();
    let calculated_distance: Vec<i32> = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| ((*a as i32) - (*b as i32)).abs())
        .collect();
    println!("calculated_distance: {:?}", calculated_distance);

    let total_distance: i32 = calculated_distance.iter().sum();
    println!("Sum of all distances: {}", total_distance);

    // Part two: Create frequency map for list2
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    for &num in &list2 {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    // println!("Frequencies in list2: {:?}", frequency_map);

    // Calculate products for matching numbers
    let mut results: Vec<(i32, i32, i32)> = Vec::new(); // (number, frequency, product)
    for &num in &list1 {
        if let Some(&freq) = frequency_map.get(&num) {
            let product = num * freq;
            results.push((num, freq, product));
        }
    }

    // Print results
    // println!("\nMatching numbers and their products:");
    // for (num, freq, product) in &results {
    //     println!("Number: {}, Frequency: {}, Product: {}", num, freq, product);
    // }

    println!(
        "Sum of the products: {}",
        results.iter().map(|(_, _, product)| product).sum::<i32>()
    );
    Ok(())
}

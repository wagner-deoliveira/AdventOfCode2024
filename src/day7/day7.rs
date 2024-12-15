use crate::utils::utils::read_lines;

fn concatenate(a: i64, b: i64) -> i64 {
    let b_str = b.to_string();
    let result = format!("{}{}", a, b_str);
    result.parse().unwrap()
}

fn evaluate(nums: &[i64], ops: &[char]) -> i64 {
    let mut result = nums[0];
    for i in 0..ops.len() {
        match ops[i] {
            '+' => result += nums[i + 1],
            '*' => result *= nums[i + 1],
            '|' => result = concatenate(result, nums[i + 1]),
            _ => panic!("Invalid operator")
        }
    }
    result
}

fn can_make_value(target: i64, nums: &[i64]) -> bool {
    if nums.len() == 1 {
        return nums[0] == target;
    }

    let ops_needed = nums.len() - 1;
    let total_combinations = 1 << (2 * ops_needed); // 3^n combinations (3 operators)

    for i in 0..total_combinations {
        let mut ops = Vec::new();
        for j in 0..ops_needed {
            let op_code = (i >> (2 * j)) & 3;
            match op_code {
                0 => ops.push('+'),
                1 => ops.push('*'),
                2 => ops.push('|'),
                _ => continue
            }
        }

        // Skip if we have the wrong number of operators
        if ops.len() != ops_needed {
            continue;
        }

        if let Ok(result) = std::panic::catch_unwind(|| evaluate(nums, &ops)) {
            if result == target {
                return true;
            }
        }
    }
    false
}

pub fn main() {
    let file = read_lines("../inputs/input7.txt").expect("Failed to read input file");
    let mut sum = 0;

    for line in file {
        let parts: Vec<&str> = line.split(": ").collect();
        let target: i64 = parts[0].parse().unwrap();
        let nums: Vec<i64> = parts[1].split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        if can_make_value(target, &nums) {
            sum += target;
        }
    }

    println!("Sum of valid test values: {}", sum);
}
use crate::utils::utils::read_lines;

fn evaluate(nums: &[i64], ops: &[char]) -> i64 {
    let mut result = nums[0];
    for i in 0..ops.len() {
        match ops[i] {
            '+' => result += nums[i + 1],
            '*' => result *= nums[i + 1],
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
    let total_combinations = 1 << ops_needed; // 2^n combinations

    for i in 0..total_combinations {
        let mut ops = Vec::new();
        for j in 0..ops_needed {
            if (i & (1 << j)) == 0 {
                ops.push('+');
            } else {
                ops.push('*');
            }
        }
        if evaluate(nums, &ops) == target {
            return true;
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
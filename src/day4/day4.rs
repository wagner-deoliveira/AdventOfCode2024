use crate::utils::utils::read_file;

pub fn main() {
    let file = read_file("../inputs/input4.txt").expect("Something went wrong");
    let horizontal = check_horizontal(&file);
    let vertical = check_vertical(&file);
    let diagonal = check_diagonal(&file);

    println!("Horizontal matches: {}", horizontal);
    println!("Vertical matches: {}", vertical);
    println!("Diagonal matches: {}", diagonal);
    println!("Total matches: {}", horizontal + vertical + diagonal);
}

fn check_horizontal(file: &str) -> i32 {
    let mut total = 0;
    for (line_num, line) in file.lines().enumerate() {
        total += count_patterns(line, &format!("Horizontal line {}", line_num + 1));
    }
    total
}

fn check_vertical(file: &str) -> i32 {
    let lines: Vec<&str> = file.lines().collect();
    let mut total = 0;

    for col in 0..lines[0].len() {
        let vertical: String = (0..lines.len())
            .filter_map(|row| lines.get(row)?.chars().nth(col))
            .collect();
        total += count_patterns(&vertical, &format!("Vertical column {}", col + 1));
    }
    total
}

fn check_diagonal(file: &str) -> i32 {
    let lines: Vec<&str> = file.lines().collect();
    let reversed_lines: Vec<&str> = lines.clone().into_iter().rev().collect();

    count_patterns_in_diagonal(lines.clone());
    count_patterns_in_diagonal(reversed_lines.clone());

    let total = count_patterns_in_diagonal(lines) + count_patterns_in_diagonal(reversed_lines);
    println!("Total matches: {}", total);
    total
}

fn count_patterns(text: &str, prefix: &str) -> i32 {
    let mut count = 0;

    for (pos, _) in text.match_indices("XMAS") {
        println!("{} - Found XMAS at position {} in: {}", prefix, pos, text);
        count += 1;
    }
    for (pos, _) in text.match_indices("SAMX") {
        println!("{} - Found SAMX at position {} in: {}", prefix, pos, text);
        count += 1;
    }

    if count > 0 {
        println!("Total patterns in this diagonal: {}\n", count);
    }

    count
}

fn count_patterns_in_diagonal(lines: Vec<&str>) -> i32 {
    let cols = lines[0].len();
    let rows = lines.len();
    let mut total = 0;
    let min_value = 4;

    for line in &lines {
        println!("Reversed line: {}", line);
    }

    for mut col in 0..cols {
        let mut diagonal = String::new();
        let mut row = 0;
        while row < rows && col < cols {
            diagonal.push(lines[row].chars().nth(col).unwrap());
            col += 1;
            row += 1;
        }
        if diagonal.len() >= min_value {
            total += count_patterns(&diagonal, &format!("Forward diagonal {} starting at col {}", diagonal, rows - row));
        }
    }

    for mut row in 1..rows {
        let mut diagonal = String::new();
        let mut col = 0;
        while row < rows && col < cols {
            diagonal.push(lines[row].chars().nth(col).unwrap());
            col += 1;
            row += 1;
        }
        if diagonal.len() >= min_value {
            total += count_patterns(&diagonal, &format!("Forward diagonal {} starting at row {}", diagonal, cols - col));
        }
    }
    total
}
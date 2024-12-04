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
    let cols = lines[0].len();
    let rows = lines.len();
    let mut total = 0;

    // Forward diagonals (top-left to bottom-right)
    // Starting from top row
    for start_col in 0..cols {
        let mut diagonal = String::new();
        let mut row = 0;
        let mut col = start_col;

        while row < rows && col < cols {
            diagonal.push(lines[row].chars().nth(col).unwrap());
            row += 1;
            col += 1;
        }
        if diagonal.len() >= 4 {
            total += count_patterns(&diagonal, &format!("Forward diagonal from top (0, {})", start_col));
        }
    }

    // Forward diagonals starting from left column (excluding first row)
    for start_row in 1..rows {
        let mut diagonal = String::new();
        let mut row = start_row;
        let mut col = 0;

        while row < rows && col < cols {
            diagonal.push(lines[row].chars().nth(col).unwrap());
            row += 1;
            col += 1;
        }
        if diagonal.len() >= 4 {
            total += count_patterns(&diagonal, &format!("Forward diagonal from left ({}, 0)", start_row));
        }
    }

    // Backward diagonals (top-right to bottom-left)
    // Starting from top row
    for start_col in (0..cols).rev() {
        let mut diagonal = String::new();
        let mut row = 0;
        let mut col = start_col;

        while row < rows && col < cols && col >= 0 {
            diagonal.push(lines[row].chars().nth(col).unwrap());
            row += 1;
            if col > 0 { col -= 1; }
        }
        if diagonal.len() >= 4 {
            total += count_patterns(&diagonal, &format!("Backward diagonal from top (0, {})", start_col));
        }
    }

    // Backward diagonals starting from right column (excluding first row)
    for start_row in 1..rows {
        let mut diagonal = String::new();
        let mut row = start_row;
        let mut col = cols - 1;

        while row < rows && col >= 0 {
            diagonal.push(lines[row].chars().nth(col).unwrap());
            row += 1;
            if col > 0 { col -= 1; }
        }
        if diagonal.len() >= 4 {
            total += count_patterns(&diagonal, &format!("Backward diagonal from right ({}, {})", start_row, cols-1));
        }
    }

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

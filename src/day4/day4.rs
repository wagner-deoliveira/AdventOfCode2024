use crate::utils::utils::read_file;

pub fn main() {
    let file = read_file("../inputs/input4.txt").expect("Something went wrong");
    let mut total = 0;

    for line in file.lines() {
        let horizontal = check_horizontal(line);
        total += horizontal;
    }

    let vertical = check_vertical(String::from(&file));
    let diagonal = check_diagonal(file);

    println!("Total matches: {}", total + vertical + diagonal);
}

fn check_horizontal(line: &str) -> i32 {
    let mut count = 0;
    let xmas_count = line.matches("XMAS").count();
    let samx_count = line.matches("SAMX").count();

    if xmas_count > 0 || samx_count > 0 {
        count += 1;
    }
    count
}

fn check_vertical(file: String) -> i32 {
    let lines: Vec<&str> = file.lines().collect();

    let cols = lines[0].len();
    let rows = lines.len();
    let mut count = 0;

    for col in 0..cols {
        let vertical: String = (0..rows)
            .filter_map(|row| lines.get(row)?.chars().nth(col))
            .collect();

        if vertical.contains("XMAS") || vertical.contains("SAMX") {
            count += 1;
        }
    }

    count
}

fn check_diagonal(file: String) -> i32 {
    let lines: Vec<&str> = file.lines().collect();
    let cols = lines[0].len();
    let rows = lines.len();
    let mut count = 0;
    let mut diagonals: Vec<String> = Vec::new();

    // Only check main diagonals and their parallels
    // Top row starting points (left to right)
    for start_col in 0..cols {
        // Forward diagonal (top-right)
        let mut forward = String::new();
        let mut row = 0;
        let mut col = start_col;

        while row < rows && col < cols {
            if let Some(ch) = lines[row].chars().nth(col) {
                forward.push(ch);
            }
            row += 1;
            col += 1;
        }

        if forward.len() >= 4 {
            diagonals.push(forward);
        }

        // Reverse diagonal (top-left)
        let mut reverse = String::new();
        row = 0;
        col = start_col;

        while row < rows && col >= 0 {
            if let Some(ch) = lines[row].chars().nth(col) {
                reverse.push(ch);
            }
            row += 1;
            if col > 0 { col -= 1; } else { break; }
        }

        if reverse.len() >= 4 {
            diagonals.push(reverse);
        }
    }

    // Left column starting points (top to bottom, excluding first row)
    for start_row in 1..rows {
        // Forward diagonal
        let mut forward = String::new();
        let mut row = start_row;
        let mut col = 0;

        while row < rows && col < cols {
            if let Some(ch) = lines[row].chars().nth(col) {
                forward.push(ch);
            }
            row += 1;
            col += 1;
        }

        if forward.len() >= 4 {
            diagonals.push(forward);
        }

        // Reverse diagonal from right column
        let mut reverse = String::new();
        row = start_row;
        col = cols - 1;

        while row < rows && col >= 0 {
            if let Some(ch) = lines[row].chars().nth(col) {
                reverse.push(ch);
            }
            row += 1;
            if col > 0 { col -= 1; } else { break; }
        }

        if reverse.len() >= 4 {
            diagonals.push(reverse);
        }
    }

    println!("Diagonal strings: {:?}", diagonals);

    // Check patterns in diagonals
    for diagonal in &diagonals {
        if diagonal.contains("XMAS") || diagonal.contains("SAMX") {
            count += 1;
            println!("Found pattern in: {}", diagonal);
        }
    }

    count
}

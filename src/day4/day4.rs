use crate::utils::utils::read_file;

pub fn main() {
    let file = read_file("../inputs/input4.txt").expect("Failed to read input file");
    // let horizontal = check_horizontal(&file);
    // let vertical = check_vertical(&file);
    // let diagonal = check_diagonal(&file);

    // println!("Horizontal matches: {}", horizontal);
    // println!("Vertical matches: {}", vertical);
    // println!("Diagonal matches: {}", diagonal);
    // println!("Total matches: {}", horizontal + vertical + diagonal);

    let mas_cross_patterns = check_cross_pattern(&file);
    println!("Cross pattern matches: {}", mas_cross_patterns);
}

fn check_horizontal(file: &str) -> i32 {
    file.lines()
        .enumerate()
        .map(|(line_num, line)| {
            count_xmas_patterns(line, &format!("Horizontal line {}", line_num + 1))
        })
        .sum()
}

fn check_vertical(file: &str) -> i32 {
    let lines: Vec<&str> = file.lines().collect();
    let width = lines[0].len();

    (0..width)
        .map(|col| {
            let vertical: String = lines
                .iter()
                .filter_map(|line| line.chars().nth(col))
                .collect();
            count_xmas_patterns(&vertical, &format!("Vertical column {}", col + 1))
        })
        .sum()
}

fn check_diagonal(file: &str) -> i32 {
    let lines: Vec<&str> = file.lines().collect();
    let forward_count = count_patterns_in_diagonal(&lines);
    let backward_count =
        count_patterns_in_diagonal(&lines.iter().rev().cloned().collect::<Vec<_>>());

    forward_count + backward_count
}

fn count_xmas_patterns(text: &str, prefix: &str) -> i32 {
    let patterns = ["XMAS", "SAMX"];
    let count: i32 = patterns
        .iter()
        .map(|&pattern| {
            text.match_indices(pattern)
                .map(|(pos, _)| {
                    println!(
                        "{} - Found {} at position {} in: {}",
                        prefix, pattern, pos, text
                    );
                    1
                })
                .sum::<i32>()
        })
        .sum();

    if count > 0 {
        println!("Total patterns in this line: {}\n", count);
    }
    count
}

fn count_patterns_in_diagonal(lines: &[&str]) -> i32 {
    let cols = lines[0].len();
    let rows = lines.len();
    let min_length = 4;
    let mut total = 0;

    // Diagonals starting from first row
    for start_col in 0..cols {
        let diagonal: String = (0..rows.min(cols - start_col))
            .filter_map(|offset| lines.get(offset)?.chars().nth(start_col + offset))
            .collect();

        if diagonal.len() >= min_length {
            total += count_xmas_patterns(
                &diagonal,
                &format!("Diagonal starting at col {}", start_col),
            );
        }
    }

    // Diagonals starting from first column (excluding first row)
    for start_row in 1..rows {
        let diagonal: String = (0..rows.min(cols - start_row))
            .filter_map(|offset| lines.get(start_row + offset)?.chars().nth(offset))
            .collect();

        if diagonal.len() >= min_length {
            total += count_xmas_patterns(
                &diagonal,
                &format!("Diagonal starting at row {}", start_row),
            );
        }
    }

    total
}

fn check_cross_pattern(file: &str) -> i32 {
    let grid: Vec<Vec<char>> = file.lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut grid_clone: Vec<Vec<char>> = vec![];

    let mut count = 0;
    let mut start_row = 0;

    // Iterate over the grid
    for row in 0..grid.len().saturating_sub(2) {
        let mut start_col = 0;
        for col in 0..grid[0].len().saturating_sub(2) {
            // Get the 3x3 window
            let top = [grid[start_row][start_col], grid[start_row][start_col + 1], grid[start_row][start_col + 2]];
            let middle = [grid[start_row + 1][start_col], grid[start_row + 1][start_col + 1], grid[start_row + 1][start_col + 2]];
            let bottom = [grid[start_row + 2][start_col], grid[start_row + 2][start_col + 1], grid[start_row + 2][start_col + 2]];

            let has_valid_top = check_ms_pattern(&top);
            let has_valid_middle = check_middle_pattern(&middle);
            let has_valid_bottom = check_ms_pattern(&bottom);

            if has_valid_top && has_valid_middle && has_valid_bottom {
                // check if the pattern is valid MAS string
                let valid_mas_string = format!("{}{}{}", top[0], middle[1], bottom[2]);
                let valid_mas_string2 = format!("{}{}{}", top[2], middle[1], bottom[0]);
                if (valid_mas_string == "MAS" || valid_mas_string == "SAM") && (valid_mas_string2 == "MAS" || valid_mas_string2 == "SAM") {
                    println!("Valid MAS string found at position ({}, {})", row, col);
                    count += 1;
                }
            }
            start_col += 1;
            grid_clone = vec![Vec::from(top), Vec::from(middle), Vec::from(bottom)];
            // println!("Grid clone: {:?}", grid_clone);
        }
        start_row += 1;
    }
    count
}

fn check_ms_pattern(chars: &[char]) -> bool {
    (chars[0] == 'M' && chars[2] == 'S')
        || (chars[0] == 'S' && chars[2] == 'M')
        || (chars[0] == 'M' && chars[2] == 'M')
        || (chars[0] == 'S' && chars[2] == 'S')
}

fn check_middle_pattern(chars: &[char]) -> bool {
    chars[1] == 'S' || chars[1] == 'A'
}
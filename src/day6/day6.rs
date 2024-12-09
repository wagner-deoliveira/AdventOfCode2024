use crate::utils::utils::read_file;

type Position = (usize, usize);

enum Direction {
    Up,
    Right,
    Left,
    Down,
}

pub fn main() {
    let file = read_file("../inputs/input6.txt").expect("Failed to read input file");
    let lines: Vec<&str> = file.lines().collect();
    let matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let find_guard_position = check_guard_position(&matrix);
    println!("Guard position: {:?}", find_guard_position);

    if find_guard_position.is_some() {
        println!("Guard found");
        let (line_num, guard_position) = find_guard_position.unwrap();

        return;
    }
    // println!("Matrix length: {:?}", matrix.len());
}

fn check_guard_position(matrix: &Vec<Vec<char>>) -> Option<Position> {
    matrix
        .iter()
        .enumerate() // Add enumerate to get line index
        .find(|(_, line)| line.iter().any(|&c| c == '^'))
        .map(|(line_num, line)| (line_num, line.iter().position(|&c| c == '^').unwrap()))
}

fn check_direction(direction: &str) -> Option<Direction> {
    let direction = match direction.as_str() {
       "^" => Direction::Up,
       ">" => Direction::Right,
       "<" => Direction::Left,
       "v" => Direction::Down,
       _ => return None,
    };
    Some(direction)
}
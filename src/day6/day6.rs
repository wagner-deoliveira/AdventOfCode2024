use crate::utils::utils::read_file;
use std::cmp::PartialEq;

type Position = (usize, usize);

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Up, Direction::Up) => true,
            (Direction::Right, Direction::Right) => true,
            (Direction::Left, Direction::Left) => true,
            (Direction::Down, Direction::Down) => true,
            _ => false,
        }
    }
}
use std::collections::HashSet;

pub fn main() {
    let file = read_file("../inputs/input6.txt").expect("Failed to read input file");
    let lines: Vec<&str> = file.lines().collect();
    let matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let find_guard_position = check_guard_position(&matrix);
    let mut visited: HashSet<Position> = HashSet::new();

    if let Some((mut line_num, mut guard_position)) = find_guard_position {
        println!("Guard starts at: ({}, {})", line_num, guard_position);
        visited.insert((line_num, guard_position));
        let mut current_direction = check_direction(&matrix[line_num][guard_position]).unwrap();
        let (rows, cols) = (matrix.len(), matrix[0].len());

        loop {
            let steps = match current_direction {
                Direction::Up => count_steps_up(&matrix, (line_num, guard_position)),
                Direction::Right => count_steps_right(&matrix, (line_num, guard_position), cols),
                Direction::Down => count_steps_down(&matrix, (line_num, guard_position), rows),
                Direction::Left => count_steps_left(&matrix, (line_num, guard_position)),
            };

            println!("Moving {:?} for {} steps", current_direction, steps);

            // Add all positions in the path to visited set
            for i in 1..=steps {
                let new_pos = match current_direction {
                    Direction::Up => (line_num - i, guard_position),
                    Direction::Right => (line_num, guard_position + i),
                    Direction::Down => (line_num + i, guard_position),
                    Direction::Left => (line_num, guard_position - i),
                };
                visited.insert(new_pos);
            }

            // Update position based on direction and steps
            match current_direction {
                Direction::Up => line_num -= steps,
                Direction::Right => guard_position += steps,
                Direction::Down => line_num += steps,
                Direction::Left => guard_position -= steps,
            };

            println!("New position: ({}, {})", line_num, guard_position);

            // Check if we've reached the boundary
            if line_num == 0 || line_num == rows - 1 ||
               guard_position == 0 || guard_position == cols - 1 {
                break;
            }

            // Rotate and continue
            current_direction = rotate_90_degrees(current_direction);
            println!("Rotating to {:?}", current_direction);
        }

        println!("Total distinct positions visited: {}", visited.len());
    }
}

fn count_steps_up(matrix: &Vec<Vec<char>>, pos: Position) -> usize {
    let (current_y, current_x) = pos;
    let mut steps = 0;
    for y in (0..current_y).rev() {
        if matrix[y][current_x] == '#' {
            break;
        }
        steps += 1;
    }
    steps
}

fn count_steps_right(matrix: &Vec<Vec<char>>, pos: Position, cols: usize) -> usize {
    let (current_y, current_x) = pos;
    let mut steps = 0;
    for x in (current_x + 1)..cols {
        if matrix[current_y][x] == '#' {
            break;
        }
        steps += 1;
    }
    steps
}

fn count_steps_down(matrix: &Vec<Vec<char>>, pos: Position, rows: usize) -> usize {
    let (current_y, current_x) = pos;
    let mut steps = 0;
    for y in (current_y + 1)..rows {
        if matrix[y][current_x] == '#' {
            break;
        }
        steps += 1;
    }
    steps
}

fn count_steps_left(matrix: &Vec<Vec<char>>, pos: Position) -> usize {
    let (current_y, current_x) = pos;
    let mut steps = 0;
    for x in (0..current_x).rev() {
        if matrix[current_y][x] == '#' {
            break;
        }
        steps += 1;
    }
    steps
}

fn check_guard_position(matrix: &Vec<Vec<char>>) -> Option<Position> {
    matrix
        .iter()
        .enumerate() // Add enumerate to get line index
        .find(|(_, line)| line.iter().any(|&c| c == '^'))
        .map(|(line_num, line)| (line_num, line.iter().position(|&c| c == '^').unwrap()))
}

fn check_direction(direction: &char) -> Option<Direction> {
    let direction = match direction {
        '^' => Direction::Up,
        '>' => Direction::Right,
        '<' => Direction::Left,
        'v' => Direction::Down,
        _ => return None,
    };
    Some(direction)
}

fn rotate_90_degrees(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

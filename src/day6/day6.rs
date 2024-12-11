use crate::utils::utils::read_file;
use std::cmp::PartialEq;
use std::collections::HashSet;

type Position = (usize, usize);

#[derive(Debug, Clone, Copy)]
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

pub fn main() {
    let file = read_file("../inputs/input6.txt").expect("Failed to read input file");
    let lines: Vec<&str> = file.lines().collect();
    let matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let loop_positions = find_loop_positions(&matrix);
    println!("Number of possible obstruction positions: {}", loop_positions.len());

/*    let find_guard_position = check_guard_position(&matrix);
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
*/
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

fn find_loop_positions(matrix: &Vec<Vec<char>>) -> HashSet<Position> {
    let mut loop_positions = HashSet::new();
    let original_path = get_guard_path(matrix);

    // Get positions adjacent to the original path
    let candidates = get_adjacent_positions(&original_path, matrix);

    for &pos in &candidates {
        if matrix[pos.0][pos.1] == '.' {  // Only try empty positions
            if would_create_loop(matrix, pos, &original_path) {
                loop_positions.insert(pos);
            }
        }
    }
    loop_positions
}

fn get_adjacent_positions(path: &HashSet<Position>, matrix: &Vec<Vec<char>>) -> HashSet<Position> {
    let mut adjacent = HashSet::new();
    let (rows, cols) = (matrix.len(), matrix[0].len());

    for &(y, x) in path {
        // Check all adjacent positions
        for &(dy, dx) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_y = y as i32 + dy;
            let new_x = x as i32 + dx;

            if new_y >= 0 && new_y < rows as i32 &&
                new_x >= 0 && new_x < cols as i32 {
                adjacent.insert((new_y as usize, new_x as usize));
            }
        }
    }

    // Remove positions that are already walls or the guard's path
    adjacent.retain(|pos| !path.contains(pos) && matrix[pos.0][pos.1] != '#');
    adjacent
}

fn would_create_loop(matrix: &Vec<Vec<char>>, obstruction: Position, original_path: &HashSet<Position>) -> bool {
    let mut visited = HashSet::new();
    let mut current_pos = check_guard_position(matrix).unwrap();
    let mut current_direction = check_direction(&matrix[current_pos.0][current_pos.1]).unwrap();
    let (rows, cols) = (matrix.len(), matrix[0].len());

    visited.insert(current_pos);

    loop {
        let (line_num, guard_position) = current_pos;
        let mut steps = match current_direction {
            Direction::Up => count_steps_up(matrix, current_pos),
            Direction::Right => count_steps_right(matrix, current_pos, cols),
            Direction::Down => count_steps_down(matrix, current_pos, rows),
            Direction::Left => count_steps_left(matrix, current_pos),
        };

        // Check if obstruction blocks the path
        if would_hit_obstruction(current_pos, current_direction, obstruction, steps) {
            steps = distance_to_obstruction(current_pos, current_direction, obstruction);
        }

        // If we revisit a position with the same direction, it's a loop
        let key = (current_pos, current_direction);
        if visited.contains(&key.0) {
            return true;
        }
        visited.insert(key.0);

        if steps == 0 {  // Hit a wall immediately
            return false;
        }

        // Update position
        current_pos = match current_direction {
            Direction::Up => (line_num - steps, guard_position),
            Direction::Right => (line_num, guard_position + steps),
            Direction::Down => (line_num + steps, guard_position),
            Direction::Left => (line_num, guard_position - steps),
        };

        if current_pos.0 == 0 || current_pos.0 == rows - 1 ||
            current_pos.1 == 0 || current_pos.1 == cols - 1 {
            return false;
        }

        current_direction = rotate_90_degrees(current_direction);
    }
}

fn would_hit_obstruction(pos: Position, dir: Direction, obstruction: Position, steps: usize) -> bool {
    match dir {
        Direction::Up => obstruction.1 == pos.1 && obstruction.0 < pos.0 && obstruction.0 >= pos.0 - steps,
        Direction::Right => obstruction.0 == pos.0 && obstruction.1 > pos.1 && obstruction.1 <= pos.1 + steps,
        Direction::Down => obstruction.1 == pos.1 && obstruction.0 > pos.0 && obstruction.0 <= pos.0 + steps,
        Direction::Left => obstruction.0 == pos.0 && obstruction.1 < pos.1 && obstruction.1 >= pos.1 - steps,
    }
}

fn distance_to_obstruction(pos: Position, dir: Direction, obstruction: Position) -> usize {
    match dir {
        Direction::Up => pos.0 - obstruction.0 - 1,
        Direction::Right => obstruction.1 - pos.1 - 1,
        Direction::Down => obstruction.0 - pos.0 - 1,
        Direction::Left => pos.1 - obstruction.1 - 1,
    }
}

fn get_guard_path(matrix: &Vec<Vec<char>>) -> HashSet<Position> {
    let mut visited = HashSet::new();
    let mut current_pos = check_guard_position(matrix).unwrap();
    let mut current_direction = check_direction(&matrix[current_pos.0][current_pos.1]).unwrap();
    let (rows, cols) = (matrix.len(), matrix[0].len());

    visited.insert(current_pos);

    loop {
        let (line_num, guard_position) = current_pos;
        let steps = match current_direction {
            Direction::Up => count_steps_up(&matrix, current_pos),
            Direction::Right => count_steps_right(&matrix, current_pos, cols),
            Direction::Down => count_steps_down(&matrix, current_pos, rows),
            Direction::Left => count_steps_left(&matrix, current_pos),
        };

        // Add path positions to visited set
        for i in 1..=steps {
            let new_pos = match current_direction {
                Direction::Up => (line_num - i, guard_position),
                Direction::Right => (line_num, guard_position + i),
                Direction::Down => (line_num + i, guard_position),
                Direction::Left => (line_num, guard_position - i),
            };
            visited.insert(new_pos);
        }

        // Update position
        current_pos = match current_direction {
            Direction::Up => (line_num - steps, guard_position),
            Direction::Right => (line_num, guard_position + steps),
            Direction::Down => (line_num + steps, guard_position),
            Direction::Left => (line_num, guard_position - steps),
        };

        if current_pos.0 == 0 || current_pos.0 == rows - 1 ||
           current_pos.1 == 0 || current_pos.1 == cols - 1 {
            break;
        }

        current_direction = rotate_90_degrees(current_direction);
    }

    visited
}

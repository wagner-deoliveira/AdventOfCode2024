use crate::utils::utils::read_lines;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

pub fn main() {
    let lines = read_lines("../inputs/input8.txt").expect("Failed to read input file");
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

    // Parse input map
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennas.entry(ch)
                    .or_default()
                    .push(Point {
                        row: row as i32,
                        col: col as i32
                    });
            }
        }
    }

    let rows = lines.len() as i32;
    let cols = lines[0].len() as i32;
    let mut antinodes: HashSet<Point> = HashSet::new();

    // For each frequency
    for (_, positions) in antennas.iter() {
        // Check all pairs of antennas with same frequency
        for i in 0..positions.len() {
            for j in (i+1)..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];

                // Calculate vector between antennas
                let dx = p2.col - p1.col;
                let dy = p2.row - p1.row;

                // Calculate antinode positions (1/3 and 2/3 points)
                let antinode1 = Point {
                    row: p1.row + dy/3,
                    col: p1.col + dx/3
                };
                let antinode2 = Point {
                    row: p1.row + 2*dy/3,
                    col: p1.col + 2*dx/3
                };

                // Add antinodes if within bounds
                if in_bounds(antinode1, rows, cols) {
                    antinodes.insert(antinode1);
                }
                if in_bounds(antinode2, rows, cols) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    println!("Number of antinodes: {}", antinodes.len());
}

fn in_bounds(p: Point, rows: i32, cols: i32) -> bool {
    p.row >= 0 && p.row < rows && p.col >= 0 && p.col < cols
}
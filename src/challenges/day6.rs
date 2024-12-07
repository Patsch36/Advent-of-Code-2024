use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::panic;

use crate::utils::readfile;

pub fn part1() {
    let initial_grid = load_grid("puzzles/puzzled6p1.txt");
    let mut current_grid = initial_grid.clone();

    loop {
        match panic::catch_unwind(|| move_guard(&current_grid, true)) {
            Ok(updated_grid) => {
                current_grid = updated_grid;
            }
            Err(_) => {
                // display_grid(&current_grid);
                let count_x = count_char(&current_grid, 'X') + 1;
                display_grid(&current_grid);
                println!(
                    "The guard is leaving the grid at {:?}",
                    find_guard_position(&current_grid).unwrap()
                );
                println!("Number of X: {}", count_x);
                break;
            }
        }
    }
}
use rayon::prelude::*; // Add this import for parallel iteration

pub fn part2() {
    let data = readfile("puzzles/puzzled6p1.txt");

    let _map: Vec<Vec<char>> = data
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    let (_, visited, visited_entry) = patrol(&_map, None, None);

    let guard_pos = get_guard_pos(&_map);
    let mut visited = visited.clone();
    visited.remove(&guard_pos); // Avoid the guard's position

    let _map_dump = serde_json::to_string(&_map).unwrap(); // Faster serialization with JSON

    let loop_count: usize = visited
        .par_iter()
        .map(|&(vi, vj)| {
            let mut _map_copy: Value = serde_json::from_str(&_map_dump.as_str()).unwrap();
            if let Some(map_row) = _map_copy.get_mut(vi) {
                if let Some(cell) = map_row.get_mut(vj) {
                    *cell = Value::String("#".to_string());
                }
            }

            let (pos, idx) = visited_entry[&(vi, vj)];
            let map_copy: Vec<Vec<char>> = _map_copy
                .as_array()
                .unwrap()
                .iter()
                .map(|row| {
                    row.as_array()
                        .unwrap()
                        .iter()
                        .map(|c| c.as_str().unwrap().chars().next().unwrap())
                        .collect()
                })
                .collect();
            let (_, visited_copy, _) = patrol(&map_copy, Some(pos), Some(idx));

            if visited_copy.len() == 0 {
                1
            } else {
                0
            }
        })
        .sum();

    println!(
        "Number of positions that can break the loop: {}",
        loop_count
    );
}

fn load_grid(file_path: &str) -> Vec<Vec<char>> {
    readfile(file_path)
        .iter()
        .map(|row| row.chars().collect())
        .collect()
}

fn get_direction_offset(direction: char) -> (i32, i32) {
    match direction {
        '^' => (-1, 0),
        '<' => (0, -1),
        'v' => (1, 0),
        '>' => (0, 1),
        _ => panic!("Invalid direction"),
    }
}

fn rotate_direction(clockwise: char) -> char {
    match clockwise {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("Invalid direction"),
    }
}

fn find_guard_position(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    grid.iter().enumerate().find_map(|(row_idx, row)| {
        row.iter()
            .position(|&cell| matches!(cell, '^' | '<' | 'v' | '>'))
            .map(|col_idx| (row_idx, col_idx))
    })
}

fn is_outside_grid(position: (i32, i32), dimensions: (i32, i32)) -> bool {
    let (x, y) = position;
    let (rows, cols) = dimensions;
    x < 0 || y < 0 || x >= rows || y >= cols
}

fn is_obstacle_ahead(position: (usize, usize), grid: &[Vec<char>]) -> bool {
    let (x, y) = position;
    let movement = get_direction_offset(grid[x][y]);
    let next_position = (x as i32 + movement.0, y as i32 + movement.1);

    if is_outside_grid(next_position, (grid.len() as i32, grid[0].len() as i32)) {
        return false;
    }

    let (next_x, next_y) = (next_position.0 as usize, next_position.1 as usize);
    grid[next_x][next_y] == '#' || grid[next_x][next_y] == '0'
}

fn get_loop_marker(grid: &[Vec<char>]) -> char {
    let guard_pos = find_guard_position(grid).expect("No guard found on the grid");
    if grid[guard_pos.0][guard_pos.1] == '>' || grid[guard_pos.0][guard_pos.1] == '<' {
        return '-';
    } else {
        return '|';
    }
}

fn move_guard(grid: &[Vec<char>], part1: bool) -> Vec<Vec<char>> {
    let guard_pos = find_guard_position(grid).expect("No guard found on the grid");
    let mut updated_grid = grid.to_vec();

    if is_obstacle_ahead(guard_pos, grid) {
        updated_grid[guard_pos.0][guard_pos.1] = rotate_direction(grid[guard_pos.0][guard_pos.1]);
    }

    let movement = get_direction_offset(updated_grid[guard_pos.0][guard_pos.1]);
    let new_position = (
        guard_pos.0 as i32 + movement.0,
        guard_pos.1 as i32 + movement.1,
    );

    if is_outside_grid(new_position, (grid.len() as i32, grid[0].len() as i32)) {
        panic!("Guard is outside the grid");
    }

    let (new_x, new_y) = (new_position.0 as usize, new_position.1 as usize);
    updated_grid[new_x][new_y] = updated_grid[guard_pos.0][guard_pos.1];
    if part1 {
        updated_grid[guard_pos.0][guard_pos.1] = 'X';
    } else {
        updated_grid[guard_pos.0][guard_pos.1] = get_loop_marker(grid);
    }

    format_corners(&updated_grid);

    updated_grid
}

fn format_corners(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut result = grid.to_vec();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '|' || grid[r][c] == '-' {
                // Check for adjacent symbols
                let has_horizontal = (c > 0 && (grid[r][c - 1] == '-' || grid[r][c - 1] == '+'))
                    || (c + 1 < cols && (grid[r][c + 1] == '-' || grid[r][c + 1] == '+'));
                let has_vertical = (r > 0 && (grid[r - 1][c] == '|' || grid[r - 1][c] == '+'))
                    || (r + 1 < rows && (grid[r + 1][c] == '|' || grid[r + 1][c] == '+'));

                // Transform to '+' if both horizontal and vertical connections exist
                if has_horizontal && has_vertical {
                    result[r][c] = '+';
                }
            }
        }
    }

    result
}

fn display_grid(grid: &[Vec<char>]) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn count_char(grid: &[Vec<char>], target: char) -> usize {
    grid.iter().flatten().filter(|&&c| c == target).count()
}

// ======================== Part 2 ========================

fn get_guard_pos(_map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0.._map.len() {
        for j in 0.._map[i].len() {
            if _map[i][j] == '^' {
                return (i, j);
            }
        }
    }
    (0, 0) // default, in case guard is not found (shouldn't happen)
}

fn patrol(
    _map: &Vec<Vec<char>>,
    pos: Option<(usize, usize)>,
    idx: Option<usize>,
) -> (
    bool,
    HashSet<(usize, usize)>,
    HashMap<(usize, usize), ((usize, usize), usize)>,
) {
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let rows = _map.len();
    let cols = _map[0].len();

    let mut visited = HashSet::new();
    let mut visited_entry = HashMap::new();

    let mut pos = pos.unwrap_or_else(|| get_guard_pos(&_map));
    let mut idx = idx.unwrap_or(0);

    visited.insert(pos);

    loop {
        let d = directions[idx];
        let n = (pos.0 as isize + d.0, pos.1 as isize + d.1);

        // Check for boundaries
        if n.0 < 0 || n.0 >= rows as isize || n.1 < 0 || n.1 >= cols as isize {
            return (true, visited, visited_entry); // Leaving the map
        }

        let n = (n.0 as usize, n.1 as usize);

        if _map[n.0][n.1] == '#' {
            idx = (idx + 1) % 4; // Change direction
            continue;
        } else {
            visited.insert(n);
            if !visited_entry.contains_key(&n) {
                visited_entry.insert(n, (pos, idx));
            } else if visited_entry[&n] == (pos, idx) {
                return (false, HashSet::new(), HashMap::new()); // Loop detected
            }
            pos = n;
        }
    }
}

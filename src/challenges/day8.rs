use std::collections::{HashMap, HashSet};

use crate::utils::readfile;

pub fn part1() {
    let field = get_field("./puzzles/puzzled8p1.txt");
    let antennas = extract_antennas(&field);

    let antinodes = calculate_antinodes_generic(&field, &antennas, false);
    println!("{:?}", antinodes.len());
}

pub fn part2() {
    let field = get_field("./puzzles/puzzled8p1.txt");
    let antennas = extract_antennas(&field);
    let antinodes = calculate_antinodes_generic(&field, &antennas, true);
    println!("Amount of antinodes: {:?}", antinodes.len());
}

fn extract_antennas(field: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (row_idx, row) in field.iter().enumerate() {
        for (col_idx, &c) in row.iter().enumerate() {
            if c.is_alphabetic() || c.is_numeric() {
                antennas
                    .entry(c)
                    .or_insert(Vec::new())
                    .push((col_idx, row_idx));
            }
        }
    }

    antennas
}

fn calculate_antinodes_generic(
    field: &Vec<Vec<char>>,
    antennas: &HashMap<char, Vec<(usize, usize)>>,
    extend: bool,
) -> HashSet<(usize, usize)> {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (_, coords) in antennas.iter() {
        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                let diff = (
                    coords[j].0 as i32 - coords[i].0 as i32,
                    coords[j].1 as i32 - coords[i].1 as i32,
                );

                for (_idx, _dir) in [(i, -1), (j, 1)] {
                    let mut pos: (i32, i32) = if extend {
                        (coords[_idx].0 as i32, coords[_idx].1 as i32)
                    } else {
                        (
                            coords[_idx].0 as i32 + (diff.0 * _dir),
                            coords[_idx].1 as i32 + (diff.1 * _dir),
                        )
                    };

                    while 0 <= pos.0
                        && pos.0 < field[0].len() as i32
                        && 0 <= pos.1
                        && pos.1 < field.len() as i32
                    {
                        antinodes.insert((pos.0 as usize, pos.1 as usize));
                        if !extend {
                            break;
                        }
                        pos = (pos.0 + (diff.0 * _dir), pos.1 + (diff.1 * _dir));
                    }
                }
            }
        }
    }

    antinodes
}

fn get_field(path: &str) -> Vec<Vec<char>> {
    let input = readfile(path);
    let mut field = Vec::new();

    for line in input.iter() {
        field.push(line.chars().collect());
    }

    field
}

fn display_field(field: &Vec<Vec<char>>) {
    for row in field.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

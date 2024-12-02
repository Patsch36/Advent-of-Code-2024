use itertools::Itertools;

use crate::utils::readfile;
use default_args::default_args;

default_args! {
    fn count_unsafe_levels(levels: Vec<Vec<i32>>, amount_of_unsafe: usize = 0) -> i32 {
        let mut _sum: i32 = 0;

        for line in levels {
            let original_line = line.clone();
            let mut is_valid = check_line_safety(&line);

            if !is_valid && amount_of_unsafe > 0 {
                // Prüfen aller Kombinationen (der Indexe), um Elemente zu entfernen
                'outer: for combination in (0..line.len()).combinations(amount_of_unsafe) {
                    // Erstellen einer neuen Zeile ohne die unsicheren Elemente
                    let mut modified_line: Vec<i32> = original_line.clone();
                    for &index in combination.iter().rev() {
                        modified_line.remove(index);
                    }

                    if check_line_safety(&modified_line) {
                        is_valid = true;
                        break 'outer;
                    }
                }
            }

            _sum += is_valid as i32;
        }

        _sum
    }
}

/// Funktion zur Überprüfung, ob eine Zeile sicher ist
fn check_line_safety(line: &Vec<i32>) -> bool {
    if line.len() < 2 {
        return true; // Eine Zeile mit weniger als 2 Elementen ist sicher
    }

    let mut is_valid: bool = true;
    let absolute: bool = (line[0] - line[1]) < 0; // Ordnung: steigend oder fallend

    for window in line.windows(2) {
        if let [a, b] = window {
            if (b - a).abs() > 3 || (b - a).abs() <= 0 || (absolute == (a > b)) {
                is_valid = false;
                break;
            }
        }
    }

    is_valid
}

pub fn part1() {
    let levels: Vec<Vec<i32>> = getlist();

    println!("Sum of valid level: {}", count_unsafe_levels!(levels));
}

pub fn part2() {
    let levels: Vec<Vec<i32>> = getlist();

    println!("Sum of valid level: {}", count_unsafe_levels!(levels, 1));
}

fn getlist() -> Vec<Vec<i32>> {
    readfile("puzzles/puzzled2p1.txt")
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

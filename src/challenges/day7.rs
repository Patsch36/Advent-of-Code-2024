use crate::utils::readfile;

pub fn part1() {
    let tests: Vec<(i64, Vec<i64>)> = get_tests();
    let operators: [char; 2] = ['+', '*'];
    let valid_calibration_sum = calculate_valid_calibration_sum(&tests, &operators);
    println!("Sum of valid calibrations: {}", valid_calibration_sum);
    println!("Result is {}", 3351424677624 == valid_calibration_sum);
}

pub fn part2() {
    let tests: Vec<(i64, Vec<i64>)> = get_tests();
    let operators: [char; 3] = ['+', '*', '|'];
    let valid_calibration_sum = calculate_valid_calibration_sum(&tests, &operators);
    println!("Sum of valid calibrations: {}", valid_calibration_sum);
    println!("Result is {}", 204976636995111 == valid_calibration_sum);
}

use rayon::prelude::*;

fn calculate_valid_calibration_sum(tests: &[(i64, Vec<i64>)], operators: &[char]) -> i64 {
    tests
        .par_iter()
        .map(|test| {
            let possible_operators = combinations(operators, test.1.len() - 1);
            for operator in possible_operators {
                let mut result: i64 = test.1[0];
                for i in 0..test.1.len() - 1 {
                    match operator[i] {
                        '+' => result += test.1[i + 1],
                        '*' => result *= test.1[i + 1],
                        '|' => {
                            let num_digits = (test.1[i + 1] as f64).log(10.0).floor() as i64 + 1; // Anzahl der Ziffern in test.1[i + 1]
                            result = result * 10i64.pow(num_digits as u32) + test.1[i + 1];
                        }

                        _ => panic!("Unknown operator"),
                    }
                }
                if result == test.0 {
                    return result;
                }
            }
            0
        })
        .sum()
}

fn get_tests() -> Vec<(i64, Vec<i64>)> {
    let mut tests = Vec::new();
    let input = readfile("./puzzles/puzzled7p1.txt");

    for line in input.iter() {
        let parts: Vec<&str> = line.trim().split(": ").collect();
        if parts.len() != 2 {
            println!("Invalid line format: {}", line); // Debug-Ausgabe
            continue;
        }

        let key: i64 = match parts[0].parse() {
            Ok(k) => k,
            Err(_) => {
                println!(
                    "Fehler beim Parsen des Schlüssels: {} in Zeile: {}",
                    parts[0], line
                ); // Fehlerausgabe
                continue;
            }
        };

        let values: Vec<i64> = parts[1]
            .split_whitespace()
            .map(|s| match s.parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("Fehler beim Parsen des Werts: {} in Zeile: {}", s, line); // Fehlerausgabe
                    return 0; // Fehlerwert, der übersprungen wird
                }
            })
            .collect();

        if values.is_empty() {
            println!("Leere Werte für Zeile: {}", line);
        }

        tests.push((key, values));
    }
    tests
}

fn combinations(operators: &[char], n: usize) -> Vec<Vec<char>> {
    if n == 0 {
        return vec![vec![]]; // Basisfall: Rückgabe eines leeren Vektors
    }

    let mut result = Vec::new();

    for &op in operators {
        let sub_combinations = combinations(operators, n - 1);
        for sub in sub_combinations {
            let mut new_combination = vec![op];
            new_combination.extend(sub);
            result.push(new_combination);
        }
    }

    result
}

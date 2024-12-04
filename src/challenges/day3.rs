use crate::utils::readfile;
use default_args::default_args;
use regex::Regex;

default_args! {
    fn read_operations(path: &str, enabling: bool=false) -> i32 {
        let lines = read(path);
        let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        let mut sum: i32 = 0;

        fn process_line(line: &str, re: &Regex) -> i32 {
            let mut sum = 0;
            for cap in re.captures_iter(line) {
                let parts: Vec<&str> = cap[0][4..cap[0].len()-1].split(',').collect();
                let num1 = parts[0].parse::<i32>().unwrap();
                let num2 = parts[1].parse::<i32>().unwrap();
                sum += num1 * num2;
            }
            sum
        }

        if !enabling {
            for line in lines {
                sum += process_line(&line, &re);
            }
        } else {
            let mut _counting = true;
            for line in lines {
                let parts: Vec<&str> = line
                    .split_inclusive("do()")
                    .flat_map(|s| s.split_inclusive("don't()"))
                    .collect();
                
                for part in parts {
                    if _counting {
                        sum += process_line(part, &re);
                    }

                    if part.ends_with("don't()") {
                        _counting = false;
                    } else if part.ends_with("do()") {
                        _counting = true;
                    }
                }
            }
        }
        sum
    }
}

pub fn part1() {
    println!(
        "Sum of all multiplication operations: {}",
        read_operations!("puzzles/puzzled3p1.txt")
    );
}

pub fn part2() {
    println!("Sum of all enabled multiplication operations: {}", read_operations!("puzzles/puzzled3p1.txt", true));
}

fn read(path: &str) -> Vec<String> {
    readfile(path)
}

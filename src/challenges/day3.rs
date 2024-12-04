use crate::utils::readfile;
use default_args::default_args;
use regex::Regex;

default_args! {
    fn read_operations(path: &str, enabling: bool=false) -> Result<i32, Box<dyn std::error::Error>> {
        let lines = read(path)?;
        let re = Regex::new(r"mul\(\d+,\d+\)")?;
        let mut total_sum: i32 = 0;

        fn process_line(line: &str, re: &Regex) -> Result<i32, Box<dyn std::error::Error>> {
            let mut line_sum = 0;
            for cap in re.captures_iter(line) {
            let parts: Vec<&str> = cap[0][4..cap[0].len()-1].split(',').collect();
            let num1 = parts[0].parse::<i32>()?;
            let num2 = parts[1].parse::<i32>()?;
            line_sum += num1 * num2;
            }
            Ok(line_sum)
        }

        if !enabling {
            total_sum = lines.iter()
            .map(|line| process_line(line, &re))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum();
        } else {
            let mut counting = true;
            for line in lines {
                let parts: Vec<&str> = line
                    .split_inclusive("do()")
                    .flat_map(|s| s.split_inclusive("don't()"))
                    .collect();

                for part in parts {
                    if counting {
                    total_sum += process_line(part, &re)?;
                    }

                    if part.ends_with("don't()") {
                    counting = false;
                    } else if part.ends_with("do()") {
                    counting = true;
                    }
                }
            }
        }
        Ok(total_sum)
    }
}

pub fn part1() {
    match read_operations!("puzzles/puzzled3p1.txt") {
        Ok(sum) => println!("Sum of all multiplication operations: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn part2() {
    match read_operations!("puzzles/puzzled3p1.txt", true) {
        Ok(sum) => println!("Sum of all enabled multiplication operations: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn read(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok(readfile(path))
}

mod challenges;
mod utils;

use chrono::{self, Datelike};

fn main() {
    // let day = 2;
    let day: u32 = chrono::offset::Local::now().day();

    match day {
        1 => {
            challenges::day1::part1();
            challenges::day1::part2();
        }
        2 => {
            challenges::day2::part1();
            challenges::day2::part2();
        }
        3 => {
            challenges::day3::part1();
            challenges::day3::part2();
        }
        4 => {
            challenges::day4::part1();
            challenges::day4::part2();
        }
        5 => {
            challenges::day5::part1();
            challenges::day5::part2();
        }
        6 => {
            challenges::day6::part1();
            challenges::day6::part2();
        }
        7 => {
            challenges::day7::part1();
            challenges::day7::part2();
        }
        8 => {
            challenges::day8::part1();
            challenges::day8::part2();
        }
        _ => {
            println!("Day {} not implemented yet", day);
        }
    }
}

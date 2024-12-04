mod challenges;
mod utils;

use chrono::{self, Datelike};

fn main() {
    // let day = 2;
    let day: u32 = chrono::offset::Local::now().day();

    if day == 1 {
        challenges::day1::part1();
        challenges::day1::part2();
    } else if day == 2 {
        challenges::day2::part1();
        challenges::day2::part2();
    } else if day == 3 {
        challenges::day3::part1();
        challenges::day3::part2();
    } else {
        println!("Day {} not implemented yet", day);
    }
}

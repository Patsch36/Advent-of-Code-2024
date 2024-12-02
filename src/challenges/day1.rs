use crate::utils::readfile;

pub fn part1() {
    let (mut first_list, mut second_list): (Vec<i32>, Vec<i32>) = getlists();

    first_list.sort();
    second_list.sort();

    let mut sum: i32 = 0;

    for i in 0..first_list.len() {
        sum += (first_list[i] + second_list[i]).abs();
    }

    println!("Sum: {}", sum);
}

pub fn part2() {
    let (first_list, second_list): (Vec<i32>, Vec<i32>) = getlists();

    let mut sum = 0;

    for i in 0..first_list.len() {
        let i: usize = i;
        for j in 0..second_list.len() {
            let j: usize = j;
            if first_list[i] == second_list[j] {
                sum += first_list[i];
            }
        }
    }

    println!("Similarity: {}", sum);
}

fn getlists() -> (Vec<i32>, Vec<i32>) {
    let v: Vec<String> = readfile("puzzles/puzzled1p1.txt");

    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    for line in v {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(first), Ok(second)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                first_list.push(first);
                second_list.push(second);
            }
        }
    }

    (first_list, second_list)
}

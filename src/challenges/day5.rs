use crate::utils::readfile;

pub fn part1() {
    let string_lists: StringLists = getlists(); // StringLists wird erstellt
                                                // println!("{}", string_lists); // Custom-Format ausgeben

    // Die Listen extrahieren
    let StringLists(l1, l2) = string_lists;

    let mut midsum = 0;

    // let midsum: i32 = l2
    //     .iter()
    //     .filter(|listel| {
    //         listel
    //             .windows(2)
    //             .all(|pair| isbefore(pair[1], pair[0], &l1))
    //     })
    //     .map(|listel| listel[listel.len() / 2])
    //     .sum();

    for listel in l2 {
        // println!("{:?} is valid: {}", listel, isvalid);
        if validate_line(&listel.clone(), &l1) {
            let midel = listel[(listel.len() / 2) as usize];
            midsum += midel
        }
    }
    println!("Sum of middle elements: {}", midsum);
}

pub fn part2() {
    let StringLists(l1, l2) = getlists();
    let mut midsum = 0;
    for listel in l2 {
        if !validate_line(&listel.clone(), &l1) {
            // get elements in right order
            let newlist = sort_correctly(&listel, &l1);

            let midel = newlist[(newlist.len() / 2) as usize];
            midsum += midel;
        }
    }
    println!("Sum of middle elements: {}", midsum);
}

fn getlists() -> StringLists {
    let lines: Vec<String> = readfile("puzzles/puzzled5p1.txt");

    let split_lines = lines
        .split(|line| line.trim().is_empty())
        .collect::<Vec<_>>();
    let (before_empty, after_empty): (&[&[String]], &[&[String]]) = split_lines.split_at(1);

    let before_empty: Vec<String> = before_empty.iter().flat_map(|&s| s.to_vec()).collect();
    let after_empty: Vec<String> = after_empty.iter().flat_map(|&s| s.to_vec()).collect();

    StringLists::new(before_empty, after_empty)
}

fn validate_line(line: &Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    let mut isvalid = true;
    for idx in (1..line.len()).rev() {
        let num = line[idx];
        let beforenum = line[idx - 1];
        let valid = isbefore(num, beforenum, &rules);
        isvalid = isvalid && valid;
    }
    isvalid
}

fn isbefore(num: i32, before: i32, rules: &Vec<Vec<i32>>) -> bool {
    // any in recursion is bad
    // rules.iter().any(|outerel| {
    //     (before == outerel[0] && num == outerel[1])
    //         || (before == outerel[0] && isbefore(num, outerel[1], rules))
    // })
    for outerel in rules.iter() {
        if before == outerel[0] {
            if num == outerel[1] {
                return true;
            }
            for innerel in rules.iter() {
                if before == innerel[0] && num == innerel[1] {
                    return true;
                }
            }
        }
    }
    false
}

fn sort_correctly(list: &Vec<i32>, rules: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut sorted_list = list.clone();

    sorted_list.sort_by(|&a, &b| {
        if isbefore(a, b, rules) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    sorted_list
}

// ==== Custom Types ==========================================================

struct StringLists(Vec<Vec<i32>>, Vec<Vec<i32>>);

impl StringLists {
    fn new(before_empty: Vec<String>, after_empty: Vec<String>) -> Self {
        let listone: Vec<Vec<i32>> = before_empty
            .iter()
            .map(|s| {
                s.split('|')
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();
        let listtwo: Vec<Vec<i32>> = after_empty
            .iter()
            .map(|s| {
                s.split(',')
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        StringLists(listone, listtwo)
    }
}

impl std::fmt::Display for StringLists {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let StringLists(before_empty, after_empty) = self;
        write!(
            f,
            "Before empty: {:?}\nAfter empty: {:?}",
            before_empty, after_empty
        )
    }
}

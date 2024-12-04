use crate::utils::readfile;

pub fn part1() {
    let grid: Vec<Vec<char>> = get_grid();
    let word: &str = "XMAS";
    let occurrences = count_occurrences(&grid, word);
    println!("Found {} occurrences of {}", occurrences, word);
}

pub fn part2() {
    let grid: Vec<Vec<char>> = get_grid();
    let xmas_patterns = count_xmas_patterns(&grid);
    println!("Found {} X-MAS patterns", xmas_patterns);
}

fn get_grid() -> Vec<Vec<char>> {
    let grid_rows: Vec<String> = readfile("puzzles/puzzled4p1.txt");
    grid_rows.iter().map(|row| row.chars().collect()).collect()
}

fn count_occurrences(grid: &Vec<Vec<char>>, word: &str) -> usize {
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (1, 1),   // Down-right
        (1, -1),  // Down-left
        (0, -1),  // Left
        (-1, 0),  // Up
        (-1, -1), // Up-left
        (-1, 1),  // Up-right
    ];
    
    let word_len = word.len();
    let word_chars: Vec<char> = word.chars().collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dx, dy) in &directions {
                let mut found = true;
                for k in 0..word_len {
                    let x = row as isize + k as isize * dx;
                    let y = col as isize + k as isize * dy;

                    if x < 0 || x >= rows as isize || y < 0 || y >= cols as isize {
                        found = false;
                        break;
                    }
                    if grid[x as usize][y as usize] != word_chars[k] {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_xmas_patterns(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut count = 0;

    // Define relative positions for diagonal checks
    let directions = [
        [(-1, -1), (1, 1)],   // Diagonal top-left to bottom-right
        [(1, -1), (-1, 1)],   // Diagonal top-right to bottom-left
        [(1, 1), (-1, -1)],   // Diagonal bottom-right to top-left
        [(-1, 1), (1, -1)],   // Diagonal bottom-left to top-right
    ];

    // Iterate through the grid, skipping the edges
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if grid[row as usize][col as usize] == 'A' {
                let mut found = 0;

                // Check all diagonal directions for the X-MAS pattern
                for &[(dx_m, dy_m), (dx_s, dy_s)] in &directions {
                    let (x_m, y_m) = (row + dx_m, col + dy_m);
                    let (x_s, y_s) = (row + dx_s, col + dy_s);

                    // Ensure neighbors are within bounds
                    if x_m >= 0 && x_m < rows && y_m >= 0 && y_m < cols
                        && x_s >= 0 && x_s < rows && y_s >= 0 && y_s < cols
                    {
                        if grid[x_m as usize][y_m as usize] == 'M'
                            && grid[x_s as usize][y_s as usize] == 'S'
                        {
                            found += 1;
                        }
                    }

                    // Count only once per 'A' if pattern is confirmed
                    if found == 2 {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }

    count
}

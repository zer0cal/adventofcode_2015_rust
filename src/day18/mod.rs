// --- Day 18: Like a GIF For Your Yard ---

use std::{cmp::max, cmp::min, fs, thread, time};

pub fn answer() {
    println!("Day 18: Like a GIF For Your Yard");
    let file = fs::read_to_string("src/day18/input.txt").unwrap();
    let ans1 = pt1(&file, 100);
    println!("Answer to pt 1 is {}", ans1);
    let ans2 = pt2(&file, 100);
    println!("Answer to pt 2 is {}", ans2);
}

fn pt1(file: &str, cycles: i32) -> u32 {
    let mut grid = load_grid(file);
    for _ in 1..=cycles {
        let next_lifecycle = next_lifecycle(&grid);
        grid = next_lifecycle;
    }
    count_lights(&grid)
}

fn _pt1_with_prints(file: &str, cycles: i32) -> u32 {
    println!("cycle 0");
    let mut grid = load_grid(file);
    println!("{}", file);
    for i in 1..=cycles {
        print!("\x1Bc");
        println!("cycle {}", i);
        let next_lifecycle = next_lifecycle(&grid);
        let s = grid_to_str(&next_lifecycle);
        println!("{}", s);
        grid = next_lifecycle;
        thread::sleep(time::Duration::from_millis(200));
    }
    count_lights(&grid)
}
fn pt2(file: &str, cycles: i32) -> u32 {
    println!("cycle 0");
    let grid = load_grid(file);
    let mut grid = lightup_corners(grid);
    println!("{}", file);
    for i in 1..=cycles {
        print!("\x1Bc");
        println!("cycle {}", i);
        let next_lifecycle = next_lifecycle_pt2(&grid);
        let s = grid_to_str(&next_lifecycle);
        println!("{}", s);
        grid = next_lifecycle;
        thread::sleep(time::Duration::from_millis(200));
    }
    count_lights(&grid)
}

fn count_lights(grid: &[Vec<bool>]) -> u32 {
    let mut lighted = 0u32;
    let width = grid.len();
    let height = grid[0].len();
    (0..width).for_each(|i| {
        (0..height).for_each(|j| {
            if grid[i][j] {
                lighted += 1;
            }
        });
    });
    lighted
}

fn next_lifecycle(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut next = grid.clone();
    let width = grid.len();
    let height = grid[0].len();
    (0..width).for_each(|i| {
        (0..height).for_each(|j| {
            next[i][j] = will_survive(i as isize, j as isize, grid);
        });
    });
    next
}

fn next_lifecycle_pt2(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut next = grid.clone();
    let width = grid.len();
    let height = grid[0].len();
    (0..width).for_each(|i| {
        (0..height).for_each(|j| {
            next[i][j] = will_survive_pt2(i as isize, j as isize, grid);
        });
    });
    next
}

fn load_grid(file: &str) -> Vec<Vec<bool>> {
    let mut result = Vec::new();
    file.lines().for_each(|line| {
        result.push(line.chars().map(|char| char == '#').collect());
    });
    result
}

fn grid_to_str(grid: &[Vec<bool>]) -> String {
    let mut s = String::new();
    let width = grid.len();
    let height = grid[0].len();
    (0..width).for_each(|i| {
        (0..height).for_each(|j| {
            s.push(if grid[i][j] { '#' } else { '.' });
        });
        s.push('\n');
    });
    s.trim().to_string()
}

fn will_survive(x: isize, y: isize, grid: &[Vec<bool>]) -> bool {
    let width = grid.len();
    let height = grid[0].len();
    let left_border = max(0, x - 1) as usize;
    let right_border = min(width - 1, x as usize + 1);
    let top_border = max(0, y - 1) as usize;
    let bottom_border = min(height - 1, y as usize + 1);
    let mut count = 0;
    (left_border..=right_border).for_each(|i| {
        (top_border..=bottom_border).for_each(|j| {
            if !(i == x as usize && j == y as usize) && grid[i][j] {
                count += 1;
            }
        });
    });
    if grid[x as usize][y as usize] {
        return count == 2 || count == 3;
    }
    count == 3
}

fn will_survive_pt2(x: isize, y: isize, grid: &[Vec<bool>]) -> bool {
    let width = grid.len();
    let height = grid[0].len();
    let left_border = max(0, x - 1) as usize;
    let right_border = min(width - 1, x as usize + 1);
    let top_border = max(0, y - 1) as usize;
    let bottom_border = min(height - 1, y as usize + 1);
    if x as usize == 0 && y as usize == 0 {
        return true;
    }
    if x as usize == 0 && y as usize == height - 1 {
        return true;
    }
    if x as usize == width - 1 && y as usize == 0 {
        return true;
    }
    if x as usize == width - 1 && y as usize == height - 1 {
        return true;
    }
    let mut count = 0;
    (left_border..=right_border).for_each(|i| {
        (top_border..=bottom_border).for_each(|j| {
            if !(i == x as usize && j == y as usize) && grid[i][j] {
                count += 1;
            }
        });
    });
    if grid[x as usize][y as usize] {
        return count == 2 || count == 3;
    }
    count == 3
}

fn lightup_corners(mut grid: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let width = grid.len();
    let height = grid[0].len();
    grid[0][0] = true;
    grid[0][height - 1] = true;
    grid[width - 1][0] = true;
    grid[width - 1][height - 1] = true;
    grid
}

#[cfg(test)]
mod tests {
    use crate::day18::{lightup_corners, next_lifecycle_pt2};

    use super::{grid_to_str, load_grid, next_lifecycle};

    #[test]
    fn load_grid_test() {
        let str_0 = ".#.\n\
            ...\n\
            #..";
        let expected_grid = [
            [false, true, false].to_vec(),
            [false, false, false].to_vec(),
            [true, false, false].to_vec(),
        ]
        .to_vec();
        let tested_grid = load_grid(str_0);
        assert_eq!(expected_grid, tested_grid);
    }

    #[test]
    fn next_lifecycle_test() {
        let state_0 = ".#.#.#\n\
            ...##.\n\
            #....#\n\
            ..#...\n\
            #.#..#\n\
            ####..";
        let grid_0 = load_grid(state_0);
        let grid_0_next_lifecycle = next_lifecycle(&grid_0);
        let state_0_next_lifecycle = grid_to_str(&grid_0_next_lifecycle);
        let state_1 = "..##..\n\
            ..##.#\n\
            ...##.\n\
            ......\n\
            #.....\n\
            #.##..";
        assert_eq!(state_0_next_lifecycle, state_1);
    }

    #[test]
    fn lightup_corners_test() {
        let state_0 = ".#.#.#\n\
            ...##.\n\
            #....#\n\
            ..#...\n\
            #.#..#\n\
            ####..";
        let grid_0 = load_grid(state_0);
        let grid_0_corners_lightup = lightup_corners(grid_0);
        let state_0_next_lifecycle = grid_to_str(&grid_0_corners_lightup);
        let state_1 = "##.#.#\n\
            ...##.\n\
            #....#\n\
            ..#...\n\
            #.#..#\n\
            ####.#";
        assert_eq!(state_0_next_lifecycle, state_1);
    }

    #[test]
    fn next_lifecycle_pt2_test() {
        let state_0 = ".#.#.#\n\
            ...##.\n\
            #....#\n\
            ..#...\n\
            #.#..#\n\
            ####..";
        let grid_0 = load_grid(state_0);
        let grid_0_next_lifecycle = next_lifecycle_pt2(&grid_0);
        let state_0_next_lifecycle = grid_to_str(&grid_0_next_lifecycle);
        let state_1 = "#.##.#\n\
            ..##.#\n\
            ...##.\n\
            ......\n\
            #.....\n\
            #.##.#";
        assert_eq!(state_0_next_lifecycle, state_1);
    }
}

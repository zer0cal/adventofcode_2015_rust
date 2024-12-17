// --- Day 3: Perfectly Spherical Houses in a Vacuum ---

#![allow(dead_code)]

use std::collections::HashSet;
use std::fs;

pub fn answer() {
    println!("Day 3: Prefectly Spherical Houses in a Vacuum");
    let input = fs::read_to_string("day3_input.txt").expect("err reading day3 input");
    let ans = at_least_one_present(&input);
    let ans2 = at_least_one_present_with_robot(&input);
    println!("answer to pt 1 is {ans}");
    println!("answer to pt 2 is {ans2}");
}

#[derive(Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn from(point: &Point) -> Point {
        Point {
            x: point.x,
            y: point.y,
        }
    }
}

fn at_least_one_present(s: &str) -> u32 {
    let mut points: HashSet<Point> = HashSet::new();
    let mut santa_position = Point::new(0, 0);
    points.insert(Point::from(&santa_position));
    s.chars().for_each(|u| match u {
        '>' => {
            santa_position = Point::new(santa_position.x + 1, santa_position.y);
            points.insert(Point::from(&santa_position));
        }
        '<' => {
            santa_position = Point::new(santa_position.x - 1, santa_position.y);
            points.insert(Point::from(&santa_position));
        }
        '^' => {
            santa_position = Point::new(santa_position.x, santa_position.y + 1);
            points.insert(Point::from(&santa_position));
        }
        'v' => {
            santa_position = Point::new(santa_position.x, santa_position.y - 1);
            points.insert(Point::from(&santa_position));
        }
        _ => {}
    });
    points.len() as u32
}

fn at_least_one_present_with_robot(s: &str) -> u32 {
    let mut points: HashSet<Point> = HashSet::new();
    let mut santa_position = Point::new(0, 0);
    let mut robot_position = Point::new(0, 0);
    points.insert(Point::from(&santa_position));
    s.chars().enumerate().for_each(|(i, u)| match (i % 2, u) {
        (0, '>') => {
            santa_position = Point::new(santa_position.x + 1, santa_position.y);
            points.insert(Point::from(&santa_position));
        }
        (0, '<') => {
            santa_position = Point::new(santa_position.x - 1, santa_position.y);
            points.insert(Point::from(&santa_position));
        }
        (0, '^') => {
            santa_position = Point::new(santa_position.x, santa_position.y + 1);
            points.insert(Point::from(&santa_position));
        }
        (0, 'v') => {
            santa_position = Point::new(santa_position.x, santa_position.y - 1);
            points.insert(Point::from(&santa_position));
        }
        (1, '>') => {
            robot_position = Point::new(robot_position.x + 1, robot_position.y);
            points.insert(Point::from(&robot_position));
        }
        (1, '<') => {
            robot_position = Point::new(robot_position.x - 1, robot_position.y);
            points.insert(Point::from(&robot_position));
        }
        (1, '^') => {
            robot_position = Point::new(robot_position.x, robot_position.y + 1);
            points.insert(Point::from(&robot_position));
        }
        (1, 'v') => {
            robot_position = Point::new(robot_position.x, robot_position.y - 1);
            points.insert(Point::from(&robot_position));
        }
        _ => {}
    });
    points.len() as u32
}

#[cfg(test)]
mod tests {
    use crate::day3::at_least_one_present;
    use crate::day3::at_least_one_present_with_robot;

    #[test]
    fn part_one() {
        assert_eq!(at_least_one_present(&String::from(">")), 2, ">");
        assert_eq!(at_least_one_present(&String::from("^>v<")), 4, "^>v<");
        assert_eq!(at_least_one_present(&String::from("^v^v^v^v^v")), 2, "^>v<");
    }

    #[test]
    fn part_two() {
        assert_eq!(
            at_least_one_present_with_robot(&String::from("^v")),
            3,
            "^v"
        );
        assert_eq!(
            at_least_one_present_with_robot(&String::from("^>v<")),
            3,
            "^>v<"
        );
        assert_eq!(
            at_least_one_present_with_robot(&String::from("^v^v^v^v^v")),
            11,
            "^v^v^v^v^v"
        );
    }
}

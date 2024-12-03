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

// Santa is delivering presents to an infinite two-dimensional grid of houses.

// He begins by delivering a present to the house at his starting location, and then an elf at the North Pole
// calls him via radio and tells him where to move next. Moves are always exactly one house to the north (^),
// south (v), east (>), or west (<). After each move, he delivers another present to the house at his new
// location.

// However, the elf back at the north pole has had a little too much eggnog, and so his directions are a little
// off, and Santa ends up visiting some houses more than once. How many houses receive at least one present?

// For example:

// > delivers presents to 2 houses: one at the starting location, and one to the east.
// ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
// ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.

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

// --- Part Two ---
// The next year, to speed up the process, Santa creates a robot version of himself, Robo-Santa, to deliver
// presents with him.

// Santa and Robo-Santa start at the same location (delivering two presents to the same starting house), then
// take turns moving based on instructions from the elf, who is eggnoggedly reading from the same script as the
// previous year.

// This year, how many houses receive at least one present?

// For example:

// ^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
// ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
// ^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.

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

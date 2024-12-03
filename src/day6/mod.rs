// --- Day 6: Probably a Fire Hazard ---

#![allow(dead_code)]

use std::fs;

pub mod opt;

pub fn answer() {
    println!("Day 6: Probably a Fire Hazard");
    let input = fs::read_to_string("day6_input.txt").expect("err reading day 6 input");
    let ans = how_many_lights_are_lit(input);
    println!("answer for pt1 is {}", ans);
}

// Because your neighbors keep defeating you in the holiday house decorating contest year after year, you've
// decided to deploy one million lights in a 1000x1000 grid.

// Furthermore, because you've been especially nice this year, Santa has mailed you instructions on how to
// display the ideal lighting configuration.

// Lights in your grid are numbered from 0 to 999 in each direction; the lights at each corner are at 0,0,
// 0,999, 999,999, and 999,0. The instructions include whether to turn on, turn off, or toggle various inclusive
// ranges given as coordinate pairs. Each coordinate pair represents opposite corners of a rectangle, inclusive;
// a coordinate pair like 0,0 through 2,2 therefore refers to 9 lights in a 3x3 square. The lights all start
// turned off.

// To defeat your neighbors this year, all you have to do is set up your lights by doing the instructions Santa
// sent you in order.

// For example:

// turn on 0,0 through 999,999 would turn on (or leave on) every light.
// toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning off the ones that were on, and
// turning on the ones that were off.
// turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights.
// After following the instructions, how many lights are lit?

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    fn from_str(s: &str) -> Point {
        let mut spl = s.split(',');
        if let (Some(sx), Some(sy)) = (spl.next(), spl.next()) {
            if let (Ok(x), Ok(y)) = (sx.parse::<u32>(), sy.parse::<u32>()) {
                return Point::new(x, y);
            }
            panic!();
        }
        panic!();
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x < other.x || self.x == other.x && self.y < other.y {
            return Some(std::cmp::Ordering::Less);
        } else if self.x <= other.x || self.x == other.x && self.y <= other.y {
            return Some(std::cmp::Ordering::Less);
        } else if self.x > other.x || self.x == other.x && self.y > other.y {
            return Some(std::cmp::Ordering::Greater);
        } else if self.x >= other.x || self.x == other.x && self.y >= other.y {
            return Some(std::cmp::Ordering::Greater);
        } else {
            return Some(std::cmp::Ordering::Equal);
        }
    }
}

impl Ord for Point {
    fn clamp(self, min: Point, max: Point) -> Point
    where
        Self: Sized,
        Self: PartialOrd,
    {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.x < other.x || self.x == other.x && self.y < other.y {
            return std::cmp::Ordering::Less;
        } else if self.x <= other.x || self.x == other.x && self.y <= other.y {
            return std::cmp::Ordering::Less;
        } else if self.x > other.x || self.x == other.x && self.y > other.y {
            return std::cmp::Ordering::Greater;
        } else if self.x >= other.x || self.x == other.x && self.y >= other.y {
            return std::cmp::Ordering::Greater;
        } else {
            return std::cmp::Ordering::Equal;
        }
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

#[derive(Debug)]
struct Rectangle {
    start: Point,
    end: Point,
}

impl Rectangle {
    fn new(start: Point, end: Point) -> Rectangle {
        Rectangle { start, end }
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

enum Action {
    Toggle(Rectangle),
    TurnOn(Rectangle),
    TurnOff(Rectangle),
    Undefined,
}

struct Lights {
    turned_on: std::collections::BTreeSet<Point>,
}

impl Lights {
    fn new() -> Lights {
        Lights {
            turned_on: std::collections::BTreeSet::new(),
        }
    }

    fn turn_on(&mut self, r: Rectangle) {
        for i in r.start.x..=r.end.x {
            for j in r.start.y..=r.end.y {
                if !self.turned_on.iter().any(|p| p.x == i && p.y == j) {
                    self.turned_on.insert(Point::new(i, j));
                }
            }
        }
    }

    fn turn_off(&mut self, r: Rectangle) {
        for i in r.start.x..=r.end.x {
            for j in r.start.y..=r.end.y {
                self.turned_on.retain(|p| p.x != i && p.y != j)
            }
        }
    }

    fn toggle(&mut self, r: Rectangle) {
        for i in r.start.x..=r.end.x {
            for j in r.start.y..=r.end.y {
                if self.turned_on.iter().any(|p| p.x == i && p.y == j) {
                    self.turned_on.retain(|p| p.x != i && p.y != j)
                } else {
                    self.turned_on.insert(Point::new(i, j));
                }
            }
        }
    }

    fn len(&self) -> usize {
        self.turned_on.len()
    }
}

fn parse_action(s: &str) -> Action {
    let mut spl = s.split_whitespace();
    match (spl.next(), spl.next(), spl.next(), spl.next(), spl.next()) {
        (Some("toggle"), Some(s1), Some(_), Some(s2), None) => {
            let space = Rectangle::new(Point::from_str(s1), Point::from_str(s2));
            return Action::Toggle(space);
        }
        (Some("turn"), Some("on"), Some(s1), Some(_), Some(s2)) => {
            let space = Rectangle::new(Point::from_str(s1), Point::from_str(s2));
            return Action::TurnOn(space);
        }
        (Some("turn"), Some("off"), Some(s1), Some(_), Some(s2)) => {
            let space = Rectangle::new(Point::from_str(s1), Point::from_str(s2));
            return Action::TurnOff(space);
        }
        _ => {
            return Action::Undefined;
        }
    }
}

fn how_many_lights_are_lit(s: String) -> usize {
    let mut lights = Lights::new();
    for line in s.split('\n') {
        match parse_action(line) {
            Action::Toggle(r) => {
                lights.toggle(r);
            }
            Action::TurnOn(r) => {
                lights.turn_on(r);
            }
            Action::TurnOff(r) => {
                lights.turn_off(r);
            }
            Action::Undefined => {}
        }
    }
    lights.len()
}

#[cfg(test)]
mod tests {
    use crate::day6::{parse_action, Action, Lights, Point, Rectangle};

    #[test]
    fn parse_action_test_toggle() {
        let input = String::from("toggle 0,0 through 1,1");
        let space = Rectangle::new(Point::new(0, 0), Point::new(1, 1));
        let result = parse_action(&input);
        if let Action::Toggle(s) = result {
            assert_eq!(s, space);
        } else {
            panic!();
        }
    }

    #[test]
    fn parse_action_test_turn_on() {
        let input = String::from("turn on 0,0 through 1,1");
        let space = Rectangle::new(Point::new(0, 0), Point::new(1, 1));
        let result = parse_action(&input);
        if let Action::TurnOn(s) = result {
            assert_eq!(s, space);
        } else {
            panic!();
        }
    }

    #[test]
    fn parse_action_test_turn_off() {
        let input = String::from("turn off 0,0 through 1,1");
        let space = Rectangle::new(Point::new(0, 0), Point::new(1, 1));
        let result = parse_action(&input);
        if let Action::TurnOff(s) = result {
            assert_eq!(s, space);
        } else {
            panic!();
        }
    }

    #[test]
    fn turn_lights_on() {
        let rectangle = Rectangle::new(Point::new(0, 0), Point::new(1, 1));
        let mut lights = Lights::new();
        lights.turn_on(rectangle);
        assert_eq!(4, lights.len());
    }

    #[test]
    fn turn_lights_off() {
        let rectangle = Rectangle::new(Point::new(0, 0), Point::new(1, 1));
        let mut lights = Lights::new();
        lights.turned_on.insert(Point::new(0, 0));
        lights.turned_on.insert(Point::new(0, 1));
        lights.turned_on.insert(Point::new(1, 0));
        lights.turned_on.insert(Point::new(1, 1));
        lights.turn_off(rectangle);
        assert_eq!(0, lights.len());
    }

    #[test]
    fn toggle_lights() {
        let rectangle = Rectangle::new(Point::new(0, 0), Point::new(1, 1));
        let mut lights = Lights::new();

        lights.turned_on.insert(Point::new(0, 0));
        lights.turned_on.insert(Point::new(0, 1));
        lights.turned_on.insert(Point::new(0, 2));
        println!("{:?}", lights.turned_on);
        lights.toggle(rectangle);
        println!("{:?}", lights.turned_on);
        assert_eq!(3, lights.len());
    }
}

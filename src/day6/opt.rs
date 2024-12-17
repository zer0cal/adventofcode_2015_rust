// --- Day 6: Probably a Fire Hazard ---

#![allow(dead_code)]

use std::fs;

pub fn answer() {
    println!("Day 6: Probably a Fire Hazard (optimalized version)");
    let input = fs::read_to_string("day6_input.txt").expect("err reading day 6 input");
    let ans = how_many_lights_are_lit(&input);
    let ans2 = total_brightness(&input);
    println!("answer for pt1 is {}", ans);
    println!("answer for pt2 is {}", ans2);
}

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
    turned_on: Vec<bool>,
}

impl Lights {
    fn set(&mut self, point: Point, value: bool) {
        if let Some(p) = self.turned_on.get_mut((point.x * 1000 + point.y) as usize) {
            *p = value;
        }
    }

    fn len(&self) -> usize {
        self.turned_on.iter().filter(|x| **x).count()
    }

    fn new() -> Lights {
        let mut vec = Vec::new();
        vec.reserve(1000000);
        for i in 0..1000000 {
            vec.insert(i, false);
        }
        Lights { turned_on: vec }
    }

    fn toggle(&mut self, r: Rectangle) {
        for i in r.start.x..=r.end.x {
            for j in r.start.y..=r.end.y {
                if let Some(p) = self.turned_on.get_mut((i * 1000 + j) as usize) {
                    *p = !(*p);
                }
            }
        }
    }

    fn turn_off(&mut self, r: Rectangle) {
        for i in r.start.x..=r.end.x {
            for j in r.start.y..=r.end.y {
                if let Some(p) = self.turned_on.get_mut((i * 1000 + j) as usize) {
                    *p = false;
                }
            }
        }
    }

    fn turn_on(&mut self, r: Rectangle) {
        for i in r.start.x..=r.end.x {
            for j in r.start.y..=r.end.y {
                if let Some(p) = self.turned_on.get_mut((i * 1000 + j) as usize) {
                    *p = true;
                }
            }
        }
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

fn how_many_lights_are_lit(s: &str) -> usize {
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

fn total_brightness(s: &str) -> usize {
    let mut lights = LightsWithBrightness::new();
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
    lights.brightness()
}

struct LightsWithBrightness {
    turned_on: Vec<u8>,
}

impl LightsWithBrightness {
    fn set(&mut self, point: Point, value: u8) {
        if let Some(p) = self.turned_on.get_mut((point.x * 1000 + point.y) as usize) {
            *p = value;
        }
    }

    fn brightness(&self) -> usize {
        self.turned_on.iter().fold(0, |acc, e| acc + *e as usize)
    }

    fn new() -> LightsWithBrightness {
        let mut vec = Vec::new();
        vec.reserve(1000000);
        for i in 0..1000000 {
            vec.insert(i, 0);
        }
        LightsWithBrightness { turned_on: vec }
    }

    fn toggle(&mut self, r: Rectangle) {
        for i in r.start.x..=r.end.x {
            for j in r.start.y..=r.end.y {
                if let Some(p) = self.turned_on.get_mut((i * 1000 + j) as usize) {
                    *p += 2;
                }
            }
        }
    }

    fn turn_off(&mut self, r: Rectangle) {
        for i in r.start.x..=r.end.x {
            for j in r.start.y..=r.end.y {
                if let Some(p) = self.turned_on.get_mut((i * 1000 + j) as usize) {
                    if *p > 0 {
                        *p -= 1;
                    }
                }
            }
        }
    }

    fn turn_on(&mut self, r: Rectangle) {
        for i in r.start.x..=r.end.x {
            for j in r.start.y..=r.end.y {
                if let Some(p) = self.turned_on.get_mut((i * 1000 + j) as usize) {
                    *p += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day6::opt::{parse_action, Action, Lights, LightsWithBrightness, Point, Rectangle};

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
        lights.set(Point::new(0, 0), true);
        lights.set(Point::new(0, 1), true);
        lights.set(Point::new(1, 0), true);
        lights.set(Point::new(1, 1), true);
        lights.turn_off(rectangle);
        assert_eq!(0, lights.len());
    }

    #[test]
    fn toggle_lights() {
        let rectangle = Rectangle::new(Point::new(0, 0), Point::new(1, 1));
        let mut lights = Lights::new();
        lights.set(Point::new(0, 0), true);
        lights.set(Point::new(0, 1), true);
        lights.set(Point::new(0, 2), true);
        lights.toggle(rectangle);
        assert_eq!(3, lights.len());
    }

    #[test]
    fn turn_lights_on_pt2() {
        let rectangle = Rectangle::new(Point::new(0, 0), Point::new(1, 1));
        let mut lights = LightsWithBrightness::new();
        lights.turn_on(rectangle);
        assert_eq!(4, lights.brightness());
        let rectangle = Rectangle::new(Point::new(0, 0), Point::new(0, 1));
        lights.turn_on(rectangle);
        assert_eq!(6, lights.brightness());
    }

    #[test]
    fn turn_lights_off_pt2() {
        let rectangle = Rectangle::new(Point::new(0, 0), Point::new(1, 1));
        let mut lights = LightsWithBrightness::new();
        lights.set(Point::new(0, 0), 1);
        lights.set(Point::new(0, 1), 1);
        lights.set(Point::new(1, 0), 1);
        lights.set(Point::new(1, 1), 1);
        lights.turn_off(rectangle);
        assert_eq!(0, lights.brightness());
        lights.set(Point::new(0, 0), 1);
        let rectangle = Rectangle::new(Point::new(0, 0), Point::new(1, 1));
        lights.turn_off(rectangle);
        assert_eq!(0, lights.brightness());
    }

    #[test]
    fn toggle_lights_pt2() {
        let rectangle = Rectangle::new(Point::new(0, 0), Point::new(1, 1));
        let mut lights = LightsWithBrightness::new();
        lights.set(Point::new(0, 2), 1);
        lights.toggle(rectangle);
        assert_eq!(9, lights.brightness());
    }
}

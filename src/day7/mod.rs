// --- Day 7: Some Assembly Required ---
#![allow(dead_code)]
use std::{collections::HashMap, fs};

pub fn answer() {
    println!("Day 7: Some Assembly Required");
    let input = fs::read_to_string("day7_input.txt").expect("err reading day 7 input");
    let input2 = fs::read_to_string("day7_input_pt2.txt").expect("err reading day 7 input");
    let ans = emulate(&input);
    let ans2 = emulate(&input2);
    println!("answer to pt 1 is {}", ans.get("a").unwrap());
    println!("answer to pt 2 is {}", ans2.get("a").unwrap());
}

#[derive(Debug, PartialEq, Eq)]
enum Operation<'a> {
    Set(&'a str),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    LShift(&'a str, u16),
    RShift(&'a str, u16),
    Not(&'a str),
    None,
}

fn parse_instruction(s: &str) -> (Operation, &str) {
    let mut tokens = s.split_whitespace();
    match (
        tokens.next(),
        tokens.next(),
        tokens.next(),
        tokens.next(),
        tokens.next(),
    ) {
        (Some(value), Some("->"), Some(output), None, None) => (Operation::Set(value), output),
        (Some(l), Some("AND"), Some(r), Some("->"), Some(output)) => (Operation::And(l, r), output),
        (Some(l), Some("OR"), Some(r), Some("->"), Some(output)) => (Operation::Or(l, r), output),
        (Some(input), Some("LSHIFT"), Some(n), Some("->"), Some(output)) => {
            if let Ok(number) = n.parse::<u16>() {
                return (Operation::LShift(input, number), output);
            }
            (Operation::None, output)
        }
        (Some(input), Some("RSHIFT"), Some(n), Some("->"), Some(output)) => {
            if let Ok(number) = n.parse::<u16>() {
                return (Operation::RShift(input, number), output);
            }
            (Operation::None, output)
        }
        (Some("NOT"), Some(input), Some("->"), Some(output), None) => {
            (Operation::Not(input), output)
        }
        _ => (Operation::None, ""),
    }
}

fn emulate(s: &str) -> HashMap<String, u16> {
    let splt: Vec<&str> = s.split("\n").collect();
    let mut hm: HashMap<String, u16> = HashMap::new();
    let mut done: Vec<String> = Vec::new();
    loop {
        let mut to_be_done = Vec::new();
        for line in splt.iter().filter(|x| !done.contains(&(x.to_string()))) {
            let operation = parse_instruction(line);
            match operation {
                (Operation::Set(value), wire) => {
                    if let Some(value) = hm.get(value) {
                        to_be_done.push(line.to_string());
                        hm.insert(wire.to_string(), *value);
                    } else if let Ok(value) = value.parse::<u16>() {
                        to_be_done.push(line.to_string());
                        hm.insert(wire.to_string(), value);
                    }
                }
                (Operation::And(l, r), wire) => {
                    if let (Some(l), Some(r)) = (hm.get(l), hm.get(r)) {
                        to_be_done.push(line.to_string());
                        hm.insert(wire.to_string(), l & r);
                    } else if let (Ok(l), Some(r)) = (l.parse::<u16>(), hm.get(r)) {
                        to_be_done.push(line.to_string());
                        hm.insert(wire.to_string(), l & r);
                    } else if let (Some(l), Ok(r)) = (hm.get(l), r.parse::<u16>()) {
                        to_be_done.push(line.to_string());
                        hm.insert(wire.to_string(), l & r);
                    } else if let (Ok(l), Ok(r)) = (l.parse::<u16>(), r.parse::<u16>()) {
                        to_be_done.push(line.to_string());
                        hm.insert(wire.to_string(), l & r);
                    }
                }
                (Operation::Or(l, r), wire) => {
                    if let (Some(l), Some(r)) = (hm.get(l), hm.get(r)) {
                        to_be_done.push(line.to_string());
                        hm.insert(wire.to_string(), l | r);
                    } else if let (Ok(l), Some(r)) = (l.parse::<u16>(), hm.get(r)) {
                        to_be_done.push(line.to_string());
                        hm.insert(wire.to_string(), l | r);
                    } else if let (Some(l), Ok(r)) = (hm.get(l), r.parse::<u16>()) {
                        to_be_done.push(line.to_string());
                        hm.insert(wire.to_string(), l | r);
                    } else if let (Ok(l), Ok(r)) = (l.parse::<u16>(), r.parse::<u16>()) {
                        to_be_done.push(line.to_string());
                        hm.insert(wire.to_string(), l | r);
                    }
                }
                (Operation::LShift(key, n), wire) => {
                    if let Some(val) = hm.get(key) {
                        to_be_done.push(line.to_string());
                        hm.insert(wire.to_string(), val << n);
                    }
                }
                (Operation::RShift(key, n), wire) => {
                    if let Some(val) = hm.get(key) {
                        to_be_done.push(line.to_string());
                        hm.insert(wire.to_string(), val >> n);
                    }
                }
                (Operation::Not(key), wire) => {
                    if let Some(val) = hm.get(key) {
                        to_be_done.push(line.to_string());
                        hm.insert(wire.to_string(), !val);
                    }
                }
                (_, _) => (),
            }
        }
        done.append(&mut to_be_done.clone());
        if to_be_done.is_empty() {
            break;
        }
    }

    hm
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::day7::{parse_instruction, Operation};

    use super::emulate;

    #[test]
    fn parse_test() {
        assert_eq!(parse_instruction("1 -> a"), (Operation::Set("1"), "a"));
        assert_eq!(
            parse_instruction("a AND b -> c"),
            (Operation::And("a", "b"), "c")
        );
        assert_eq!(
            parse_instruction("a OR b -> c"),
            (Operation::Or("a", "b"), "c")
        );
        assert_eq!(
            parse_instruction("a LSHIFT 2 -> b"),
            (Operation::LShift("a", 2), "b")
        );
        assert_eq!(
            parse_instruction("a RSHIFT 2 -> b"),
            (Operation::RShift("a", 2), "b")
        );
        assert_eq!(parse_instruction("NOT a -> b"), (Operation::Not("a"), "b"));
    }

    #[test]
    fn test_1() {
        let mut exp: HashMap<String, u16> = HashMap::new();
        exp.insert("d".to_string(), 72);
        exp.insert("e".to_string(), 507);
        exp.insert("f".to_string(), 492);
        exp.insert("g".to_string(), 114);
        exp.insert("h".to_string(), 65412);
        exp.insert("i".to_string(), 65079);
        exp.insert("x".to_string(), 123);
        exp.insert("y".to_string(), 456);
        let s = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        let ans = emulate(s);
        assert_eq!(ans, exp);
    }

    #[test]
    fn test_2() {
        let mut exp: HashMap<String, u16> = HashMap::new();
        exp.insert("d".to_string(), 72);
        exp.insert("e".to_string(), 507);
        exp.insert("f".to_string(), 492);
        exp.insert("g".to_string(), 114);
        exp.insert("h".to_string(), 65412);
        exp.insert("i".to_string(), 65079);
        exp.insert("j".to_string(), 144);
        exp.insert("x".to_string(), 123);
        exp.insert("y".to_string(), 456);
        let s = "123 -> x
d LSHIFT 1 -> j
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        let ans = emulate(s);
        assert_eq!(ans, exp);
    }
}

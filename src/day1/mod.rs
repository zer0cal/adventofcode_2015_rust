// --- Day 1: Not Quite Lisp ---

#![allow(dead_code)]

use std::fs;

pub fn answer() {
    println!("Day 1: Not Quite Lisp");
    let input = fs::read_to_string("day1_input.txt").expect("err reading day 1 input");
    let ans = what_flor(&input);
    let ans2 = position(&input);
    println!("answer to pt 1 is {}", ans);
    println!("answer to pt 2 is {}", ans2);
}

fn what_flor(s: &str) -> i32 {
    s.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .reduce(|acc, e| acc + e)
        .unwrap_or_default()
}

fn position(s: &str) -> i32 {
    let mut acc = 0;
    for (i, c) in s.chars().enumerate() {
        acc += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if acc == -1 {
            return (i + 1) as i32;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::position;
    use super::what_flor;

    #[test]
    fn first() {
        let result = what_flor(&String::from("(())"));
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn second() {
        let result = what_flor(&String::from("()()"));
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn third() {
        let result = what_flor(&String::from("((("));
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn forth() {
        let result = what_flor(&String::from("(()()("));
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn fifth() {
        let result = what_flor(&String::from(")))"));
        let expected = -3;
        assert_eq!(result, expected);
    }

    #[test]
    fn pt2_first() {
        let res = position(&String::from(")"));
        let exp = 1;
        assert_eq!(res, exp);
    }

    #[test]
    fn pt2_second() {
        let res = position(&String::from("()())"));
        let exp = 5;
        assert_eq!(res, exp);
    }
}

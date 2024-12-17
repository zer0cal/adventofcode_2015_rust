// --- Day 5: Doesn't He Have Intern-Elves For This? ---

#![allow(dead_code)]

use std::fs;

pub fn answer() {
    println!("Day 5: Doesn't He Have Intern-Elves For This?");
    let input = fs::read_to_string("day5_input.txt").expect("err reading day 5 input");
    let ans = input.split_whitespace().filter(|x| nice_string(x)).count();
    let ans2 = input
        .split_whitespace()
        .filter(|x| better_nice_string(x))
        .count();
    println!("answer for pt1 is {ans}");
    println!("answer for pt2 is {ans2}");
}

fn nice_string(s: &str) -> bool {
    if s.as_bytes()
        .iter()
        .filter(|x| [b'a', b'e', b'i', b'o', b'u'].contains(x))
        .count()
        < 3
    {
        return false;
    }
    if !s.as_bytes().windows(2).any(|x| x[0] == x[1]) {
        return false;
    }
    if !s
        .as_bytes()
        .windows(2)
        .any(|x| [(b'a', b'b'), (b'c', b'd'), (b'p', b'q'), (b'x', b'y')].contains(&(x[0], x[1])))
    {
        return true;
    }
    false
}

fn better_nice_string(s: &str) -> bool {
    let l = s.len();
    if !s.as_bytes().windows(3).any(|x| x[0] == x[2]) {
        return false;
    }
    for (i, pair) in s[..l - 2].as_bytes().windows(2).enumerate() {
        if s[i + 2..].as_bytes().windows(2).any(|x| x == pair) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::day5::{better_nice_string, nice_string};

    #[test]
    fn part_one() {
        assert!(nice_string("ugknbfddgicrmopn"));
        assert!(nice_string("aaa"));
        assert!(!nice_string("jchzalrnumimnmhp"));
        assert!(!nice_string("haegwjzuvuyypxyu"));
        assert!(!nice_string("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part_two() {
        assert!(better_nice_string("qjhvhtzxzqqjkmpb"));
        assert!(better_nice_string("xxyxx"));
        assert!(!better_nice_string("uurcxstgmygtbstg"));
        assert!(!better_nice_string("ieodomkazucvgmuy"));
    }
}

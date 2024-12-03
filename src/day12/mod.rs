// --- Day 12: JSAbacusFramework.io ---
#![allow(dead_code)]

use std::fs;

pub fn answer() {
    println!("Day 12: JSAbacusFramework.io");
    let input = fs::read_to_string("day12_input.txt").expect("err reading day 12 input");
    let ans1 = sum_nums(&input);
    let ans2 = sum_nums_without_red(&input);
    println!("answer to pt 1 is {}", ans1);
    println!("answer to pt 2 is {}", ans2);
}

// Santa's Accounting-Elves need help balancing the books after a recent order.
// Unfortunately, their accounting software uses a peculiar storage format.
// That's where you come in.

// They have a JSON document which contains a variety of things: arrays
// ([1,2,3]), objects ({"a":1, "b":2}), numbers, and strings. Your first job is
// to simply find all of the numbers throughout the document and add them together.

// For example:
// [1,2,3] and {"a":2,"b":4} both have a sum of 6.
// [[[3]]] and {"a":{"b":4},"c":-1} both have a sum of 3.
// {"a":[-1,1]} and [-1,{"a":1}] both have a sum of 0.
// [] and {} both have a sum of 0.
// You will not encounter any strings containing numbers.

// What is the sum of all numbers in the document?

fn sum_nums(s: &str) -> i32 {
    let mut acc = 0;
    let mut num_cache = String::new();

    for byte in s.bytes() {
        match byte {
            b':' | b',' => {
                if let Ok(value) = num_cache.parse::<i32>() {
                    acc += value;
                }
                num_cache.clear();
            }
            b'0'..=b'9' | b'-' => num_cache.push(char::from(byte)),
            _ => (),
        }
    }
    if num_cache.len() > 0 {
        if let Ok(value) = num_cache.parse::<i32>() {
            acc += value;
        }
    }
    acc
}

// --- Part Two ---
// Uh oh - the Accounting-Elves have realized that they double-counted

// everything red.
// Ignore any object (and all of its children) which has any property with the
//  value "red". Do this only for objects ({...}), not arrays ([...]).

// [1,2,3] still has a sum of 6.
// [1,{"c":"red","b":2},3] now has a sum of 4, because the middle object is ignored.
// {"d":"red","e":[1,2,3,4],"f":5} now has a sum of 0, because the entire structure is ignored.
// [1,"red",5] has a sum of 6, because "red" in an array has no effect.

fn sum_nums_without_red(s: &str) -> i32 {
    let mut accs = Vec::new();
    accs.push(0);
    let mut bs = Vec::new();
    let mut vc = String::new();
    let mut rv = false;
    let mut nc = String::new();

    for byte in s.bytes() {
        match byte {
            b'{' | b'[' => {
                accs.push(0);
                if let Some(b'r') = bs.last() {
                    bs.push(b'r');
                } else {
                    bs.push(byte);
                }
            }
            b']' | b'}' => {
                if let Ok(value) = nc.parse::<i32>() {
                    if let Some(last) = accs.last_mut() {
                        *last += value;
                    }
                }
                if let Some(value) = accs.pop() {
                    if let Some(last) = accs.last_mut() {
                        *last += value;
                    }
                }
                if let Some(bracket) = bs.pop() {
                    match (bracket, byte) {
                        (b'{', b'}') => (),
                        (b'r', b'}') => (),
                        (b'r', b']') => (),
                        _ => (),
                    }
                }
                nc.clear();
            }
            b'"' => {
                rv = !rv;
                if let Some(bracket) = bs.last() {
                    if vc == "red" && *bracket == b'{' {
                        if let Some(last) = accs.last_mut() {
                            *last = 0;
                        }
                        bs.pop();
                        bs.push(b'r');
                    }
                }
                vc.clear();
            }
            b':' | b',' | b';'
                if (*bs.last().unwrap_or_else(|| &b'[') == b'['
                    || *bs.last().unwrap_or_else(|| &b'{') == b'{')
                    && !rv =>
            {
                if let Ok(value) = nc.parse::<i32>() {
                    if let Some(last) = accs.last_mut() {
                        *last += value;
                    }
                }
                nc.clear();
            }
            b'0'..=b'9' | b'-'
                if (*bs.last().unwrap_or_else(|| &b'[') == b'['
                    || *bs.last().unwrap_or_else(|| &b'{') == b'{')
                    && !rv =>
            {
                nc.push(char::from(byte))
            }
            b'a'..=b'z' | b'A'..=b'Z' if rv => vc.push(char::from(byte)),
            _ => (),
        }
    }
    if let Ok(value) = nc.parse::<i32>() {
        *accs.last_mut().unwrap() = value;
    }
    *accs.last().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day12::{sum_nums, sum_nums_without_red};

    #[test]
    fn singe() {
        assert!(sum_nums_without_red(r#""12345"12345"#) == 12345);
        assert!(sum_nums_without_red(r#"12345"12345""#) == 12345);
    }

    #[test]
    fn array() {
        assert!(sum_nums("[1,2,3]") == 6);
        assert!(sum_nums("[-4,2,3]") == 1);
        assert!(sum_nums_without_red("[1,2,3]") == 6);
        assert!(sum_nums_without_red("[-4,2,3]") == 1);
        assert!(sum_nums_without_red(r#"[-4,2,"red",3]"#) == 1);
        assert!(sum_nums_without_red(r#"["red",-4,2,0,3]"#) == 1);
        assert!(sum_nums_without_red(r#"["-1",1]"#) == 1);
    }

    #[test]
    fn map() {
        assert!(sum_nums(r#"{"a":1,"b":2,"qwer":3}"#) == 6);
        assert!(sum_nums(r#"{"a":-5,"b":2,"qwer":3}"#) == 0);
        assert!(sum_nums_without_red(r#"{"a":1,"b":2,"c":3}"#) == 6);
        assert!(sum_nums_without_red(r#"{"a":"red","b":2,"c":3}"#) == 0);
        assert!(sum_nums_without_red(r#"{"a":1,"b":"red","c":3}"#) == 0);
    }

    #[test]
    fn nested() {
        assert!(sum_nums(r#"{"a":{"b":4},"c":-1}"#) == 3);
        assert!(sum_nums(r#"[[[3]]]"#) == 3);
        assert!(sum_nums(r#"{"a":[-1,1]}"#) == 0);
        assert!(sum_nums(r#"[-1,{"a":1}]"#) == 0);
        assert!(sum_nums_without_red(r#"{"a":{"b":4},"c":-1}"#) == 3);
        assert!(sum_nums_without_red(r#"[[[3]]]"#) == 3);
        assert!(sum_nums_without_red(r#"{"a":[-1,1]}"#) == 0);
        assert!(sum_nums_without_red(r#"[-1,{"a":1}]"#) == 0);
        assert!(sum_nums_without_red(r#"{"d":"red","e":[1,2,{"a":"red"},4],"f":5}"#) == 0);
        assert!(sum_nums_without_red(r#"[1,{"c":"red","b":2},3]"#) == 4);
        assert!(sum_nums_without_red(r#"[1,{"c":"red","b":2},{"a":1},3]"#) == 5);
        assert!(sum_nums_without_red(r#"{"a":["red",1,2],"b":2}"#) == 5);
        assert!(sum_nums_without_red(r#"[[{"a":"a"}]]"#) == 0);
        assert!(sum_nums_without_red(r#"[[{"a":"red","a":"a"}]]"#) == 0);
        assert!(sum_nums_without_red(r#"[1,{"b":"red","b":{"c":"red"}},{"a":1},3]"#) == 5);
        assert!(
            sum_nums_without_red(
                r#"[1,{"b":"red",{"a":1},[1,2,{"a":1},"red"],"b":{"c":"red"}},{"a":1},3],[1]"#
            ) == 6
        );
        assert!(sum_nums_without_red(r#"{"b":"red","a":1},{"a":["red", 1],"b":1}"#) == 2);
    }

    #[test]
    fn empty() {
        assert!(sum_nums(r#"[]"#) == 0);
        assert!(sum_nums(r#"{}"#) == 0);
        assert!(sum_nums_without_red(r#"{}"#) == 0);
        assert!(sum_nums_without_red(r#"[]"#) == 0);
    }
}

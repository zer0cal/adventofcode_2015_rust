// --- Day 10: Elves Look, Elves Say ---
#![allow(dead_code)]

pub fn answer() {
    println!("Day 10: Elves Look, Elves Say");
    let ans1 = iter_process_over_seq("1113222113", 40).len();
    println!("answer to pt 1 is {}", ans1);
    let ans2 = iter_process_over_seq("1113222113", 50).len();
    println!("answer to pt 2 is {}", ans2);
}

fn iter_process_over_seq(seq: &str, iterations: usize) -> String {
    let mut cache = String::new();
    cache.push_str(seq);
    for _ in 0..iterations {
        cache = process_seq(&cache);
    }
    cache
}

fn process_seq(seq: &str) -> String {
    let seq = seq.as_bytes();
    let mut l: usize = 0;
    let mut cache = String::new();
    for i in 1..seq.len() {
        if seq[l] != seq[i] {
            cache.push_str(&format!("{}", i - l));
            cache.push_str(&format!("{}", String::from_utf8(vec![seq[l]]).unwrap()));
            l = i;
        }
    }
    if l != seq.len() {
        cache.push_str(&format!("{}", seq.len() - l));
        cache.push_str(&format!("{}", String::from_utf8(vec![seq[l]]).unwrap()));
    }
    cache
}

#[cfg(test)]
mod tests {
    use crate::day10::iter_process_over_seq;

    use super::process_seq;
    // 1 becomes 11 (1 copy of digit 1).
    // 11 becomes 21 (2 copies of digit 1).
    // 21 becomes 1211 (one 2 followed by one 1).
    // 1211 becomes 111221 (one 1, one 2, and two 1s).
    // 111221 becomes 312211 (three 1s, two 2s, and one 1).

    #[test]
    fn one_one() {
        let tested = process_seq("1");
        let expexted = String::from("11");
        assert_eq!(tested, expexted)
    }

    #[test]
    fn two_ones() {
        let tested = process_seq("11");
        let expexted = String::from("21");
        assert_eq!(tested, expexted)
    }

    #[test]
    fn one_two_one_one() {
        let tested = process_seq("21");
        let expexted = String::from("1211");
        assert_eq!(tested, expexted)
    }

    #[test]
    fn one_one_one_two_two_one() {
        let tested = process_seq("1211");
        let expexted = String::from("111221");
        assert_eq!(tested, expexted)
    }

    #[test]
    fn three_one_two_two_one_one() {
        let tested = process_seq("111221");
        let expexted = String::from("312211");
        assert_eq!(tested, expexted)
    }

    #[test]
    fn five_iter_over_one_one() {
        let tested = iter_process_over_seq("1", 5);
        let expexted = String::from("312211");
        assert_eq!(tested, expexted)
    }
}

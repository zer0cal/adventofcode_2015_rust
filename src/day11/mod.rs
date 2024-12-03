// --- Day 11: Corporate Policy ---
#![allow(dead_code)]
use std::time::Instant;

pub fn answer() {
    println!("Day 11: Corporate Policy");

    let t = Instant::now();
    let ans1u8 = next_pass_u8("cqjxjnds", 99999999);
    println!("answer to pt 1 is {} in {:.2?}", ans1u8, t.elapsed());
    let t = Instant::now();
    let ans2u8 = next_pass_u8(&ans1u8, 99999999);
    println!("answer to pt 2 is {} in {:.2?}", ans2u8, t.elapsed());
    // opt
    let t = Instant::now();
    let ans1u8mut = mut_next_pass_u8("cqjxjnds", 99999999);
    println!("opt answer to pt 1 is {} in {:.2?}", ans1u8mut, t.elapsed());
    let t = Instant::now();
    let ans2u8mut = mut_next_pass_u8(&ans1u8mut, 99999999);
    println!("opt answer to pt 2 is {} in {:.2?}", ans2u8mut, t.elapsed());
}

// Santa's previous password expired, and he needs help choosing a new one.
// To help him remember his new password after the old one expires, Santa has
// devised a method of coming up with a password based on the previous one
// Corporate policy dictates that passwords must be exactly eight lowercase
// letters (for security reasons), so he finds his new password by incrementing
// his old password string repeatedly until it is valid.
// Incrementing is just like counting with numbers: xx, xy, xz, ya, yb,
// and so on. Increase the rightmost letter one step; if it was z, it wraps
// around to a, and repeat with the next letter to the left until one doesn't
// wrap around.
// Unfortunately for Santa, a new Security-Elf recently started, and he has
// imposed some additional password requirements:
// - Passwords must include one increasing straight of at least three letters,
//   doesn't count.
//   like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd
// - Passwords may not contain the letters i, o, or l, as these letters can be
//   mistaken for other characters and are therefore confusing.
// - Passwords must contain at least two different, non-overlapping pairs of
//   letters, like aa, bb, or zz.

// For example:
// - hijklmmn meets the first requirement (because it contains the straight hij)
//   but fails the second requirement requirement (because it contains i and l).
// - abbceffg meets the third requirement (because it repeats bb and ff) but
//   fails the first requirement.
// - abbcegjk fails the third requirement, because it only has one double
//   letter (bb).
// - The next password after abcdefgh is abcdffaa.
// - The next password after ghijklmn is ghjaabcc, because you eventually skip
//   all the passwords that start with ghi..., since i is not allowed.
// - Given Santa's current password (your puzzle input), what should his next
//   password be?

fn check_first_requirement_u8(pass: &[u8]) -> bool {
    for u in pass.windows(3) {
        if u[0] == u[1] - 1 && u[1] == u[2] - 1 {
            return true;
        }
    }
    false
}

fn check_second_requirement_u8(pass: &[u8]) -> bool {
    for u in pass {
        if *u == b'i' || *u == b'o' || *u == b'l' {
            return false;
        }
    }
    true
}

fn check_third_requirement_u8(pass: &[u8]) -> bool {
    let mut count = 0;
    let mut iter = pass.windows(2);
    while let Some(u) = iter.next() {
        if u[0] == u[1] {
            count += 1;
            iter.next();
        }
    }
    if count > 1 {
        return true;
    }
    false
}

fn increment_char(u: &u8, c: u32) -> (u8, u32) {
    if c == 0 {
        return (*u, 0);
    }
    match u {
        b'z' => (b'a', c),
        x if (b'a'..=b'y').contains(x) => (*x + 1, c - 1),
        _ => panic!("Wrong character"),
    }
}

fn incremet_pass_u8(pass: &[u8]) -> Vec<u8> {
    let mut iter = pass.iter().rev();
    let mut c = 1;
    let mut cache: Vec<u8> = Vec::new();
    loop {
        if let Some(u) = iter.next() {
            let (u, cr) = increment_char(u, c);
            c = cr;
            cache.push(u);
        } else if c > 0 {
            cache.push(b'a' + (c as u8) - 1u8);
            break;
        } else {
            break;
        }
    }
    cache.reverse();
    cache
}

fn mut_incremet_pass_u8(pass: &mut [u8]) {
    let mut iter = pass.iter_mut().rev();
    let mut c = 1;
    while let Some(u) = iter.next() {
        let (nu, cr) = increment_char(u, c);
        c = cr;
        *u = nu;
    }
}

fn next_pass_u8(pass: &str, iterations: usize) -> String {
    let mut new_pass = Vec::from_iter(pass.bytes());
    for _ in 0..iterations {
        new_pass = incremet_pass_u8(&new_pass);
        if check_first_requirement_u8(&new_pass)
            && check_second_requirement_u8(&new_pass)
            && check_third_requirement_u8(&new_pass)
        {
            return String::from_utf8(new_pass).unwrap();
        }
    }
    panic!("did not find pass ");
}

fn mut_next_pass_u8(pass: &str, iterations: usize) -> String {
    let mut new_pass = Vec::from_iter(pass.bytes());
    for _ in 0..iterations {
        mut_incremet_pass_u8(&mut new_pass);
        if check_first_requirement_u8(&new_pass)
            && check_second_requirement_u8(&new_pass)
            && check_third_requirement_u8(&new_pass)
        {
            return String::from_utf8(new_pass).unwrap();
        }
    }
    panic!("did not find pass ");
}

#[cfg(test)]
mod tests {
    use crate::day11::{
        check_first_requirement_u8, check_second_requirement_u8, check_third_requirement_u8,
        incremet_pass_u8, mut_incremet_pass_u8,
    };

    #[test]
    fn next_pass_u8_test() {
        assert_eq!(
            incremet_pass_u8("aaa".as_bytes()),
            Vec::from_iter("aab".bytes())
        );
        assert_eq!(
            incremet_pass_u8("aaz".as_bytes()),
            Vec::from_iter("aba".bytes())
        );
        assert_eq!(
            incremet_pass_u8("zzz".as_bytes()),
            Vec::from_iter("aaaa".bytes())
        );
    }

    #[test]
    fn mut_next_pass_u8_test() {
        let pass = &mut [97u8, 97u8, 97u8];
        mut_incremet_pass_u8(pass);
        assert_eq!(pass, &[97u8, 97u8, 98u8]);
    }

    #[test]
    fn first_req_u8_test() {
        assert!(check_first_requirement_u8("abc".as_bytes()), "abc");
        assert!(check_first_requirement_u8("hhhhabc".as_bytes()), "hhhabc");
        assert!(check_first_requirement_u8("abchhhh".as_bytes()), "abchhh");
        assert!(
            check_first_requirement_u8("hhhabchhh".as_bytes()),
            "hhhabchhh"
        );
    }

    #[test]
    fn second_req_u8_test() {
        assert!(check_second_requirement_u8("abc".as_bytes()));
        assert!(!check_second_requirement_u8("aibc".as_bytes()));
        assert!(!check_second_requirement_u8("aobc".as_bytes()));
        assert!(!check_second_requirement_u8("albc".as_bytes()));
    }

    #[test]
    fn third_req_u8_test() {
        assert!(!check_third_requirement_u8("abc".as_bytes()));
        assert!(!check_third_requirement_u8("abcxyzag".as_bytes()));
        assert!(!check_third_requirement_u8("abbcdefghij".as_bytes()));
        assert!(check_third_requirement_u8("abbcc".as_bytes()));
        assert!(check_third_requirement_u8(
            "acdrgbbasdfgeiccasdf".as_bytes()
        ));
    }
}

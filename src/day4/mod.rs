// --- Day 4: The Ideal Stocking Stuffer ---

#![allow(dead_code)]

use md5::Digest;
use md5::Md5;

pub fn answer() {
    println!("Day 4: The Ideal Stocking Stuffer");
    let ans = five_zeros(&String::from("ckczppom"));
    println!("answer to pt 1 is {ans}");
    let ans2 = six_zeros(&String::from("ckczppom"));
    println!("answer to pt 2 is {ans2}");
}

// Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the economically
// forward-thinking little girls and boys.

// To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes. The input to
// the MD5 hash is some secret key (your puzzle input, given below) followed by a number in decimal. To mine
// AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces
// such a hash.

// For example:

// If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts with five
// zeroes (000001dbbfa...), and it is the lowest such number to do so.
// If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting with five
// zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....

fn five_zeros(s: &str) -> u32 {
    let mut i: u32 = 0;
    while !check_five_zeros(s, &i) && i < u32::MAX {
        i += 1;
    }
    i
}

fn check_five_zeros(s: &str, n: &u32) -> bool {
    let mut hasher = Md5::new();
    hasher.update(format!("{}{}", s, n));
    let result = hasher.finalize();
    result[0] == 0 && result[1] == 0 && result[2] < 16
}

// --- Part Two ---
// Now find one that starts with six zeroes.

fn six_zeros(s: &str) -> u32 {
    let mut i: u32 = 0;
    while !check_six_zeros(s, &i) && i < u32::MAX {
        i += 1;
    }
    i
}

fn check_six_zeros(s: &str, n: &u32) -> bool {
    let mut hasher = Md5::new();
    hasher.update(format!("{}{}", s, n));
    let result = hasher.finalize();
    result[0] == 0 && result[1] == 0 && result[2] == 0
}

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

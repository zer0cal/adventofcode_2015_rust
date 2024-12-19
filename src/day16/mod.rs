// --- Day 16: Aunt Sue ---

use std::{collections::HashMap, fs};

pub fn answer() {
    let file = fs::read_to_string("day16/input.txt").unwrap();
    let mut looked_aunt = HashMap::new();
    looked_aunt.insert("children", "3");
    looked_aunt.insert("cats", "7");
    looked_aunt.insert("samoyeds", "2");
    looked_aunt.insert("pomeranians", "3");
    looked_aunt.insert("akitas", "0");
    looked_aunt.insert("vizslas", "0");
    looked_aunt.insert("goldfish", "5");
    looked_aunt.insert("trees", "3");
    looked_aunt.insert("cars", "2");
    looked_aunt.insert("perfumes", "1");
    let aunts = file.lines().map(|line| {
        let s = line
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|s| s.split(": ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        let mut hs = HashMap::new();
        s.iter().for_each(|v| {
            hs.insert(v[0], v[1]);
        });
        hs
    });
    for (i, aunt) in aunts.enumerate() {
        if aunt.iter().all(|(&key, &val)| *looked_aunt[key] == *val) {
            println!("{} {:?}", i + 1, aunt)
        }
    }
}

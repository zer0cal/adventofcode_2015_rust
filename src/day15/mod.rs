#![allow(dead_code)]
// --- Day 15: Science for Hungry People ---

use std::fs;

pub fn answer() {
    let ingridents_str = fs::read_to_string("day15/input.txt").unwrap();
    let mut ingridents = Vec::new();
    for line in ingridents_str.lines() {
        let s: Vec<&str> = line.split(";").collect();
        let ingrident = Ingrident {
            name: s[0],
            capacity: s[1].parse::<i32>().unwrap(),
            durability: s[2].parse::<i32>().unwrap(),
            flavor: s[3].parse::<i32>().unwrap(),
            texture: s[4].parse::<i32>().unwrap(),
            calories: s[5].parse::<i32>().unwrap(),
        };
        ingridents.push(ingrident);
    }

    let k = ingridents.len();
    let l = 2;
    let n = k as i32 * l;
    let c = 100 / k;
    let mut teaspoons = Vec::new();
    for ingrident in ingridents.iter() {
        teaspoons.push(Teaspoon {
            number: c as i32,
            ingrident,
        })
    }
    let mut komb = Kombinator::new(n, k as u8);
    let mut beast_score = i32::MIN;
    let mut y = true;
    while y {
        y = false;
        while let Some(v) = komb.next() {
            let mut t_ts = teaspoons.clone();
            let proportions: Vec<usize> = (0..k)
                .map(|i| v.iter().filter(|x| **x == i as u8).count())
                .collect();
            t_ts.iter_mut()
                .enumerate()
                .for_each(|(i, x)| x.number += proportions[i] as i32 - l);
            if t_ts.iter().any(|x| x.number < 0) {
                continue;
            }
            let cal = calc_cal(&t_ts);
            if cal != 500 {
                continue;
            }
            let score = calc_total_score(&t_ts);
            if beast_score < score {
                y = true;
                beast_score = score;
                println!("{}, {}", cal, score);
                teaspoons = t_ts.clone();
            }
        }
    }
    println!("{}", beast_score);
}

#[derive(Debug, Clone, Copy)]
struct Ingrident<'a> {
    name: &'a str,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

#[derive(Debug, Clone, Copy)]
struct Teaspoon<'a, 'b> {
    number: i32,
    ingrident: &'a Ingrident<'b>,
}

fn calc_total_score(teaspoons: &[Teaspoon]) -> i32 {
    let (c, d, f, t) = teaspoons
        .iter()
        .map(|x| {
            (
                x.ingrident.capacity * x.number,
                x.ingrident.durability * x.number,
                x.ingrident.flavor * x.number,
                x.ingrident.texture * x.number,
            )
        })
        .fold((0, 0, 0, 0), |acc, x| {
            (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2, acc.3 + x.3)
        });
    c * d * f * t
}

fn calc_cal(teaspoons: &[Teaspoon]) -> i32 {
    let cal = teaspoons
        .iter()
        .map(|x| x.ingrident.calories * x.number)
        .sum::<i32>();
    cal
}

struct Kombinator {
    n: i32,
    k: u8,
    v: Vec<u8>,
}

impl Kombinator {
    pub fn new(n: i32, k: u8) -> Kombinator {
        let v = vec![0; n as usize];
        Kombinator { n, k, v }
    }

    fn increment(&mut self) {
        let mut cur = 0;
        if self.v.iter().all(|&x| x == self.k - 1) {
            return;
        }
        for i in 0..self.n {
            if self.v[i as usize] == self.k - 1 {
                cur = 1;
                self.v[i as usize] = 0;
                continue;
            }
            if self.v[i as usize] + cur < self.k {
                self.v[i as usize] += 1;
                break;
            }
            self.v[i as usize] = 0;
        }
    }

    pub fn next(&mut self) -> Option<&Vec<u8>> {
        let v = self.v.clone();
        self.increment();
        if self.v != v {
            return Some(&self.v);
        }
        None
    }
}

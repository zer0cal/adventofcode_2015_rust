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
    highest_scoring_cookie(&ingridents);
    highest_scoring_cookie_with_500cal(&ingridents);
}

fn highest_scoring_cookie(ingridents: &[Ingrident]) {
    let mut teaspoons = Vec::new();
    for ingrident in ingridents.iter() {
        teaspoons.push(Teaspoon {
            count: 25,
            ingrident,
        })
    }
    let mut combinator = Combinator::new(100, ingridents.len() as u8);
    let mut max_score = calc_total_score(&teaspoons);
    while let Some(v) = combinator.next_proportion() {
        teaspoons
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| x.count = v[i] as i32);
        let score = calc_total_score(&teaspoons);
        if max_score < score {
            max_score = score;
        }
    }
    println!("Highest total score: {}", max_score);
}

fn highest_scoring_cookie_with_500cal(ingridents: &[Ingrident]) {
    let mut teaspoons = Vec::new();
    for ingrident in ingridents.iter() {
        teaspoons.push(Teaspoon {
            count: 25,
            ingrident,
        })
    }
    let mut combinator = Combinator::new(100, ingridents.len() as u8);
    let mut max_score = 0;
    while let Some(v) = combinator.next_proportion() {
        teaspoons
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| x.count = v[i] as i32);
        if calc_cal(&teaspoons) != 500 {
            continue;
        }
        let score = calc_total_score(&teaspoons);
        if max_score < score {
            max_score = score;
        }
    }
    println!("Highest total score with 500 cal: {}", max_score);
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
    count: i32,
    ingrident: &'a Ingrident<'b>,
}

fn calc_total_score(teaspoons: &[Teaspoon]) -> i32 {
    let (mut c, mut d, mut f, mut t) = teaspoons
        .iter()
        .map(|x| {
            (
                x.ingrident.capacity * x.count,
                x.ingrident.durability * x.count,
                x.ingrident.flavor * x.count,
                x.ingrident.texture * x.count,
            )
        })
        .fold((0, 0, 0, 0), |acc, x| {
            (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2, acc.3 + x.3)
        });
    if c < 0 {
        c = 0;
    }
    if d < 0 {
        d = 0;
    }
    if f < 0 {
        f = 0;
    }
    if t < 0 {
        t = 0;
    }
    c * d * f * t
}

fn calc_cal(teaspoons: &[Teaspoon]) -> i32 {
    let cal = teaspoons
        .iter()
        .map(|x| x.ingrident.calories * x.count)
        .sum::<i32>();
    cal
}

struct Combinator {
    length: i32,
    elements: u8,
    state: Vec<u8>,
}

impl Combinator {
    pub fn new(length: i32, elements: u8) -> Combinator {
        let state = vec![0; length as usize];
        Combinator {
            length,
            elements,
            state,
        }
    }

    pub fn next_combination(&mut self) -> Option<&Vec<u8>> {
        let mut cur = 0;
        if self.state.iter().all(|&x| x == self.elements - 1) {
            return None;
        }
        for i in 0..self.length {
            if self.state[i as usize] == self.elements - 1 {
                cur = 1;
                continue;
            }
            if self.state[i as usize] + cur < self.elements {
                let next_value = self.state[i as usize] + 1;
                self.state
                    .iter_mut()
                    .take(i as usize)
                    .for_each(|x| *x = next_value);
                self.state[i as usize] = next_value;
                break;
            }
        }
        Some(&self.state)
    }

    pub fn next_proportion(&mut self) -> Option<Vec<usize>> {
        let k = self.elements;
        if let Some(v) = self.next_combination() {
            let proportions: Vec<usize> = (0..k)
                .map(|i| v.iter().filter(|x| **x == i).count())
                .collect();
            return Some(proportions);
        }
        None
    }
}

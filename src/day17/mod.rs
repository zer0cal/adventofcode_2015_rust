// --- Day 17: No Such Thing as Too Much ---

use std::fs;

const EGGNOG_VOL: u32 = 150;

pub fn answer() {
    println!("Day 17: No Such Thing as Too Much");
    let file = fs::read_to_string("src/day17/input.txt").unwrap();
    let ans1 = pt1(&file);
    println!("answer to pt 1 is {}", ans1);
    let ans2 = pt2(&file);
    println!("answer to pt 2 is {}", ans2);
}

fn pt1(file: &str) -> u32 {
    let containers = get_containters(file);
    let mut seq_com = SeqCombinator::new(Box::new(containers.iter().collect()));
    let mut number_of_combinations = 0;
    while let Some(seq) = seq_com.next() {
        let sum: u32 = seq.iter().fold(0, |acc, x| acc + **x);
        if sum == EGGNOG_VOL {
            number_of_combinations += 1;
        };
    }
    number_of_combinations
}

fn pt2(file: &str) -> u32 {
    let containers = get_containters(file);
    let mut seq_com = SeqCombinator::new(Box::new(containers.iter().collect()));
    let mut lowest_count = containers.len();
    let mut numbar_of_combinations = 0;
    while let Some(seq) = seq_com.next() {
        let sum: u32 = seq.iter().fold(0, |acc, x| acc + **x);
        if sum == EGGNOG_VOL {
            match seq.len().cmp(&lowest_count) {
                std::cmp::Ordering::Less => {
                    lowest_count = seq.len();
                    numbar_of_combinations = 1;
                }
                std::cmp::Ordering::Equal => {
                    numbar_of_combinations += 1;
                }
                _ => (),
            }
        };
    }
    numbar_of_combinations
}

fn get_containters(file: &str) -> Vec<u32> {
    file.lines().flat_map(|x| x.parse()).collect()
}

type BoxVec<T> = Box<Vec<T>>;

struct SeqCombinator<T: Sized> {
    members: BoxVec<T>,
    i: usize,
    c: Vec<bool>,
}

impl<T: Sized> SeqCombinator<T> {
    pub fn new(members: BoxVec<T>) -> SeqCombinator<T> {
        let len = members.len();
        SeqCombinator {
            members,
            i: 0,
            c: vec![false; len],
        }
    }
}

type BVec<T> = Box<Vec<T>>;

impl<T: Sized> SeqCombinator<T> {
    fn next(&mut self) -> Option<BVec<&T>> {
        if self.i < 2_usize.pow(self.c.len() as u32) {
            self.inc_c();
            let c: Box<Vec<&T>> = Box::new(
                self.members
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| self.c[*i])
                    .map(|(_, x)| x)
                    .collect(),
            );
            self.i += 1;
            return Some(c);
        }
        None
    }

    fn inc_c(&mut self) {
        for b in self.c.iter_mut() {
            if *b {
                *b = false;
            } else {
                *b = true;
                break;
            }
        }
    }
}

// --- Day 13: Knights of the Dinner Table ---

use std::{collections::HashMap, fs};

pub fn answer() {
    println!("Day 13: Knights of the Dinner Table");
    let input = fs::read_to_string("day13_input.txt").expect("err reading day 13 input");
    let ans1 = optimal_happiness(&input);
    println!("answer to pt 1 is {}", ans1);
    let ans2 = optimal_happiness_pt2(&input);
    println!("answer to pt 2 is {}", ans2);
}

fn optimal_happiness(s: &str) -> i32 {
    let relations = generate_relations(s);
    let people = Vec::from_iter(relations.keys());
    let mut seq_permutator = SeqPermutator::new(Box::new(people));
    let seq = seq_permutator.next_permutation().unwrap();
    let mut max_happiness = calculate_happiness(&relations, seq.iter().map(|x| **x).collect());

    while let Some(seq) = seq_permutator.next_permutation() {
        let happinness = calculate_happiness(&relations, seq.iter().map(|x| **x).collect());
        if happinness > max_happiness {
            max_happiness = happinness;
        }
    }

    max_happiness
}

fn optimal_happiness_pt2(s: &str) -> i32 {
    let mut relations = generate_relations(s);
    let mut people = Vec::from_iter(relations.keys().cloned());
    let my_relations = HashMap::new();
    for (_, hm) in relations.iter_mut() {
        hm.insert("Me", 0);
    }
    relations.insert("Me", my_relations);
    for person in &people {
        if let Some(me) = relations.get_mut("Me") {
            me.insert(person, 0);
        }
    }
    people.push("Me");
    let mut seq_permutator = SeqPermutator::new(Box::new(people));
    let seq = seq_permutator.next_permutation().unwrap();
    let mut max_happiness = calculate_happiness(&relations, seq.iter().map(|x| *x).collect());

    while let Some(seq) = seq_permutator.next_permutation() {
        let happinness = calculate_happiness(&relations, seq.iter().map(|x| *x).collect());
        if happinness > max_happiness {
            max_happiness = happinness;
        }
    }

    max_happiness
}

struct SeqPermutator<'a, T: ?Sized> {
    seq: Box<Vec<&'a T>>,
    i: usize,
    c: Vec<usize>,
}

impl<'a, T: ?Sized> SeqPermutator<'a, T> {
    pub fn new(seq: Box<Vec<&'a T>>) -> SeqPermutator<'a, T> {
        let len = seq.len();
        SeqPermutator {
            seq,
            i: 0,
            c: vec![0; len],
        }
    }

    pub fn next_permutation(&mut self) -> Option<Box<Vec<&'a T>>> {
        if self.i == 0usize {
            self.i += 1;
            return Some(Box::clone(&self.seq));
        }
        while self.i < self.seq.len() {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    let tmp = self.seq[0];
                    self.seq[0] = self.seq[self.i];
                    self.seq[self.i] = tmp;
                } else {
                    let tmp = self.seq[self.c[self.i]];
                    self.seq[self.c[self.i]] = self.seq[self.i];
                    self.seq[self.i] = tmp;
                }
                self.c[self.i] += 1;
                self.i = 1;
                return Some(Box::clone(&self.seq));
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }
        None
    }
}

fn calculate_happiness(relations: &HashMap<&str, HashMap<&str, i32>>, mut seq: Vec<&str>) -> i32 {
    assert!(relations.len() >= 3);
    assert!(relations.len() == seq.len());

    seq.extend_from_within(..2);
    let mut happiness = 0;

    for names in seq.windows(3) {
        let left_person = names[0];
        let right_person = names[2];
        if let Some(given_relations) = relations.get(names[1]) {
            if let Some(left_relation) = given_relations.get(left_person) {
                if let Some(right_relation) = given_relations.get(right_person) {
                    happiness += left_relation;
                    happiness += right_relation;
                } else {
                    panic!("right ");
                }
            } else {
                panic!("left");
            }
        }
    }
    happiness
}

fn generate_relations(s: &str) -> HashMap<&str, HashMap<&str, i32>> {
    let mut lines = s.lines();
    let mut relations: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    while let Some(val) = lines.next() {
        let (person, other, value) = parse_line(val);
        if let Some(relation) = relations.get_mut(person) {
            relation.insert(other, value);
        } else {
            let mut relation = HashMap::new();
            relation.insert(other, value);
            relations.insert(person, relation);
        }
    }
    relations
}

fn parse_line(line: &str) -> (&str, &str, i32) {
    let mut split = line.split(" ");
    let person = split.next();
    let person = person.unwrap();
    split.next();
    let sentiment = split.next();
    let sentiment = sentiment.unwrap();
    let value = split.next();
    let value = value.unwrap();
    let value = value.parse::<i32>();
    let mut value = value.unwrap();
    if sentiment == "lose" {
        value *= -1;
    }
    let mut split = split.skip(6);
    let other = split.next();
    let other = other.unwrap();
    let other = &other[..other.len() - 1];
    (person, other, value)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::day13::{calculate_happiness, parse_line};

    #[test]
    fn calculate_happiness_test() {
        let mut people = HashMap::new();
        let mut a_relations = HashMap::new();
        let mut b_relations = HashMap::new();
        let mut c_relations = HashMap::new();
        a_relations.insert("B", 1);
        a_relations.insert("C", -1);
        people.insert("A", a_relations);
        b_relations.insert("A", -2);
        b_relations.insert("C", 0);
        people.insert("B", b_relations);
        c_relations.insert("A", 1);
        c_relations.insert("B", -1);
        people.insert("C", c_relations);
        let happiness = calculate_happiness(&people, vec!["A", "B", "C"]);
        assert!(happiness == -2);
    }

    #[test]
    fn parse_line_test() {
        let (person, other, value) =
            parse_line("Alice would gain 54 happiness units by sitting next to Bob.");
        assert!(person == "Alice");
        assert!(other == "Bob");
        assert!(value == 54);

        let (person, other, value) =
            parse_line("Mallory would gain 7 happiness units by sitting next to George.");
        assert!(person == "Mallory");
        assert!(other == "George");
        assert!(value == 7);
    }
}

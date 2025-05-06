// --- Day 19: Medicine for Rudolph ---

use std::collections::{hash_set::HashSet, HashMap};

pub fn answer() {
    println!("Day 19: Medicine for Rudolph");
    let input = std::fs::read_to_string("src/day19/input.txt").unwrap();
    let replacments = std::fs::read_to_string("src/day19/replacements.txt").unwrap();
    let ans1 = pt1(&input, &replacments);
    println!("Answer to pt1 is {}", ans1);
    let ans2 = pt2(&input, &replacments);
    println!("Answer to pt2 is {}", ans2);
}

type Atom<'a> = &'a str;

fn atom(input: &str) -> Atom<'_> {
    input
}

type Molecule<'a> = Vec<Atom<'a>>;

type Rules<'a> = HashMap<Molecule<'a>, Atom<'a>>;

fn get_atoms<'a>(input: &'a str) -> Vec<Atom<'a>> {
    let mut atoms: HashSet<Atom<'a>> = HashSet::new();
    let mut chars = input.chars().peekable();
    for i in 0..input.len() {
        match (chars.next(), chars.peek()) {
            (Some(first), Some(second))
                if first.is_ascii_uppercase() && second.is_ascii_lowercase() =>
            {
                atoms.insert(atom(&input[i..i + 2]));
            }
            (Some(first), Some(second))
                if first.is_ascii_uppercase() && second.is_ascii_uppercase() =>
            {
                atoms.insert(atom(&input[i..i + 1]));
            }
            (Some(first), Some(second))
                if first.is_ascii_lowercase() && second.is_ascii_uppercase() =>
            {
                continue;
            }
            (Some(first), None) if first.is_ascii_uppercase() => {
                atoms.insert(atom(&input[i..i + 1]));
                break;
            }
            (_, _) => break,
        }
    }
    Vec::from_iter(atoms)
}

fn get_rules(input: &str) -> Rules {
    let mut rules = HashMap::new();
    for line in input.lines() {
        let splt: Vec<&str> = line.split_whitespace().collect();
        if splt.len() < 3 {
            continue;
        }
        let mol = get_molecule(splt[2]);
        let atom = atom(splt[0]);
        rules.insert(mol, atom);
    }
    rules
}

fn get_molecule(input: &str) -> Molecule {
    let mut chars = input.chars().peekable();
    let mut molecule: Molecule = Vec::new();
    let atoms = get_atoms(input);
    loop {
        match (chars.next(), chars.peek()) {
            (Some(first), Some(second)) => {
                let str = String::from_iter([first, *second].iter());
                if let Some(atom) = atoms.iter().find(|x| ***x == str) {
                    molecule.push(atom);
                } else if let Some(atom) = atoms.iter().find(|x| ***x == first.to_string()) {
                    molecule.push(atom);
                }
            }
            (Some(first), None) => {
                if let Some(atom) = atoms.iter().find(|x| ***x == first.to_string()) {
                    molecule.push(atom);
                }
            }
            (_, _) => break,
        }
    }
    molecule
}

fn expand_molecule<'a>(
    rule: (&Molecule<'a>, &Atom<'a>),
    molecule: &Molecule<'a>,
) -> Vec<Molecule<'a>> {
    let mut molecules: Vec<Molecule<'_>> = Vec::new();
    let (m, a) = rule;
    for i in 0..molecule.len() {
        if molecule[i] != *a {
            continue;
        }
        let (head, tail) = molecule.split_at(i);
        if let Some((_, tail)) = tail.split_first() {
            molecules.push([head, m, tail].concat());
        } else if tail.last().is_some() {
            molecules.push([head, m].concat());
        } else {
            molecules.push([head].concat());
        }
    }
    molecules
}

fn reduce_molecule<'a>(
    rule: (&Molecule<'a>, &Atom<'a>),
    molecule: &Molecule<'a>,
) -> Vec<Molecule<'a>> {
    let mut molecules: Vec<Molecule<'_>> = Vec::new();
    let (m, a) = rule;
    if m.len() > molecule.len() {
        return molecules;
    }
    for mol_index in (0..(molecule.len() - m.len() + 1)).rev() {
        if molecule[mol_index..mol_index + m.len()].to_vec() == *m {
            let (head, rest) = molecule.split_at(mol_index);
            let (_, tail) = rest.split_at(m.len());
            molecules.push([head, &[a], tail].concat());
        }
    }
    molecules
}

fn get_steps_by_reduce(rules: &Rules, molecule: &Molecule) -> Option<usize> {
    let e = Molecule::from_iter([atom("e")]);
    let reduced_molecules: Vec<Molecule> = rules
        .iter()
        .flat_map(|rule| reduce_molecule((rule.0, rule.1), molecule))
        .collect();
    let mut reduced_molecules: Vec<Molecule> = reduced_molecules.into_iter().collect();
    reduced_molecules.sort_unstable_by_key(|x| {
        x.len()
            + x.windows(2)
                .filter_map(|window| {
                    if window[0] == window[1] {
                        Some(())
                    } else {
                        None
                    }
                })
                .count()
    });
    for reduced in reduced_molecules.iter() {
        if *reduced == e {
            println!("::: 1. {}", mol_to_str(reduced));
            return Some(1);
        }
        if reduced.contains(e.first()?) {
            continue;
        }
        if let Some(steps) = get_steps_by_reduce(rules, reduced) {
            println!("    {}. {}", steps + 1, mol_to_str(reduced));
            return Some(steps + 1);
        }
    }
    None
}

fn mol_to_str(molecule: &Molecule) -> String {
    let mut s = String::new();
    for &atom in molecule {
        s.push_str(atom);
    }
    s
}

fn pt1(input: &str, replacments: &str) -> usize {
    let rules = get_rules(replacments);
    let molecule = get_molecule(input);
    let molecules: HashSet<Molecule> = rules
        .iter()
        .flat_map(|rule| expand_molecule((rule.0, rule.1), &molecule))
        .collect();
    molecules.len()
}

fn pt2(input: &str, replacments: &str) -> usize {
    let rules = get_rules(replacments);
    let looked_molecule = get_molecule(input);
    let steps = get_steps_by_reduce(&rules, &looked_molecule);
    steps.unwrap()
}

#[cfg(test)]
mod tests {

    use crate::day19::{
        atom, expand_molecule, get_atoms, get_molecule, get_rules, get_steps_by_reduce, Atom, Rules,
    };

    #[test]
    fn get_atoms_test() {
        let input = "HHeHOHFeF";
        let mut tested = get_atoms(input);
        let mut expected: Vec<Atom> =
            Vec::from([atom("H"), atom("He"), atom("O"), atom("Fe"), atom("F")]);
        tested.sort();
        expected.sort();
        assert_eq!(expected, tested);
    }

    #[test]
    fn get_rules_test() {
        let replacments = "H => HO\n\
        H => OH\n\
        O => HH";
        let tested = get_rules(replacments);
        let mut expected = Rules::new();
        expected.insert([atom("H"), atom("O")].to_vec(), atom("H"));
        expected.insert([atom("O"), atom("H")].to_vec(), atom("H"));
        expected.insert([atom("H"), atom("H")].to_vec(), atom("O"));
        assert_eq!(expected, tested);
    }

    #[test]
    fn get_rules_with_e_test() {
        let replacments = "H => HO\n\
        H => OH\n\
        e => H\n\
        e => O\n\
        O => HH";
        let tested = get_rules(replacments);
        let mut expected = Rules::new();
        expected.insert([atom("H"), atom("O")].to_vec(), atom("H"));
        expected.insert([atom("O"), atom("H")].to_vec(), atom("H"));
        expected.insert([atom("H")].to_vec(), atom("e"));
        expected.insert([atom("O")].to_vec(), atom("e"));
        expected.insert([atom("H"), atom("H")].to_vec(), atom("O"));
        assert_eq!(expected, tested);
    }

    #[test]
    fn get_molecule_test() {
        let input = "HOHeHO";
        let expected = [atom("H"), atom("O"), atom("He"), atom("H"), atom("O")].to_vec();
        let tested = get_molecule(input);
        assert_eq!(expected, tested);
    }

    #[test]
    fn generate_all_posible_molecules_test() {
        let replacments = "H => HO";
        let input = "HOH";
        let rules = get_rules(replacments);
        let molecule = get_molecule(input);
        let tested = expand_molecule(
            rules
                .get_key_value([atom("H"), atom("O")].to_vec().as_slice())
                .unwrap(),
            &molecule,
        );
        let expected = Vec::from([
            [atom("H"), atom("O"), atom("O"), atom("H")].to_vec(),
            [atom("H"), atom("O"), atom("H"), atom("O")].to_vec(),
        ]);
        assert_eq!(expected, tested);
    }

    #[test]
    fn get_steps1_test() {
        let replacments = "H => HO\n\
            H => OH\n\
            e => H\n\
            e => O\n\
            O => HH";
        let input = "HOH";
        let rules = get_rules(replacments);
        let molecule = get_molecule(input);
        let steps = get_steps_by_reduce(&rules, &molecule);
        assert_eq!(3usize, steps.unwrap());
        let input = "HOHOHO";
        let molecule = get_molecule(input);
        let steps = get_steps_by_reduce(&rules, &molecule);
        assert_eq!(6usize, steps.unwrap());
        let input = "OOHOOHHO";
        let molecule = get_molecule(input);
        let steps = get_steps_by_reduce(&rules, &molecule);
        assert_eq!(8usize, steps.unwrap());
    }
}

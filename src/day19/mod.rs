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

fn get_atoms<'a>(input: &'a str) -> HashSet<Atom<'a>> {
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
    atoms
}

fn get_rules<'a>(input: &'a str, atoms: &HashSet<Atom<'a>>) -> Rules<'a> {
    let mut rules = HashMap::new();
    for line in input.lines() {
        let splt: Vec<&str> = line.split_whitespace().collect();
        let mol = get_molecule(splt[2], atoms);
        let atom = atom(splt[0]);
        rules.insert(mol, atom);
    }
    rules
}

fn get_molecule<'a>(input: &'a str, atoms: &HashSet<Atom<'a>>) -> Molecule<'a> {
    let mut chars = input.chars().peekable();
    let mut molecule: Molecule = Vec::new();
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
) -> HashSet<Molecule<'a>> {
    let mut molecules: HashSet<Molecule<'_>> = HashSet::new();
    let (m, a) = rule;
    for i in 0..molecule.len() {
        if molecule[i] != *a {
            continue;
        }
        let (head, tail) = molecule.split_at(i);
        if let Some((_, tail)) = tail.split_first() {
            molecules.insert([head, m, tail].concat());
        } else if tail.last().is_some() {
            molecules.insert([head, m].concat());
        } else {
            molecules.insert([head].concat());
        }
    }
    molecules
}

fn reduce_molecule<'a>(
    rule: (&Molecule<'a>, &Atom<'a>),
    molecule: &Molecule<'a>,
) -> HashSet<Molecule<'a>> {
    let mut molecules: HashSet<Molecule<'_>> = HashSet::new();
    let (m, a) = rule;
    if m.len() > molecule.len() {
        return molecules;
    }
    for mol_index in 0..(molecule.len() - m.len() + 1) {
        if molecule[mol_index..mol_index + m.len()].to_vec() == *m {
            let (head, rest) = molecule.split_at(mol_index);
            let (_, tail) = rest.split_at(m.len());
            molecules.insert([head, &[a], tail].concat());
        }
    }
    molecules
}

fn get_steps(rules: &Rules, molecule: &Molecule) -> Option<usize> {
    let e = Molecule::from_iter([atom("e")]);
    let reduced_molecules: HashSet<Molecule> = rules
        .iter()
        .flat_map(|rule| reduce_molecule((rule.0, rule.1), molecule))
        .collect();
    let mut reduced_molecules: Vec<Molecule> = reduced_molecules.into_iter().collect();
    reduced_molecules.sort_unstable_by_key(|x| posible_reductions(rules, x));
    for reduced in reduced_molecules.iter().rev() {
        if *reduced == e {
            return Some(1);
        }
        if reduced.contains(&atom("e")) {
            continue;
        }
        if let Some(steps) = get_steps(rules, reduced) {
            return Some(steps + 1);
        }
    }
    None
}

fn posible_reductions(rules: &HashMap<Molecule, Atom>, molecule: &Molecule) -> usize {
    let mut posibilities = 0usize;
    for key in rules.keys() {
        molecule[..].windows(key.len()).for_each(|w| {
            if key == w {
                posibilities += 1
            }
        });
    }
    posibilities
}

fn pt1(input: &str, replacments: &str) -> usize {
    let atoms = get_atoms(input);
    let rules = get_rules(replacments, &atoms);
    let molecule = get_molecule(input, &atoms);
    let molecules: HashSet<Molecule> = rules
        .iter()
        .flat_map(|rule| expand_molecule((rule.0, rule.1), &molecule))
        .collect();
    molecules.len()
}

fn pt2(input: &str, replacments: &str) -> usize {
    let atoms = get_atoms(input);
    let rules = get_rules(replacments, &atoms);
    let looked_molecule = get_molecule(input, &atoms);
    let steps = get_steps(&rules, &looked_molecule);
    steps.unwrap()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day19::{
        atom, expand_molecule, get_atoms, get_molecule, get_rules, get_steps, reduce_molecule,
        Atom, Rules,
    };

    #[test]
    fn get_atoms_test() {
        let input = "HHeHOHFeF";
        let tested = get_atoms(input);
        let expected: HashSet<Atom> =
            HashSet::from([atom("H"), atom("He"), atom("O"), atom("Fe"), atom("F")]);
        assert_eq!(expected, tested);
    }

    #[test]
    fn get_rules_test() {
        let replacments = "H => HO\n\
        H => OH\n\
        O => HH";
        let input = "HO";
        let atoms = get_atoms(input);
        let tested = get_rules(replacments, &atoms);
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
        let input = "HO";
        let atoms = get_atoms(input);
        let tested = get_rules(replacments, &atoms);
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
        let atoms = get_atoms(input);
        let expected = [atom("H"), atom("O"), atom("He"), atom("H"), atom("O")].to_vec();
        let tested = get_molecule(input, &atoms);
        assert_eq!(expected, tested);
    }

    #[test]
    fn generate_all_posible_molecules_test() {
        let replacments = "H => HO\n\
        H => OH\n\
        O => HH";
        let input = "HOH";
        let atoms = get_atoms(input);
        let rules = get_rules(replacments, &atoms);
        let molecule = get_molecule(input, &atoms);
        let tested = expand_molecule(
            rules
                .get_key_value([atom("H"), atom("O")].as_slice())
                .unwrap(),
            &molecule,
        );
        let expected = HashSet::from([
            [atom("H"), atom("O"), atom("O"), atom("H")].to_vec(),
            [atom("H"), atom("O"), atom("H"), atom("O")].to_vec(),
        ]);
        assert_eq!(expected, tested);
    }

    #[test]
    fn get_all_reduced_molecules_test() {
        let replacments = "H => HO\n\
        H => OH\n\
        H => OHe\n\
        He => OHe\n\
        O => HH";
        let input = "HOHO";
        let atoms = get_atoms(input);
        let rules = get_rules(replacments, &atoms);
        let molecule = get_molecule(input, &atoms);
        let reduced_molecules = reduce_molecule(
            rules
                .get_key_value([atom("H"), atom("O")].as_slice())
                .unwrap(),
            &molecule,
        );
        let expected = HashSet::from([
            [atom("H"), atom("H"), atom("O")].to_vec(),
            [atom("H"), atom("O"), atom("H")].to_vec(),
        ]);
        assert_eq!(expected, reduced_molecules);
    }

    #[test]
    fn get_steps_test() {
        let replacments = "H => HO\n\
        H => OH\n\
        e => H\n\
        e => O\n\
        O => HH";
        let input = "HOH";
        let atoms = get_atoms(input);
        let rules = get_rules(replacments, &atoms);
        let molecule = get_molecule(input, &atoms);
        let steps = get_steps(&rules, &molecule);
        assert_eq!(3usize, steps.unwrap());
        let input = "HOHOHO";
        let molecule = get_molecule(input, &atoms);
        let steps = get_steps(&rules, &molecule);
        assert_eq!(6usize, steps.unwrap());
        let input = "OOHOHOHOHHOOHHOHO";
        let molecule = get_molecule(input, &atoms);
        let steps = get_steps(&rules, &molecule);
        assert_eq!(17usize, steps.unwrap());
    }
}

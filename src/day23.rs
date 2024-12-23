use core::str;
use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    str::FromStr,
};

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

pub fn day23part1(input: &str) -> usize {
    let edges = parse_edges(input);
    let mut vertices: HashMap<ComputerName, Vec<ComputerName>> = HashMap::new();

    for (a, b) in edges {
        let a_neighbours = vertices.entry(a).or_default();
        a_neighbours.push(b);
        a_neighbours.sort();
        let b_neighbours = vertices.entry(b).or_default();
        b_neighbours.push(a);
        b_neighbours.sort();
    }

    // find the 3-loops where one starts with t
    let mut triplets = HashSet::new();
    for (a, a_neighbours) in &vertices {
        if !a.starts_with('t') {
            continue;
        }
        for b in a_neighbours {
            if let Some(b_neighbours) = vertices.get(b) {
                for c in b_neighbours {
                    if c != a && vertices.get(c).unwrap().contains(a) {
                        // a <-> b <-> c <-> a
                        let mut triplet = [*a, *b, *c];
                        triplet.sort();
                        triplets.insert(triplet);
                    }
                }
            }
        }
    }

    triplets.len()
}

pub fn day23part2(input: &str) -> String {
    let edges = parse_edges(input);
    let mut vertices: HashMap<ComputerName, Vec<ComputerName>> = Default::default();

    for &(a, b) in &edges {
        let a_neighbours = vertices.entry(a).or_default();
        a_neighbours.push(b);
        a_neighbours.sort();
        let b_neighbours = vertices.entry(b).or_default();
        b_neighbours.push(a);
        b_neighbours.sort();
    }

    let mut sets = edges
        .iter()
        .map(|&(a, b)| if b > a { vec![a, b] } else { vec![b, a] })
        .collect_vec();

    loop {
        sets.sort();
        sets.dedup();

        // try to add each vertex to each set in turn, if it's connected to all members
        let mut new_sets = vec![];
        for (a, a_neighbours) in &vertices {
            for set in &sets {
                // are all members of the set my neighbours?
                if sorted_superset(a_neighbours, set) {
                    let mut new_set = set.clone();
                    new_set.push(*a);
                    new_set.sort();
                    new_sets.push(new_set);
                }
            }
        }
        if new_sets.is_empty() {
            // cannot be improved
            break;
        } else {
            sets = new_sets;
        }
    }

    // implied by the question: there is only one answer
    assert_eq!(sets.len(), 1);

    sets[0].iter().map(|n| format!("{}", n)).join(",")
}

fn sorted_superset<T: Ord>(greater: &[T], lesser: &[T]) -> bool {
    let mut it1 = greater.iter();
    'outer: for item in lesser {
        'inner: for other in it1.by_ref() {
            match other.cmp(item) {
                Ordering::Equal => continue 'outer,
                Ordering::Less => continue 'inner,
                Ordering::Greater => return false,
            }
        }
        return false;
    }
    true
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ComputerName {
    name: [u8; 2],
}

impl FromStr for ComputerName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            Err(())
        } else {
            let bytes = s.as_bytes();
            Ok(Self {
                name: [bytes[0], bytes[1]],
            })
        }
    }
}

impl ComputerName {
    pub fn starts_with(self, c: char) -> bool {
        self.name[0] as char == c
    }
}

impl Display for ComputerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = str::from_utf8(&self.name).unwrap();
        Display::fmt(s, f)
    }
}

impl Debug for ComputerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self)
    }
}

fn parse_edges(input: &str) -> Vec<(ComputerName, ComputerName)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (n1, n2) = l.split_once('-').unwrap();
            (n1.parse().unwrap(), n2.parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "\
        kh-tc\n\
        qp-kh\n\
        de-cg\n\
        ka-co\n\
        yn-aq\n\
        qp-ub\n\
        cg-tb\n\
        vc-aq\n\
        tb-ka\n\
        wh-tc\n\
        yn-cg\n\
        kh-ub\n\
        ta-co\n\
        de-co\n\
        tc-td\n\
        tb-wq\n\
        wh-td\n\
        ta-ka\n\
        td-qp\n\
        aq-cg\n\
        wq-ub\n\
        ub-vc\n\
        de-ta\n\
        wq-aq\n\
        wq-vc\n\
        wh-yn\n\
        ka-de\n\
        kh-ta\n\
        co-tc\n\
        wh-qp\n\
        tb-vc\n\
        td-yn\n\
    ";

    #[test]
    fn part1test() {
        assert_eq!(day23part1(TEST_INPUT), 7);
    }

    #[test]
    fn part2test() {
        assert_eq!(&day23part2(TEST_INPUT), "co,de,ka,ta");
    }

    #[test]
    fn superset_test() {
        assert!(sorted_superset(&[1, 2, 3, 4], &[2, 4]));
        assert!(!sorted_superset(&[1, 2, 3, 4], &[1, 5]));
        assert!(!sorted_superset(&[1, 2, 4], &[2, 3]));
    }
}

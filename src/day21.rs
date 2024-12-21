use std::collections::BinaryHeap;

use hashbrown::HashMap;
use itertools::Itertools;

pub fn day21part1(input: &str) -> usize {
    let mut complexity = 0;

    let codes = input.trim().lines();

    for code in codes {
        let numeric_part: usize = code[..code.len() - 1].parse().unwrap();

        let seq = find_optimal_sequence(code, 2);

        complexity += seq.len() * numeric_part;
    }

    complexity
}

fn find_optimal_sequence(code: &str, n_robots: usize) -> String {
    let keypads = std::iter::repeat_n(Keypad::directional(), n_robots)
        .chain([Keypad::numeric()])
        .collect_vec();

    let queues = std::iter::repeat_n(String::new(), n_robots + 1)
        .chain([code.to_string()])
        .collect_vec();
    let positions = keypads.iter().map(|kp| kp.starting_pos()).collect_vec();
    let seed = (vec![0; keypads.len()], queues, positions);

    let mut prio_queue = BinaryHeap::new();
    prio_queue.push(seed);

    'pq: while let Some((costs, mut queues, mut positions)) = prio_queue.pop() {
        // println!("{}", queues.join("\t"));
        for i in 0..keypads.len() {
            let q = &mut queues[i + 1];
            if !q.is_empty() {
                let c = q.remove(0);
                let kp = &keypads[i];
                let pos = positions[i];
                let (seqs, new_pos) = kp.get_sequences(pos, c);
                positions[i] = new_pos;
                for seq in seqs {
                    let mut costs2 = costs.clone();
                    costs2[i] -= seq.len() as i32;
                    let mut queues2 = queues.clone();
                    queues2[i].push_str(&seq);
                    let positions2 = positions.clone();
                    prio_queue.push((costs2, queues2, positions2));
                }
                if prio_queue.len() > 10000 {
                    panic!("queue too long");
                }
                continue 'pq;
            }
        }
        // nothing in any of the queues, i.e. the input has been handled
        return std::mem::take(&mut queues[0]);
    }

    panic!()
}

#[derive(Debug, Clone)]
struct Keypad {
    keys: HashMap<char, (i32, i32)>,
    starting_pos: (i32, i32),
    blank: (i32, i32),
}

impl Keypad {
    pub fn numeric() -> Self {
        let keys = [
            ('0', (1, 0)),
            ('A', (2, 0)),
            ('1', (0, 1)),
            ('2', (1, 1)),
            ('3', (2, 1)),
            ('4', (0, 2)),
            ('5', (1, 2)),
            ('6', (2, 2)),
            ('7', (0, 3)),
            ('8', (1, 3)),
            ('9', (2, 3)),
        ]
        .into_iter()
        .collect();
        let starting_pos = (2, 0);
        let blank = (0, 0);

        Self {
            keys,
            starting_pos,
            blank,
        }
    }

    pub fn directional() -> Self {
        let keys = [
            ('<', (0, 0)),
            ('v', (1, 0)),
            ('>', (2, 0)),
            ('^', (1, 1)),
            ('A', (2, 1)),
        ]
        .into_iter()
        .collect();
        let starting_pos = (2, 1);
        let blank = (0, 1);

        Self {
            keys,
            starting_pos,
            blank,
        }
    }

    pub fn starting_pos(&self) -> (i32, i32) {
        self.starting_pos
    }

    pub fn get_sequences(&self, from: (i32, i32), key: char) -> (Vec<String>, (i32, i32)) {
        let dest = *self.keys.get(&key).unwrap();
        let delta = (dest.0 - from.0, dest.1 - from.1);

        let mut paths = vec![];

        if delta.0 >= 0 && delta.1 >= 0 {
            paths.push(
                (0..delta.0)
                    .map(|_| '>')
                    .chain((0..delta.1).map(|_| '^'))
                    .chain(['A'])
                    .collect(),
            );
            if delta.0 != 0 && delta.1 != 0 && (self.blank.1 != dest.1 || self.blank.0 != from.0) {
                paths.push(
                    (0..delta.1)
                        .map(|_| '^')
                        .chain((0..delta.0).map(|_| '>'))
                        .chain(['A'])
                        .collect(),
                );
            }
        } else if delta.0 >= 0 && delta.1 < 0 {
            paths.push(
                (0..delta.0)
                    .map(|_| '>')
                    .chain((0..(-delta.1)).map(|_| 'v'))
                    .chain(['A'])
                    .collect(),
            );
            if delta.0 != 0 && (self.blank.1 != dest.1 || self.blank.0 != from.0) {
                paths.push(
                    (0..(-delta.1))
                        .map(|_| 'v')
                        .chain((0..delta.0).map(|_| '>'))
                        .chain(['A'])
                        .collect(),
                );
            }
        } else if delta.0 < 0 && delta.1 >= 0 {
            if delta.1 != 0 && (self.blank.1 != from.1 || self.blank.0 != dest.0) {
                paths.push(
                    (0..(-delta.0))
                        .map(|_| '<')
                        .chain((0..delta.1).map(|_| '^'))
                        .chain(['A'])
                        .collect(),
                );
            }
            paths.push(
                (0..delta.1)
                    .map(|_| '^')
                    .chain((0..(-delta.0)).map(|_| '<'))
                    .chain(['A'])
                    .collect(),
            );
        } else if delta.0 < 0 || delta.1 < 0 {
            if self.blank.1 != from.1 || self.blank.0 != dest.0 {
                paths.push(
                    (0..(-delta.0))
                        .map(|_| '<')
                        .chain((0..(-delta.1)).map(|_| 'v'))
                        .chain(['A'])
                        .collect(),
                );
            }
            paths.push(
                (0..(-delta.1))
                    .map(|_| 'v')
                    .chain((0..(-delta.0)).map(|_| '<'))
                    .chain(['A'])
                    .collect(),
            );
        }

        (paths, dest)
    }

    pub fn replay(&self, moves: &str) -> Option<String> {
        let keys_by_pos: HashMap<_, _> = self.keys.iter().map(|(&k, &v)| (v, k)).collect();

        let mut output = String::new();
        let mut pos = self.starting_pos;

        for m in moves.chars() {
            if m == 'A' {
                output.push(*keys_by_pos.get(&pos)?);
            } else {
                let unit_vec = match m {
                    '>' => (1, 0),
                    '<' => (-1, 0),
                    '^' => (0, 1),
                    'v' => (0, -1),
                    _ => return None,
                };
                pos = (pos.0 + unit_vec.0, pos.1 + unit_vec.1);
                if !keys_by_pos.contains_key(&pos) {
                    return None;
                }
            }
        }
        Some(output)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "
        029A\n\
        980A\n\
        179A\n\
        456A\n\
        379A\n\
    ";

    #[test]
    fn part1test() {
        assert_eq!(day21part1(TEST_INPUT), 126384);
    }

    #[test]
    fn get_sequences_test() {
        let numpad = Keypad::numeric();
        let dpad = Keypad::directional();

        let (paths_to_3, _pos_3) = numpad.get_sequences(numpad.starting_pos(), '3');
        assert_eq!(&paths_to_3, &["^A".to_string()]);
        let (mut paths_to_2, _pos_2) = numpad.get_sequences(numpad.starting_pos(), '2');
        paths_to_2.sort();
        assert_eq!(&paths_to_2, &["<^A".to_string(), "^<A".to_string()]);
        let (paths_to_1, pos_1) = numpad.get_sequences(numpad.starting_pos(), '1');
        assert_eq!(&paths_to_1, &["^<<A".to_string()]);
        let (mut paths_1_to_9, _pos_9) = numpad.get_sequences(pos_1, '9');
        paths_1_to_9.sort();
        assert_eq!(&paths_1_to_9, &[">>^^A".to_string(), "^^>>A".to_string()]);

        let (paths_to_left, _pos_left) = dpad.get_sequences(dpad.starting_pos, '<');
        assert_eq!(&paths_to_left, &["v<<A".to_string()]);
    }

    #[test]
    fn part_1_examples() {
        let examples = [
            (
                "029A",
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
            ),
            (
                "980A",
                "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A",
            ),
            (
                "179A",
                "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            ),
            (
                "456A",
                "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A",
            ),
            (
                "379A",
                "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            ),
        ];

        for (code, ref_seq) in examples {
            let seq = find_optimal_sequence(code, 2);
            assert_eq!(seq.len(), ref_seq.len());
        }
    }
}

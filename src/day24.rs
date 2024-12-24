use core::str;
use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    str::FromStr,
};

use itertools::Itertools;

pub fn day24part1(input: &str) -> u64 {
    let (inputs, rules) = parse_input(input);

    let (x, y) = deserialize_inputs(&inputs);

    elf_add(&rules, x, y)
}

pub fn day24part2(input: &str) -> String {
    let (_, mut rules) = parse_input(input);

    let mut swapped = vec![];
    let mut frozen = vec![];

    for i in 0..47 {
        while let Some((a, b)) = fix_bit(&rules, i, &mut frozen) {
            swap_rules(&mut rules, a, b);
            swapped.push((a, b));
        }
    }

    let mut affected_outputs = vec![];
    for (a, b) in swapped {
        affected_outputs.push(a);
        affected_outputs.push(b);
    }
    affected_outputs.sort();

    affected_outputs.iter().map(|n| format!("{}", n)).join(",")
}

fn fix_bit(rules: &[Rule], bit: u64, frozen: &mut Vec<Node>) -> Option<(Node, Node)> {
    let n_possible_nodes = Node::max().idx() + 1;

    // figure out what influences the bit we're trying to fix

    let mut direct_orgin_map = vec![vec![]; n_possible_nodes];
    let mut rules_by_output = vec![vec![]; n_possible_nodes];
    for (rule_idx, rule) in rules.iter().enumerate() {
        rules_by_output[rule.output.idx()].push(rule_idx);
        direct_orgin_map[rule.output.idx()].push(rule.inputs.0);
        direct_orgin_map[rule.output.idx()].push(rule.inputs.1);
        direct_orgin_map[rule.output.idx()].sort();
        direct_orgin_map[rule.output.idx()].dedup();
    }

    if direct_orgin_map[Node::z(bit as u16).idx()].is_empty() {
        return None;
    }

    let mut origins = vec![];
    let mut relevant_rules = vec![];
    {
        let mut node_queue = VecDeque::new();
        node_queue.push_back(Node::z(bit as u16));
        while let Some(node) = node_queue.pop_front() {
            let new_origins = &direct_orgin_map[node.idx()];
            node_queue.extend(new_origins.iter().copied());
            origins.extend_from_slice(new_origins);
        }
        origins.sort();
        origins.dedup();

        let mut rule_queue = VecDeque::new();
        rule_queue.push_back(Node::z(bit as u16));
        while let Some(output) = rule_queue.pop_front() {
            let rule_indices = &rules_by_output[output.idx()];
            for &rule_idx in rule_indices {
                let rule = rules[rule_idx];
                rule_queue.push_back(rule.inputs.0);
                rule_queue.push_back(rule.inputs.1);
            }
            relevant_rules.extend_from_slice(rule_indices);
        }
    }

    if test_bit_rules(rules, bit) {
        frozen.extend(relevant_rules.iter().map(|&i| rules[i].output));
        frozen.sort();
        return None;
    }

    let expected_x_bits = (0..=bit).map(|i| Node::x(i as u16)).collect_vec();
    let expected_y_bits = (0..=bit).map(|i| Node::y(i as u16)).collect_vec();
    let illegal_x_bits = ((bit + 1)..=45).map(|i| Node::x(i as u16)).collect_vec();
    let illegal_y_bits = ((bit + 1)..=45).map(|i| Node::y(i as u16)).collect_vec();

    let mut input_rule_map: Vec<Vec<&Rule>> = vec![];
    input_rule_map.resize_with(n_possible_nodes, Default::default);
    for rule in rules {
        input_rule_map[rule.inputs.0.idx()].push(rule);
        input_rule_map[rule.inputs.1.idx()].push(rule);
    }

    let nodes_with_plausible_inputs = {
        let mut inputs_are_right = vec![false; n_possible_nodes];
        let mut queue = VecDeque::new();
        queue.extend(expected_x_bits.iter().copied());
        queue.extend(expected_y_bits.iter().copied());
        while let Some(node) = queue.pop_front() {
            if frozen.binary_search(&node).is_err() {
                if !node.is_x() && !node.is_y() {
                    inputs_are_right[node.idx()] = true;
                }
                for rule in &input_rule_map[node.idx()] {
                    if !queue.contains(&rule.output) {
                        queue.push_back(rule.output);
                    }
                }
            }
        }

        queue.extend(illegal_x_bits.iter().copied());
        queue.extend(illegal_y_bits.iter().copied());
        while let Some(node) = queue.pop_front() {
            if frozen.binary_search(&node).is_err() {
                if !node.is_x() && !node.is_y() {
                    inputs_are_right[node.idx()] = false;
                }
                for rule in &input_rule_map[node.idx()] {
                    if !queue.contains(&rule.output) {
                        queue.push_back(rule.output);
                    }
                }
            }
        }

        inputs_are_right
            .iter()
            .enumerate()
            .filter_map(|(i, is_usable)| {
                if *is_usable {
                    Some(Node(i as u16))
                } else {
                    None
                }
            })
            .collect_vec()
    };

    let nodes_leading_to_output = relevant_rules
        .iter()
        .map(|r| rules[*r].output)
        .collect_vec();

    for &node1 in &nodes_with_plausible_inputs {
        for &node2 in &nodes_leading_to_output {
            if node1 != node2 {
                // Try swapping!
                let mut new_rules = rules.to_vec();
                swap_rules(&mut new_rules, node1, node2);
                if test_bit_rules(&new_rules, bit) {
                    return Some((node1, node2));
                }
            }
        }
    }

    panic!("Failed to find solution");
}

fn swap_rules(rules: &mut [Rule], a: Node, b: Node) {
    for r in rules {
        if r.output == a {
            r.output = b;
        } else if r.output == b {
            r.output = a;
        }
    }
}

fn test_bit_rules(rules: &[Rule], bit: u64) -> bool {
    let mut assertions = vec![];
    if bit <= 44 {
        assertions.push((1 << bit, 1 << bit, false));
        assertions.push((1 << bit, 0 << bit, true));
        assertions.push((0 << bit, 1 << bit, true));
        assertions.push((0 << bit, 0 << bit, false));
        if bit > 0 {
            assertions.push((0b01 << (bit - 1), 0b00 << (bit - 1), false));
            assertions.push((0b00 << (bit - 1), 0b01 << (bit - 1), false));
            assertions.push((0b01 << (bit - 1), 0b01 << (bit - 1), true));
            assertions.push((0b10 << (bit - 1), 0b01 << (bit - 1), true));
            assertions.push((0b01 << (bit - 1), 0b10 << (bit - 1), true));
            assertions.push((0b11 << (bit - 1), 0b00 << (bit - 1), true));
            assertions.push((0b00 << (bit - 1), 0b11 << (bit - 1), true));
            assertions.push((0b11 << (bit - 1), 0b01 << (bit - 1), false));
            assertions.push((0b01 << (bit - 1), 0b11 << (bit - 1), false));
            assertions.push((0b11 << (bit - 1), 0b10 << (bit - 1), false));
            assertions.push((0b10 << (bit - 1), 0b11 << (bit - 1), false));
            assertions.push((0b11 << (bit - 1), 0b11 << (bit - 1), true));
        }
    } else {
        assertions.push((0b01 << (bit - 1), 0b00 << (bit - 1), false));
        assertions.push((0b00 << (bit - 1), 0b01 << (bit - 1), false));
        assertions.push((0b01 << (bit - 1), 0b01 << (bit - 1), true));
    }

    for (x, y, ans) in assertions {
        let z = elf_add(rules, x, y);
        let bit_set = (z & (1 << bit)) != 0;
        if ans != bit_set {
            return false;
        }
    }

    true
}

/// Get x and y from inputs
fn deserialize_inputs(inputs: &[(Node, bool)]) -> (u64, u64) {
    let mut x = 0;
    let mut y = 0;
    // let min_x = 33 * 1296;
    let min_y = 34 * 1296;
    let mut x_bit = 0;
    let mut y_bit = 0;

    for (n, val) in inputs {
        if n.0 >= min_y {
            y |= (*val as u64) << y_bit;
            y_bit += 1;
        } else {
            x |= (*val as u64) << x_bit;
            x_bit += 1;
        }
    }

    (x, y)
}

fn elf_add(rules: &[Rule], x: u64, y: u64) -> u64 {
    let n_possible_nodes = Node::max().idx() + 1;

    let mut input_rule_map: Vec<Vec<&Rule>> = vec![];
    input_rule_map.resize_with(n_possible_nodes, Default::default);
    for rule in rules {
        input_rule_map[rule.inputs.0.idx()].push(rule);
        input_rule_map[rule.inputs.1.idx()].push(rule);
    }

    let mut vals = vec![None; n_possible_nodes];
    let mut queue = VecDeque::new();

    // input the values
    for bit in 0..45 {
        let val = x & (1 << bit) != 0;
        let node = Node::x(bit);
        queue.push_back((node, val));
    }
    for bit in 0..45 {
        let val = y & (1 << bit) != 0;
        let node = Node::y(bit);
        queue.push_back((node, val));
    }

    while let Some((node, val)) = queue.pop_front() {
        if vals[node.idx()].is_none() {
            vals[node.idx()] = Some(val);
            for rule in &input_rule_map[node.idx()] {
                if let (Some(lhs), Some(rhs), None) = (
                    vals[rule.inputs.0.idx()],
                    vals[rule.inputs.1.idx()],
                    vals[rule.output.idx()],
                ) {
                    queue.push_back((rule.output, rule.operation.apply(lhs, rhs)));
                }
            }
        }
    }

    // Get the z's
    let first_z = 35 * 1296;
    let mut z = 0;
    for val in vals[first_z..].iter().rev() {
        if let &Some(bit) = val {
            z <<= 1;
            z |= bit as u64;
        }
    }

    z
}

fn parse_input(input: &str) -> (Vec<(Node, bool)>, Vec<Rule>) {
    let (part1, part2) = input.trim().split_once("\n\n").unwrap();

    let inputs = part1
        .lines()
        .map(|line| {
            let (node_name, val_str) = line.split_once(": ").unwrap();
            let node = node_name.parse().unwrap();
            let val = val_str == "1";
            (node, val)
        })
        .collect();

    let rules = part2
        .lines()
        .map(|line| {
            let bits = line.split_whitespace().collect_vec();
            let inputs = (bits[0].parse().unwrap(), bits[2].parse().unwrap());
            let operation = bits[1].parse().unwrap();
            let output = bits[4].parse().unwrap();
            Rule {
                inputs,
                output,
                operation,
            }
        })
        .collect();

    (inputs, rules)
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    inputs: (Node, Node),
    output: Node,
    operation: Operation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    pub fn apply(self, a: bool, b: bool) -> bool {
        match self {
            Operation::And => a && b,
            Operation::Or => a || b,
            Operation::Xor => a ^ b,
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Node(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InvalidNodeName;

impl FromStr for Node {
    type Err = InvalidNodeName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err(InvalidNodeName);
        }
        let mut n = 0;
        for c in s.bytes() {
            let digit = match c {
                c if c.is_ascii_lowercase() => (c - b'a') as u16 + 10,
                c if c.is_ascii_digit() => (c - b'0') as u16,
                _ => return Err(InvalidNodeName),
            };
            n *= 36;
            n += digit;
        }
        Ok(Self(n))
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_ascii_char = |digit| {
            if digit < 10 {
                b'0' + (digit as u8)
            } else {
                b'a' + (digit as u8 - 10)
            }
        };
        let chars = [
            as_ascii_char((self.0 / 1296) % 36),
            as_ascii_char((self.0 / 36) % 36),
            as_ascii_char(self.0 % 36),
        ];
        let s = str::from_utf8(&chars).unwrap();
        Display::fmt(s, f)
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<<{}>>", self)
    }
}

impl Node {
    pub const fn max() -> Self {
        Self(36 * 36 * 36 - 1)
    }

    pub fn x(idx: u16) -> Self {
        Self(1296 * 33 + (idx / 10) * 36 + idx % 10)
    }

    pub fn y(idx: u16) -> Self {
        Self(1296 * 34 + (idx / 10) * 36 + idx % 10)
    }

    pub fn z(idx: u16) -> Self {
        Self(1296 * 35 + (idx / 10) * 36 + idx % 10)
    }

    pub fn idx(self) -> usize {
        self.0 as usize
    }

    pub fn is_x(self) -> bool {
        (Node::x(0).0..=Node::x(45).0).contains(&self.0)
    }

    pub fn is_y(self) -> bool {
        (Node::y(0).0..=Node::y(45).0).contains(&self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1test() {
        let input1 = "\
            x00: 1\n\
            x01: 1\n\
            x02: 1\n\
            y00: 0\n\
            y01: 1\n\
            y02: 0\n\
            \n\
            x00 AND y00 -> z00\n\
            x01 XOR y01 -> z01\n\
            x02 OR y02 -> z02\n\
        ";
        assert_eq!(day24part1(input1), 4);

        let input2 = "\
            x00: 1\n\
            x01: 0\n\
            x02: 1\n\
            x03: 1\n\
            x04: 0\n\
            y00: 1\n\
            y01: 1\n\
            y02: 1\n\
            y03: 1\n\
            y04: 1\n\
            \n\
            ntg XOR fgs -> mjb\n\
            y02 OR x01 -> tnw\n\
            kwq OR kpj -> z05\n\
            x00 OR x03 -> fst\n\
            tgd XOR rvg -> z01\n\
            vdt OR tnw -> bfw\n\
            bfw AND frj -> z10\n\
            ffh OR nrd -> bqk\n\
            y00 AND y03 -> djm\n\
            y03 OR y00 -> psh\n\
            bqk OR frj -> z08\n\
            tnw OR fst -> frj\n\
            gnj AND tgd -> z11\n\
            bfw XOR mjb -> z00\n\
            x03 OR x00 -> vdt\n\
            gnj AND wpb -> z02\n\
            x04 AND y00 -> kjc\n\
            djm OR pbm -> qhw\n\
            nrd AND vdt -> hwm\n\
            kjc AND fst -> rvg\n\
            y04 OR y02 -> fgs\n\
            y01 AND x02 -> pbm\n\
            ntg OR kjc -> kwq\n\
            psh XOR fgs -> tgd\n\
            qhw XOR tgd -> z09\n\
            pbm OR djm -> kpj\n\
            x03 XOR y03 -> ffh\n\
            x00 XOR y04 -> ntg\n\
            bfw OR bqk -> z06\n\
            nrd XOR fgs -> wpb\n\
            frj XOR qhw -> z04\n\
            bqk OR frj -> z07\n\
            y03 OR x01 -> nrd\n\
            hwm AND bqk -> z03\n\
            tgd XOR rvg -> z12\n\
            tnw OR pbm -> gnj\n\
        ";
        assert_eq!(day24part1(input2), 2024);
    }
}

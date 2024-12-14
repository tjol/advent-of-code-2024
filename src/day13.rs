#![allow(clippy::let_and_return)]

use itertools::Itertools;
use regex::Regex;

pub fn day13part1(input: &str) -> i64 {
    let machines = parse_rules(input);

    let wins = machines.iter().filter_map(get_move);

    let price = wins.map(|(a, b)| 3 * a + b).sum();

    price
}

pub fn day13part2(input: &str) -> i64 {
    let mut machines = parse_rules(input);

    for machine in &mut machines {
        machine.prize = (
            machine.prize.0 + 10000000000000,
            machine.prize.1 + 10000000000000,
        );
    }

    let wins = machines.iter().filter_map(get_move);

    let price = wins.map(|(a, b)| 3 * a + b).sum();

    price
}

fn get_move(machine: &ClawMachine) -> Option<(i64, i64)> {
    let num = machine.prize.0 * machine.b.1 - machine.prize.1 * machine.b.0;
    let denom = machine.a.0 * machine.b.1 - machine.a.1 * machine.b.0;

    let n = num / denom;
    if n * denom == num {
        // integer solution for n
        let m = (machine.prize.0 - n * machine.a.0) / machine.b.0;

        // check the solution
        if n * machine.a.0 + m * machine.b.0 == machine.prize.0
            && n * machine.a.1 + m * machine.b.1 == machine.prize.1
        {
            return Some((n, m));
        }
    }
    None
}

#[derive(Debug, Clone)]
struct ClawMachine {
    pub a: (i64, i64),
    pub b: (i64, i64),
    pub prize: (i64, i64),
}

fn parse_rules(input: &str) -> Vec<ClawMachine> {
    let re1 = Regex::new(r#"Button .: X\+(\d+), Y\+(\d+)"#).unwrap();
    let re2 = Regex::new(r#"Prize: X=(\d+), Y=(\d+)"#).unwrap();

    let mut machines = vec![];

    let lines = input.lines().collect_vec();
    let n = (lines.len() + 1) / 4;
    for i in 0..n {
        let m_a = re1.captures(lines[4 * i]).unwrap();
        let m_b = re1.captures(lines[4 * i + 1]).unwrap();
        let m_p = re2.captures(lines[4 * i + 2]).unwrap();

        let a1 = m_a.get(1).unwrap().as_str().parse().unwrap();
        let a2 = m_a.get(2).unwrap().as_str().parse().unwrap();
        let b1 = m_b.get(1).unwrap().as_str().parse().unwrap();
        let b2 = m_b.get(2).unwrap().as_str().parse().unwrap();
        let x = m_p.get(1).unwrap().as_str().parse().unwrap();
        let y = m_p.get(2).unwrap().as_str().parse().unwrap();

        machines.push(ClawMachine {
            a: (a1, a2),
            b: (b1, b2),
            prize: (x, y),
        });
    }

    machines
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "\
        Button A: X+94, Y+34\n\
        Button B: X+22, Y+67\n\
        Prize: X=8400, Y=5400\n\
        \n\
        Button A: X+26, Y+66\n\
        Button B: X+67, Y+21\n\
        Prize: X=12748, Y=12176\n\
        \n\
        Button A: X+17, Y+86\n\
        Button B: X+84, Y+37\n\
        Prize: X=7870, Y=6450\n\
        \n\
        Button A: X+69, Y+23\n\
        Button B: X+27, Y+71\n\
        Prize: X=18641, Y=10279\n\
    ";

    #[test]
    fn part1test() {
        assert_eq!(day13part1(TEST_INPUT), 480);
    }

    #[test]
    fn part2test() {
        let mut machines = parse_rules(TEST_INPUT);

        for machine in &mut machines {
            machine.prize = (
                machine.prize.0 + 10000000000000,
                machine.prize.1 + 10000000000000,
            );
        }

        assert!(get_move(&machines[0]).is_none());
        assert!(get_move(&machines[1]).is_some());
        assert!(get_move(&machines[2]).is_none());
        assert!(get_move(&machines[3]).is_some());
    }
}

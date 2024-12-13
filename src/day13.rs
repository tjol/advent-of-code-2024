use itertools::Itertools;
use regex::Regex;
use nalgebra::{Matrix2, Point2, Vector2};

pub fn day13part1(input: &str) -> i32 {
    let machines = parse_rules(input);

    let wins = machines.iter().filter_map(|m| {
        if let Some(inv_mat) = m.matrix.try_inverse() {
            let the_move = inv_mat * m.prize;
            if (the_move.x.round() - the_move.x).abs() < 1e-6 && (the_move.y.round() - the_move.y).abs() < 1e-6 {
                return Some((the_move.x.round() as i32, the_move.y.round() as i32))
            }
        }
        None
    });

    let price = wins.map(|(a, b)| 3*a + b).sum();

    price
}

pub fn day13part2(input: &str) -> u64 {
    let mut machines = parse_rules(input);

    for machine in &mut machines {
        machine.prize += Vector2::new(10000000000000.0, 10000000000000.0);
    }

    let wins = machines.iter().filter_map(|m| {
        if let Some(inv_mat) = m.matrix.try_inverse() {
            let the_move = inv_mat * m.prize;
            if (the_move.x.round() - the_move.x).abs() < 1e-6 && (the_move.y.round() - the_move.y).abs() < 1e-6 {
                return Some((the_move.x.round() as u64, the_move.y.round() as u64))
            }
        }
        None
    });

    let price = wins.map(|(a, b)| 3*a + b).sum();

    price
}

#[derive(Debug, Clone)]
struct ClawMachine {
    pub matrix: Matrix2<f64>,
    pub prize: Point2<f64>
}

fn parse_rules(input: &str) -> Vec<ClawMachine> {
    let re1 = Regex::new(r#"Button .: X\+(\d+), Y\+(\d+)"#).unwrap();
    let re2 = Regex::new(r#"Prize: X=(\d+), Y=(\d+)"#).unwrap();

    let mut machines = vec![];

    let lines = input.lines().collect_vec();
    let n = (lines.len() + 1) / 4;
    for i in 0..n {
        let m_a = re1.captures(lines[4*i]).unwrap();
        let m_b = re1.captures(lines[4*i+1]).unwrap();
        let m_p = re2.captures(lines[4*i+2]).unwrap();

        let dx_a = m_a.get(1).unwrap().as_str().parse().unwrap();
        let dy_a = m_a.get(2).unwrap().as_str().parse().unwrap();
        let dx_b = m_b.get(1).unwrap().as_str().parse().unwrap();
        let dy_b = m_b.get(2).unwrap().as_str().parse().unwrap();
        let x = m_p.get(1).unwrap().as_str().parse().unwrap();
        let y = m_p.get(2).unwrap().as_str().parse().unwrap();

        machines.push(ClawMachine {
            matrix: Matrix2::new(dx_a, dx_b, dy_a, dy_b),
            prize: Point2::new(x, y)
        });
    }

    machines
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT : &str = "\
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
}


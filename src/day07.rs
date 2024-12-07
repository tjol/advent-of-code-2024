use std::str::FromStr;

pub fn day07part1(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    let eqs = parse_rules(&lines);

    eqs.into_iter().filter_map(|eq| find_operators(eq.result, eq.operands[0], &eq.operands[1..]).is_some().then_some(eq.result)).sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BareEquation {
    pub result: i64,
    pub operands: Vec<i64>
}

fn find_operators(result: i64, first: i64, tail: &[i64]) -> Option<String> {
    if tail.is_empty() {
        if result == first {
            return Some("".to_string());
        } else {
            return None;
        }
    }
    let next = tail[0];
    if first * next <= result {
        if let Some(ans) = find_operators(result, first*next, &tail[1..]) {
            return Some(ans);
        }
    }
    if first + next <= result {
        return find_operators(result, first+next, &tail[1..]);
    }
    None
}

impl FromStr for BareEquation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(':').unwrap();
        let result = left.parse().unwrap();
        let operands = right.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        
        Ok(Self{result, operands})
    }
}

fn parse_rules(lines: &[&str]) -> Vec<BareEquation> {
    lines
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect()
}


#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &'static str = "\
        190: 10 19\n\
        3267: 81 40 27\n\
        83: 17 5\n\
        156: 15 6\n\
        7290: 6 8 6 15\n\
        161011: 16 10 13\n\
        192: 17 8 14\n\
        21037: 9 7 18 13\n\
        292: 11 6 16 20\n\
        ";

    #[test]
    fn part1test() {
        assert_eq!(day07part1(TEST_INPUT), 3749);
    }
}

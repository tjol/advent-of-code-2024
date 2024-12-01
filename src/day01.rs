use std::collections::BTreeMap;

pub fn day01part1(input: &str) -> i32 {
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();
    for line in input.split('\n') {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if !parts.is_empty() {
            left.push(parts[0].parse().unwrap());
            right.push(parts[1].parse().unwrap());
        }
    }

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right.into_iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn day01part2(input: &str) -> i64 {
    let mut left = Vec::<i64>::new();
    let mut right = Vec::<i64>::new();
    for line in input.split('\n') {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if !parts.is_empty() {
            left.push(parts[0].parse().unwrap());
            right.push(parts[1].parse().unwrap());
        }
    }

    let mut counts = BTreeMap::<i64, i64>::new();
    for n in right {
        counts.insert(n, counts.get(&n).copied().unwrap_or_default() + 1);
    }

    left.into_iter()
        .map(|n| n * counts.get(&n).copied().unwrap_or_default())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
            3   4\n\
            4   3\n\
            2   5\n\
            1   3\n\
            3   9\n\
            3   3\n\
            ";
        assert_eq!(day01part1(input), 11);
    }
}

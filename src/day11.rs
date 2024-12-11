use std::collections::BTreeMap;

pub fn day11part1(input: &str) -> usize {
    let mut counters = parse_stones(input);

    for _ in 0..25 {
        counters = blink(&counters);
    }

    total_stones(&counters)
}

pub fn day11part2(input: &str) -> usize {
    let mut counters = parse_stones(input);

    for _ in 0..75 {
        counters = blink(&counters);
    }

    total_stones(&counters)
}

fn parse_stones(input: &str) -> BTreeMap<usize, usize> {
    input
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .zip(std::iter::repeat(1))
        .collect()
}

fn blink(counters: &BTreeMap<usize, usize>) -> BTreeMap<usize, usize> {
    let mut new_stones = BTreeMap::new();

    for (&value, &count) in counters {
        if value == 0 {
            *new_stones.entry(1).or_default() += count;
        } else {
            let n_digits = digits(value);
            if (n_digits & 1) == 0 {
                // even nr of digits
                let m = 10_usize.pow(n_digits / 2);
                let a = value / m;
                let b = value % m;
                *new_stones.entry(a).or_default() += count;
                *new_stones.entry(b).or_default() += count;
            } else {
                // odd nr of digits
                *new_stones.entry(value * 2024).or_default() += count;
            }
        }
    }

    new_stones
}

fn total_stones(counters: &BTreeMap<usize, usize>) -> usize {
    counters.values().sum()
}

fn digits(mut n: usize) -> u32 {
    let mut digits = 1;
    while n >= 10 {
        n /= 10;
        digits += 1;
    }
    digits
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1test_small() {
        let mut counters = parse_stones("125 17");
        for _ in 0..6 {
            counters = blink(&counters);
        }
        assert_eq!(total_stones(&counters), 22);
    }

    #[test]
    fn part1test_full() {
        assert_eq!(day11part1("125 17\n"), 55312);
    }
}

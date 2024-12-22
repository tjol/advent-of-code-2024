use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

pub fn day22part1(input: &str) -> u64 {
    let numbers: Vec<u32> = input.lines().filter_map(|s| s.parse().ok()).collect();

    let mut sum = 0;
    for seed in numbers {
        let mut n = seed;
        for _ in 0..2000 {
            n = monkey(n);
        }
        sum += n as u64;
    }
    sum
}

pub fn day22part2(input: &str) -> u64 {
    let seeds: Vec<u32> = input.lines().filter_map(|s| s.parse().ok()).collect();

    let mut all_prices = vec![];

    for seed in seeds {
        let mut prices = vec![];
        let mut n = seed;
        for _ in 0..2000 {
            n = monkey(n);
            let price = (n % 10) as i8;
            prices.push(price);
        }
        all_prices.push(prices);
    }

    let diffs = all_prices
        .iter()
        .map(|prices| {
            prices
                .iter()
                .zip(&prices[1..])
                .map(|(&p1, &p2)| p2 - p1)
                .collect_vec()
        })
        .collect_vec();

    let all_instructions = diffs
        .iter()
        .zip(&all_prices)
        .map(|(diffs, prices)| {
            let mut instructions = HashMap::new();
            for i in 3..diffs.len() {
                let price = prices[i + 1];
                let seq = (diffs[i - 3], diffs[i - 2], diffs[i - 1], diffs[i]);
                instructions.entry(seq).or_insert(price);
            }
            instructions
        })
        .collect_vec();

    let possible_instruction: HashSet<_> = all_instructions
        .iter()
        .flat_map(|ii| ii.keys().copied())
        .collect();

    let mut max_bananas = 0;

    for instr in possible_instruction {
        let bananas = all_instructions
            .iter()
            .map(|ii| ii.get(&instr).copied().unwrap_or_default() as u64)
            .sum::<u64>();
        max_bananas = max_bananas.max(bananas);
    }

    max_bananas
}

fn monkey(mut n: u32) -> u32 {
    n = ((n << 6) ^ n) & 0xffffff;
    n = ((n >> 5) ^ n) & 0xfffffff;
    n = ((n << 11) ^ n) & 0xffffff;
    n
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "\
        1\n\
        10\n\
        100\n\
        2024\n\
    ";

    #[test]
    fn part1test() {
        assert_eq!(day22part1(TEST_INPUT), 37327623);
    }
}

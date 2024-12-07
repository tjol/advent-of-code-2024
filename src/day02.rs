use itertools::Itertools;

pub fn day02part1(input: &str) -> i32 {
    input
        .split('\n')
        .filter(|line| {
            let levels = line
                .trim()
                .split_ascii_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<i32>>();
            valid1(&levels, 1) || valid1(&levels, -1)
        })
        .count() as i32
}

pub fn day02part2(input: &str) -> i32 {
    let level_lines = input
        .split('\n')
        .map(|line| {
            line.trim()
                .split_ascii_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<i32>>()
        })
        .filter(|v| !v.is_empty());

    level_lines
        .filter(|levels| valid2(levels, 1) || valid2(levels, -1))
        .count() as i32
}

fn valid1(levels: &[i32], sign: i32) -> bool {
    match levels
        .iter()
        .tuple_windows()
        .map(|(a, b)| (b - a) * sign)
        .minmax()
    {
        itertools::MinMaxResult::MinMax(mindiff, maxdiff) => mindiff >= 1 && maxdiff <= 3,
        itertools::MinMaxResult::OneElement(diff) => (1..=3).contains(&diff),
        itertools::MinMaxResult::NoElements => false,
    }
}

fn valid2(levels: &[i32], sign: i32) -> bool {
    if levels.is_empty() {
        return false;
    }
    let mut had_invalid = false;
    let mut prev = levels[0];

    for i in 1..levels.len() {
        let diff = (levels[i] - prev) * sign;
        if !(1..=3).contains(&diff) {
            if had_invalid {
                return false;
            } else if i >= 2 {
                // check if removing the previous level might help
                let diff2 = (levels[i] - levels[i - 2]) * sign;
                if (1..=3).contains(&diff2) && valid1(&levels[i..], sign) {
                    return true;
                }
            } else if i == 1 && valid1(&levels[i..], sign) {
                return true;
            }
            had_invalid = true;
        } else {
            // valid - accept
            prev = levels[i];
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1test() {
        let input = "\
            7 6 4 2 1\n\
            1 2 7 8 9\n\
            9 7 6 2 1\n\
            1 3 2 4 5\n\
            8 6 4 4 1\n\
            1 3 6 7 9\n\
        ";
        assert_eq!(day02part1(input), 2);
    }

    #[test]
    fn part2test() {
        let input = "\
            7 6 4 2 1\n\
            1 2 7 8 9\n\
            9 7 6 2 1\n\
            1 3 2 4 5\n\
            8 6 4 4 1\n\
            1 3 6 7 9\n\
        ";
        assert_eq!(day02part2(input), 4);
    }

    #[test]
    fn part2_edge_cases() {
        assert!(valid2(&[1, 2, 3, 4, 5], 1));
        assert!(valid2(&[1, 2, 3, 100, 4, 5], 1));
        assert!(!valid2(&[1, 2, 3, 100, 100, 4, 5], 1));
        assert!(valid2(&[1, 2, 3, 4, 5, 100], 1));
        assert!(valid2(&[100, 1, 2, 3, 4, 5], 1));
        assert!(!valid2(&[100, 1, 2, 3, 4, 5, 100], 1));
        assert!(valid2(&[1, 3, 2, 3], 1));
        assert!(!valid2(&[1, 3, 2, 0], 1));
        assert!(!valid2(&[1, 3, 2, 100, 5], 1));
    }
}

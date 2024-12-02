use itertools::Itertools;

pub fn day02part1(input: &str) -> i32 {
    input
        .split('\n')
        .filter(|line| {
            if let itertools::MinMaxResult::MinMax(mindiff, maxdiff) = line
                .trim()
                .split_ascii_whitespace()
                .filter_map(|s| s.parse().ok())
                .tuple_windows()
                .map(|(a, b): (i32, i32)| a - b)
                .minmax()
            {
                (mindiff >= 1 && maxdiff <= 3) || (mindiff >= -3 && maxdiff <= -1)
            } else {
                false
            }
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
        .filter(|levels| min_invalid(levels) <= 1)
        .count() as i32
}

fn count_invalid(levels: &[i32], sign: i32) -> i32 {
    if levels.is_empty() {
        return 0;
    }
    let mut invalid_count = 0;
    let mut prev = levels[0];
    for level in &levels[1..] {
        let diff = (*level - prev) * sign;
        if diff >= 1 && diff <= 3 {
            prev = *level;
        } else {
            invalid_count += 1;
        }
    }
    invalid_count
}

fn min_invalid(levels: &[i32]) -> i32 {
    [
        count_invalid(levels, 1),
        count_invalid(levels, -1),
        count_invalid(&levels[1..], 1) + 1,
        count_invalid(&levels[1..], -1) + 1,
    ]
    .into_iter()
    .min()
    .unwrap()
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
        assert_eq!(min_invalid(&[1, 2, 3, 4, 5]), 0);
        assert_eq!(min_invalid(&[1, 2, 3, 100, 4, 5]), 1);
        assert_eq!(min_invalid(&[1, 2, 3, 100, 100, 4, 5]), 2);
        assert_eq!(min_invalid(&[1, 2, 3, 4, 5, 100]), 1);
        assert_eq!(min_invalid(&[100, 1, 2, 3, 4, 5, 100]), 2);
        assert_eq!(min_invalid(&[100, 1, 2, 3, 100, 4, 5, 100]), 3);
        assert_eq!(min_invalid(&[1, 3, 2, 3]), 1);
    }
}

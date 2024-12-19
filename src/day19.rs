use hashbrown::HashMap;
use itertools::Itertools;

pub fn day19(input: &str) -> (usize, usize) {
    let (towels_owned, designs) = parse_input(input);

    let mut towels = towels_owned.iter().map(String::as_str).collect_vec();
    towels.sort();

    let mut cache = Default::default();

    let mut valid = 0;
    let mut total_solutions = 0;

    for design in &designs {
        let count = make_design(design, &towels, &mut cache);
        if count != 0 {
            total_solutions += count;
            valid += 1;
        }
    }

    (valid, total_solutions)
}

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = input.trim().lines();
    let towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(str::to_string)
        .collect();
    lines.next();
    let designs = lines.map(str::to_string).collect();
    (towels, designs)
}

fn make_design<'d>(design: &'d str, towels: &[&str], cache: &mut HashMap<&'d str, usize>) -> usize {
    if let Some(&answer) = cache.get(&design) {
        return answer;
    } else if design.is_empty() {
        cache.insert(design, 1);
        return 1;
    }

    // find towels that *might* be usable for the start
    let end = towels.partition_point(|&t| t <= design);
    let begin = towels[..end].partition_point(|&t| t < &design[..1]);

    let mut solutions_found = 0;

    for towel in &towels[begin..end] {
        if let Some(rest) = design.strip_prefix(towel) {
            let tail_solutions = make_design(rest, towels, cache);
            solutions_found += tail_solutions;
        }
    }

    cache.insert(design, solutions_found);
    solutions_found
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "\
        r, wr, b, g, bwu, rb, gb, br\n\
        \n\
        brwrr\n\
        bggr\n\
        gbbr\n\
        rrbgbr\n\
        ubwu\n\
        bwurrg\n\
        brgr\n\
        bbrgwb\n\
    ";

    #[test]
    fn part1test() {
        assert_eq!(day19(TEST_INPUT).0, 6);
    }

    #[test]
    fn part2test() {
        assert_eq!(day19(TEST_INPUT).1, 16);
    }
}

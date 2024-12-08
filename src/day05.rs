use std::{collections::BTreeMap, str::FromStr};

pub fn day05part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().map(str::trim).collect();
    let blank_idx = lines
        .iter()
        .enumerate()
        .filter_map(|(idx, line)| line.is_empty().then_some(idx))
        .next()
        .unwrap();
    let (rule_lines, remaining_lines) = lines.split_at(blank_idx);
    let trial_lines = &remaining_lines[1..];

    let rules = parse_rules(rule_lines);
    let page_lists: Vec<_> = trial_lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| parse_pagelist(l))
        .collect();
    let valid: Vec<_> = page_lists
        .iter()
        .filter(|page_list| validate(page_list, &rules))
        .collect();

    let mid_sum = valid
        .iter()
        .map(|page_list| page_list[page_list.len() / 2])
        .sum();

    mid_sum
}

pub fn day05part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().map(str::trim).collect();
    let blank_idx = lines
        .iter()
        .enumerate()
        .filter_map(|(idx, line)| line.is_empty().then_some(idx))
        .next()
        .unwrap();
    let (rule_lines, remaining_lines) = lines.split_at(blank_idx);
    let trial_lines = &remaining_lines[1..];

    let rules = parse_rules(rule_lines);
    let page_lists: Vec<_> = trial_lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| parse_pagelist(l))
        .collect();

    let mut invalid: Vec<_> = page_lists
        .iter()
        .filter(|page_list| !validate(page_list, &rules))
        .cloned()
        .collect();

    for list in &mut invalid {
        reorder_list(list, &rules);
    }

    let mid_sum = invalid
        .iter()
        .map(|page_list| page_list[page_list.len() / 2])
        .sum();

    mid_sum
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rule(i32, i32);

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1s, p2s) = s.split_once('|').unwrap();
        let p1 = p1s.parse().unwrap();
        let p2 = p2s.parse().unwrap();
        Ok(Self(p1, p2))
    }
}

impl Rule {
    pub fn check_page_map(&self, page_map: &BTreeMap<i32, usize>) -> bool {
        let Self(p1, p2) = self;
        if let (Some(&i1), Some(&i2)) = (page_map.get(p1), page_map.get(p2)) {
            i1 < i2
        } else {
            true
        }
    }

    pub fn enforce(&self, page_list: &mut [i32]) -> bool {
        let Self(p1, p2) = *self;
        let mut p1_idx = None;
        let mut p2_idx = None;
        for (idx, &p) in page_list.iter().enumerate() {
            if p == p1 {
                if p2_idx.is_none() {
                    return false;
                } else {
                    p1_idx = Some(idx);
                    break;
                }
            } else if p == p2 {
                // before p1
                p2_idx = Some(idx);
            }
        }

        if let (Some(i), Some(j)) = (p2_idx, p1_idx) {
            page_list.copy_within((i + 1)..=j, i);
            page_list[j] = p2;
            return true;
        }
        false
    }
}

fn parse_rules(lines: &[&str]) -> Vec<Rule> {
    lines.iter().map(|line| line.parse().unwrap()).collect()
}

fn parse_pagelist(s: &str) -> Vec<i32> {
    s.split(',').map(|w| w.parse().unwrap()).collect()
}

fn make_page_map(page_list: &[i32]) -> BTreeMap<i32, usize> {
    page_list
        .iter()
        .enumerate()
        .map(|(idx, &page)| (page, idx))
        .collect()
}

fn validate(page_list: &[i32], rules: &[Rule]) -> bool {
    let page_map = make_page_map(page_list);
    rules.iter().all(|rule| rule.check_page_map(&page_map))
}

fn reorder_list(page_list: &mut [i32], rules: &[Rule]) {
    loop {
        let mut any_enforced = false;
        for rule in rules {
            any_enforced |= rule.enforce(page_list);
        }
        if !any_enforced {
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &'static str = "\
        47|53\n\
        97|13\n\
        97|61\n\
        97|47\n\
        75|29\n\
        61|13\n\
        75|53\n\
        29|13\n\
        97|29\n\
        53|29\n\
        61|53\n\
        97|53\n\
        61|29\n\
        47|13\n\
        75|47\n\
        97|75\n\
        47|61\n\
        75|61\n\
        47|29\n\
        75|13\n\
        53|13\n\
        \n\
        75,47,61,53,29\n\
        97,61,53,29,13\n\
        75,29,13\n\
        75,97,47,61,53\n\
        61,13,29\n\
        97,13,75,29,47\n\
        ";

    #[test]
    fn part1test() {
        assert_eq!(day05part1(TEST_INPUT), 143);
    }

    #[test]
    fn part2test() {
        assert_eq!(day05part2(TEST_INPUT), 123);
    }
}

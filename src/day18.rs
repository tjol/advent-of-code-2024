use std::collections::{BinaryHeap, HashSet};

use itertools::Itertools;

pub fn day18part1(input: &str) -> usize {
    let blocks = parse(input);
    shortest_path_len_from_input(71, 71, &blocks[0..1024]).unwrap_or_default()
}

pub fn day18part2(input: &str) -> String {
    let blocks = parse(input);

    if let Some((x, y)) = first_block_in_path(71, 71, &blocks) {
        format!("{},{}", x, y)
    } else {
        "?".to_string()
    }
}

fn shortest_path_len_from_input(
    width: usize,
    height: usize,
    blocks: &[(usize, usize)],
) -> Option<usize> {
    let mut map = (0..height).map(|_| vec![false; width]).collect_vec();

    for &(x, y) in blocks {
        map[y][x] = true;
    }

    shortest_path_len(&map)
}

fn first_block_in_path(
    width: usize,
    height: usize,
    blocks: &[(usize, usize)],
) -> Option<(usize, usize)> {
    let mut map = (0..height).map(|_| vec![false; width]).collect_vec();

    for &(x, y) in blocks {
        map[y][x] = true;
        if shortest_path_len(&map).is_none() {
            return Some((x, y));
        }
    }
    None
}

fn shortest_path_len(map: &Vec<Vec<bool>>) -> Option<usize> {
    let height = map.len();
    let width = map[0].len();

    let dest = (width - 1, height - 1);
    let start = (0, 0);
    let total_dist = -((width + height - 2) as isize);

    let mut queue = BinaryHeap::new();
    queue.push((total_dist, 0, start));

    let mut seen = HashSet::new();

    while !queue.is_empty() {
        let (_dist, steps, (x, y)) = queue.pop().unwrap();
        if (x, y) == dest {
            return Some((-steps) as usize);
        }
        if seen.contains(&(x, y)) {
            continue;
        }
        seen.insert((x, y));

        if x + 1 < width && !map[y][x + 1] && !seen.contains(&(x + 1, y)) {
            let dist_to_end = dest.0.abs_diff(x + 1) + dest.1.abs_diff(y);
            let min_steps = -(dist_to_end as isize) + steps - 1;
            queue.push((min_steps, steps - 1, (x + 1, y)));
        }
        if x > 0 && !map[y][x - 1] && !seen.contains(&(x - 1, y)) {
            let dist_to_end = dest.0.abs_diff(x - 1) + dest.1.abs_diff(y);
            let min_steps = -(dist_to_end as isize) + steps - 1;
            queue.push((min_steps, steps - 1, (x - 1, y)));
        }
        if y + 1 < height && !map[y + 1][x] && !seen.contains(&(x, y + 1)) {
            let dist_to_end = dest.0.abs_diff(x) + dest.1.abs_diff(y + 1);
            let min_steps = -(dist_to_end as isize) + steps - 1;
            queue.push((min_steps, steps - 1, (x, y + 1)));
        }
        if y > 0 && !map[y - 1][x] && !seen.contains(&(x, y - 1)) {
            let dist_to_end = dest.0.abs_diff(x) + dest.1.abs_diff(y - 1);
            let min_steps = -(dist_to_end as isize) + steps - 1;
            queue.push((min_steps, steps - 1, (x, y - 1)));
        }
    }

    None
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .filter_map(|l| {
            if let Some((a, b)) = l.split_once(',') {
                Some((a.parse().ok()?, b.parse().ok()?))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "\
        5,4\n\
        4,2\n\
        4,5\n\
        3,0\n\
        2,1\n\
        6,3\n\
        2,4\n\
        1,5\n\
        0,6\n\
        3,3\n\
        2,6\n\
        5,1\n\
        1,2\n\
        5,5\n\
        2,5\n\
        6,5\n\
        1,4\n\
        0,4\n\
        6,4\n\
        1,1\n\
        6,1\n\
        1,0\n\
        0,5\n\
        1,6\n\
        2,0\n\
    ";

    #[test]
    fn part1test() {
        let blocks = parse(TEST_INPUT);
        assert_eq!(shortest_path_len_from_input(7, 7, &blocks[0..12]), Some(22));
    }

    #[test]
    fn part2test() {
        let blocks = parse(TEST_INPUT);
        assert_eq!(first_block_in_path(7, 7, &blocks), Some((6, 1)));
    }
}

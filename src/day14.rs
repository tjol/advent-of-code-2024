use std::{fs::OpenOptions, io::Write};

use itertools::Itertools;
use regex::Regex;

const REGION_SIZE: (i64, i64) = (101, 103);

pub fn day14part1(input: &str) -> i64 {
    let robots = parse_robots(input);
    let robots = simulate_robots(&robots, REGION_SIZE, 100);
    safety_score(&robots, REGION_SIZE)
}

pub fn day14part2(input: &str) -> i64 {
    let mut robots = parse_robots(input);

    let mut out_f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("robots.bin")
        .unwrap();

    const ANSWER: i64 = 6587; // found by looking at the video in imagej

    for i in 0..(103 * 101) {
        let img = robot_density_img(&robots, REGION_SIZE);
        let img_bytes =
            unsafe { std::slice::from_raw_parts(img.as_ptr() as *const u8, img.len() * 2) };
        out_f.write_all(img_bytes).unwrap();

        if i == ANSWER {
            print_robots(&robots, REGION_SIZE);
        }

        step_robots_once(&mut robots, REGION_SIZE);
    }

    ANSWER
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pub pos: (i64, i64),
    pub v: (i64, i64),
}

fn parse_robots(input: &str) -> Vec<Robot> {
    let re = Regex::new(r#"p=(\d+),(\d+) v=(-?\d+),(-?\d+)"#).unwrap();
    re.captures_iter(input)
        .map(|c| {
            let x = c.get(1).unwrap().as_str().parse().unwrap();
            let y = c.get(2).unwrap().as_str().parse().unwrap();
            let vx = c.get(3).unwrap().as_str().parse().unwrap();
            let vy = c.get(4).unwrap().as_str().parse().unwrap();
            Robot {
                pos: (x, y),
                v: (vx, vy),
            }
        })
        .collect()
}

fn simulate_robots(robots: &[Robot], region_size: (i64, i64), iterations: i64) -> Vec<Robot> {
    let (w, h) = region_size;

    robots
        .iter()
        .map(|r| {
            let x = (r.pos.0 + iterations * r.v.0).rem_euclid(w);
            let y = (r.pos.1 + iterations * r.v.1).rem_euclid(h);

            Robot {
                pos: (x, y),
                v: r.v,
            }
        })
        .collect()
}

fn step_robots_once(robots: &mut [Robot], region_size: (i64, i64)) {
    let (w, h) = region_size;

    for r in robots {
        let x = (r.pos.0 + r.v.0).rem_euclid(w);
        let y = (r.pos.1 + r.v.1).rem_euclid(h);
        r.pos = (x, y);
    }
}

fn print_robots(robots: &[Robot], region_size: (i64, i64)) {
    let (w, h) = region_size;
    let mut map = (0..h).map(|_| vec![0; w as usize]).collect_vec();
    for r in robots {
        map[r.pos.1 as usize][r.pos.0 as usize] += 1;
    }

    let map_s = map
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|count| if count > 0 { '*' } else { '.' })
                .collect::<String>()
        })
        .join("\n");
    println!("{}", map_s);
}

fn robot_density_img(robots: &[Robot], region_size: (i64, i64)) -> Vec<u16> {
    let (w, h) = region_size;
    let mut img = vec![0; (w * h) as usize];
    for r in robots {
        let idx = r.pos.0 as usize + (r.pos.1 * w) as usize;
        img[idx] += 1;
    }
    img
}

fn safety_score(robots: &[Robot], region_size: (i64, i64)) -> i64 {
    let (w, h) = region_size;
    let mid_x = w / 2;
    let mid_y = h / 2;

    let mut tl = 0;
    let mut tr = 0;
    let mut br = 0;
    let mut bl = 0;

    for r in robots {
        if r.pos.0 < mid_x && r.pos.1 < mid_y {
            tl += 1;
        }
        if r.pos.0 > mid_x && r.pos.1 < mid_y {
            tr += 1;
        }
        if r.pos.0 > mid_x && r.pos.1 > mid_y {
            br += 1;
        }
        if r.pos.0 < mid_x && r.pos.1 > mid_y {
            bl += 1;
        }
    }

    tl * tr * br * bl
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "\
        p=0,4 v=3,-3\n\
        p=6,3 v=-1,-3\n\
        p=10,3 v=-1,2\n\
        p=2,0 v=2,-1\n\
        p=0,0 v=1,3\n\
        p=3,0 v=-2,-2\n\
        p=7,6 v=-1,-3\n\
        p=3,0 v=-1,-2\n\
        p=9,3 v=2,3\n\
        p=7,3 v=-1,2\n\
        p=2,4 v=2,-3\n\
        p=9,5 v=-3,-3\n\
    ";

    const TEST_SIZE: (i64, i64) = (11, 7);

    #[test]
    fn part1test() {
        let robots = parse_robots(TEST_INPUT);
        let robots = simulate_robots(&robots, TEST_SIZE, 100);
        assert_eq!(safety_score(&robots, TEST_SIZE), 12)
    }

    #[test]
    fn test_wrapping() {
        // let's just check that we're going around the edges correctly
        assert_eq!(
            simulate_robots(
                &[Robot {
                    pos: (1, 1),
                    v: (2, 0)
                }],
                (3, 3),
                1
            )[0]
            .pos,
            (0, 1)
        );

        // this fails when we use the wrong modulo operator... (thanks, C)
        assert_eq!(
            simulate_robots(
                &[Robot {
                    pos: (1, 1),
                    v: (-2, 0)
                }],
                (3, 3),
                1
            )[0]
            .pos,
            (2, 1)
        );
    }
}

use itertools::Itertools;
use regex::Regex;

const REGION_SIZE: (i64, i64) = (101, 103);

pub fn day14part1(input: &str) -> i64 {
    let robots = parse_robots(input);
    let robots = simulate_robots(&robots, REGION_SIZE, 100);
    safety_score(&robots, REGION_SIZE)
}

pub fn day14part2(input: &str) -> i64 {
    let robots = parse_robots(input);

    let (w, h) = REGION_SIZE;
    let dim = w.max(h);
    let states = (0..dim)
        .map(|i| simulate_robots(&robots, REGION_SIZE, i))
        .collect_vec();

    // the x coordinate repeats every w frames
    let x_vars = states[0..w as usize]
        .iter()
        .map(|robots| {
            let n = robots.len() as f64;
            let mean = robots.iter().map(|r| r.pos.0 as f64).sum::<f64>() / n;
            let var = robots
                .iter()
                .map(|r| (r.pos.0 as f64 - mean).powi(2))
                .sum::<f64>()
                / n;
            var
        })
        .collect_vec();
    // the y coordinate repeats every h frames
    let y_vars = states[0..h as usize]
        .iter()
        .map(|robots| {
            let n = robots.len() as f64;
            let mean = robots.iter().map(|r| r.pos.1 as f64).sum::<f64>() / n;
            let var = robots
                .iter()
                .map(|r| (r.pos.1 as f64 - mean).powi(2))
                .sum::<f64>()
                / n;
            var
        })
        .collect_vec();

    // find the minimum variance -> that's when the coordinates match our tree
    let a = x_vars
        .iter()
        .enumerate()
        .min_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap())
        .unwrap()
        .0 as i64;
    let b = y_vars
        .iter()
        .enumerate()
        .min_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap())
        .unwrap()
        .0 as i64;

    // Find integers (n, m) s.t. a + n*w = b + m*h
    // equiv: find an integer n such that a - b + n * w is divisible by h
    for n in 0..h {
        if (a - b + n * w).rem_euclid(h) == 0 {
            let t = a + n * w;
            // print_robots(&simulate_robots(&robots, REGION_SIZE, t), REGION_SIZE);
            return t;
        }
    }

    0
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

#[allow(unused)]
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

#[allow(unused)]
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

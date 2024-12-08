use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

pub fn day08part1(input: &str) -> usize {
    let map = parse_tower_locations(input);

    let mut antinodes = HashSet::new();

    for towers in map.locations_by_freq.values() {
        antinodes.extend(find_antinodes(map.size, towers));
    }

    antinodes.len()
}

pub fn day08part2(input: &str) -> usize {
    let map = parse_tower_locations(input);

    let mut antinodes = HashSet::new();

    for towers in map.locations_by_freq.values() {
        antinodes.extend(find_antinodes2(map.size, towers));
    }

    antinodes.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Size(i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2(i32, i32);

impl Add<Vec2> for Vec2 {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add<Vec2> for Point {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Vec2> for Point {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub<Point> for Point {
    type Output = Vec2;
    fn sub(self, rhs: Point) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Point {
    pub fn in_bounds(self, size: Size) -> bool {
        (0..size.0).contains(&self.0) && (0..size.1).contains(&self.1)
    }
}

struct TowerMap {
    pub size: Size,
    pub locations_by_freq: HashMap<char, Vec<Point>>,
}

fn parse_tower_locations(input: &str) -> TowerMap {
    let mut locations = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        width = width.max(line.len() as i32);
        height = height.max(y as i32 + 1);
        for (x, c) in line.char_indices() {
            if c != '.' && !c.is_whitespace() {
                locations
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push(Point(x as i32, y as i32));
            }
        }
    }
    TowerMap {
        size: Size(width, height),
        locations_by_freq: locations,
    }
}

fn find_antinodes(size: Size, towers: &[Point]) -> HashSet<Point> {
    let mut antinodes = HashSet::new();

    for i in 0..towers.len() - 1 {
        for j in i + 1..towers.len() {
            let a = towers[i];
            let b = towers[j];
            let v = b - a;
            let node1 = a - v;
            let node2 = b + v;
            if node1.in_bounds(size) {
                antinodes.insert(node1);
            }
            if node2.in_bounds(size) {
                antinodes.insert(node2);
            }
        }
    }

    antinodes
}

fn find_antinodes2(size: Size, towers: &[Point]) -> HashSet<Point> {
    let mut antinodes = HashSet::new();

    for i in 0..towers.len() - 1 {
        for j in i + 1..towers.len() {
            let a = towers[i];
            let b = towers[j];
            let v = b - a;

            let mut node = a;
            while node.in_bounds(size) {
                antinodes.insert(node);
                node = node - v;
            }

            node = b;
            while node.in_bounds(size) {
                antinodes.insert(node);
                node = node + v;
            }
        }
    }

    antinodes
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &'static str = "\
        ............\n\
        ........0...\n\
        .....0......\n\
        .......0....\n\
        ....0.......\n\
        ......A.....\n\
        ............\n\
        ............\n\
        ........A...\n\
        .........A..\n\
        ............\n\
        ............\n\
        ";

    #[test]
    fn part1test() {
        assert_eq!(day08part1(TEST_INPUT), 14);
    }

    #[test]
    fn part2test() {
        assert_eq!(day08part2(TEST_INPUT), 34);
    }
}

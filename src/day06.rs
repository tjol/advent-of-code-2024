use std::{collections::HashSet, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapTile {
    Visited,
    NotVisited,
    Obstacle,
}

struct Map {
    width: usize,
    matrix: Vec<MapTile>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(pub i32, pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Add<Direction> for Pos {
    type Output = Self;
    fn add(self, dir: Direction) -> Self::Output {
        let Self(x, y) = self;
        match dir {
            Direction::North => Self(x, y - 1),
            Direction::East => Self(x + 1, y),
            Direction::South => Self(x, y + 1),
            Direction::West => Self(x - 1, y),
        }
    }
}

impl Direction {
    pub fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

impl Map {
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.matrix.len() / self.width
    }

    pub fn contains(&self, pos: Pos) -> bool {
        let Pos(x, y) = pos;
        x >= 0 && x < self.width() as i32 && y >= 0 && y < self.height() as i32
    }

    pub fn get(&self, pos: Pos) -> Option<MapTile> {
        if self.contains(pos) {
            let Pos(x, y) = pos;
            let idx = self.width() * y as usize + x as usize;
            Some(self.matrix[idx])
        } else {
            None
        }
    }

    pub fn mark(&mut self, pos: Pos) {
        let Pos(x, y) = pos;
        let idx = self.width() * y as usize + x as usize;
        self.matrix[idx] = MapTile::Visited;
    }

    pub fn add_obstable(&mut self, pos: Pos) {
        let Pos(x, y) = pos;
        let idx = self.width() * y as usize + x as usize;
        self.matrix[idx] = MapTile::Obstacle;
    }

    pub fn remove_obstable(&mut self, pos: Pos) {
        let Pos(x, y) = pos;
        let idx = self.width() * y as usize + x as usize;
        self.matrix[idx] = MapTile::NotVisited;
    }

    pub fn count_visited(&self) -> usize {
        self.matrix
            .iter()
            .filter(|&&tile| tile == MapTile::Visited)
            .count()
    }
}

fn parse_map(s: &str) -> (Pos, Map) {
    let mut guard_pos = None;
    let lines: Vec<_> = s.lines().collect();
    let width = lines.iter().map(|s| s.len()).max().unwrap();
    let height = lines.len();
    let mut matrix = vec![MapTile::NotVisited; height * width];
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let idx = x + y * width;
            match c {
                '#' => matrix[idx] = MapTile::Obstacle,
                '^' => guard_pos = Some(Pos(x as i32, y as i32)),
                _ => (),
            }
        }
    }
    (guard_pos.unwrap(), Map { width, matrix })
}

pub fn day06part1(input: &str) -> usize {
    let (mut guard_pos, mut map) = parse_map(input);

    let mut dir = Direction::North;
    loop {
        map.mark(guard_pos);
        let mut new_pos = guard_pos + dir;
        while map.get(new_pos) == Some(MapTile::Obstacle) {
            dir = dir.turn_right();
            new_pos = guard_pos + dir;
        }
        if map.contains(new_pos) {
            guard_pos = new_pos;
        } else {
            break;
        }
    }

    map.count_visited()
}

pub fn day06part2(input: &str) -> usize {
    let (orig_pos, mut map) = parse_map(input);
    let mut candidate_locations = HashSet::new();

    let mut dir = Direction::North;
    let mut guard_pos = orig_pos;
    loop {
        let mut new_pos = guard_pos + dir;
        while map.get(new_pos) == Some(MapTile::Obstacle) {
            dir = dir.turn_right();
            new_pos = guard_pos + dir;
        }
        if map.contains(new_pos) {
            // What if there were an obstacle here?
            if new_pos != orig_pos && !candidate_locations.contains(&new_pos) {
                map.add_obstable(new_pos);
                if has_loop(&map, orig_pos, Direction::North) {
                    candidate_locations.insert(new_pos);
                }
                map.remove_obstable(new_pos);
            }
            guard_pos = new_pos;
        } else {
            break;
        }
    }

    candidate_locations.len()
}

fn has_loop(map: &Map, mut pos: Pos, mut dir: Direction) -> bool {
    let mut visited = HashSet::<(Pos, Direction)>::new();

    loop {
        if !visited.insert((pos, dir)) {
            // previously visited!
            return true;
        }
        let mut new_pos = pos + dir;
        while map.get(new_pos) == Some(MapTile::Obstacle) {
            dir = dir.turn_right();
            new_pos = pos + dir;
        }
        if map.contains(new_pos) {
            pos = new_pos;
        } else {
            // we left the map -> no loop
            return false;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &'static str = "\
        ....#.....\n\
        .........#\n\
        ..........\n\
        ..#.......\n\
        .......#..\n\
        ..........\n\
        .#..^.....\n\
        ........#.\n\
        #.........\n\
        ......#...\n\
        ";

    #[test]
    fn part1test() {
        assert_eq!(day06part1(TEST_INPUT), 41);
    }

    #[test]
    fn part2test() {
        assert_eq!(day06part2(TEST_INPUT), 6);
    }
}

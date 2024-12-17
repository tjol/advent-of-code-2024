use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
    ops::Add,
    str::FromStr,
};

use itertools::Itertools;

pub fn day16(input: &str) -> (i32, usize) {
    let map: Map<Tile> = input.parse().unwrap();

    let (score, paths) = get_best_paths(&map);

    let mut best_spots = HashSet::new();
    for path in paths {
        best_spots.extend(path);
    }

    (score, best_spots.len())
}

#[allow(unused)]
fn draw_path(path: &[Pos]) {
    let max_x = path.iter().map(|p| p.0).max().unwrap();
    let max_y = path.iter().map(|p| p.0).max().unwrap();

    let mut bitmap = Vec::new();
    for _y in 0..=max_y {
        bitmap.push(vec![false; (max_x + 1) as usize]);
    }

    for &Pos(x, y) in path {
        bitmap[y as usize][x as usize] = true;
    }

    println!(
        "{}",
        bitmap
            .iter()
            .map(|row| row
                .iter()
                .map(|visited| visited.then_some('#').unwrap_or(' '))
                .collect::<String>())
            .join("\n")
    );
}

fn get_best_paths(map: &Map<Tile>) -> (i32, Vec<Vec<Pos>>) {
    let start_pos = map.find_all(|&tile| tile == Tile::Start).next().unwrap();
    let end_pos = map.find_all(|&tile| tile == Tile::End).next().unwrap();

    let distance = |pos: Pos| {
        let x_dist = (pos.0 - end_pos.0).abs();
        let y_dist = (pos.0 - end_pos.0).abs();
        let turn_penalty = if x_dist != 0 && y_dist != 0 { 1000 } else { 0 };
        x_dist + y_dist + turn_penalty
    };

    let mut queue = BinaryHeap::new();
    queue.push((-distance(start_pos), 0, vec![start_pos], Direction::East));

    let mut min_score = HashMap::new();

    let mut paths = vec![];
    let mut score = None;

    while let Some((_dist, balance, mut path, dir)) = queue.pop() {
        let pos = *path.last().unwrap();
        let min_score_here = min_score.entry((pos, dir)).or_insert(-balance);
        match (*min_score_here).cmp(&(-balance)) {
            Ordering::Less => continue,
            _ => *min_score_here = -balance,
        }

        if score.is_some_and(|s| -balance > s) {
            break;
        }

        if map.get(pos).is_some_and(|&t| t == Tile::End) {
            // found a path to the end
            paths.push(path);
            score = Some(-balance);
            continue;
        }
        // survey the options!
        let a_droite = pos + dir.turn_right();
        if map.get(a_droite).is_some_and(|&t| t != Tile::Wall) {
            // going right might be nice
            let mut new_path = path.clone();
            new_path.push(a_droite);
            queue.push((
                -distance(a_droite) + balance - 1001,
                balance - 1001,
                new_path,
                dir.turn_right(),
            ));
        }
        let a_gauche = pos + dir.turn_left();
        if map.get(a_gauche).is_some_and(|&t| t != Tile::Wall) {
            // im Zweifel links
            let mut new_path = path.clone();
            new_path.push(a_gauche);
            queue.push((
                -distance(a_gauche) + balance - 1001,
                balance - 1001,
                new_path,
                dir.turn_left(),
            ));
        }
        let straight = pos + dir;
        if map.get(straight).is_some_and(|&t| t != Tile::Wall) {
            // can go straight
            path.push(straight);
            queue.push((-distance(straight) + balance - 1, balance - 1, path, dir));
        }
    }

    (score.unwrap_or(-1), paths)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos(pub i32, pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    pub fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Debug, Clone)]
struct Map<Item>
where
    Item: Debug + Clone + Sized,
{
    width: i32,
    matrix: Vec<Item>,
}

impl<Item: FromStr + Debug + Clone + Sized> FromStr for Map<Item> {
    type Err = <Item as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.trim().lines().collect();
        let width = lines[0].len() as i32;
        assert!(!lines.iter().any(|l| l.len() as i32 != width));
        let mut matrix = vec![];
        for line in lines {
            for c in line.chars() {
                let s = c.to_string();
                matrix.push(s.parse()?)
            }
        }
        Ok(Self { width, matrix })
    }
}

impl<Item: Debug + Clone + Sized> Map<Item> {
    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.matrix.len() as i32 / self.width
    }

    pub fn get(&self, Pos(x, y): Pos) -> Option<&Item> {
        if x < 0 || y < 0 || x >= self.width() || y >= self.height() {
            return None;
        }
        let idx = (self.width() * y + x) as usize;
        self.matrix.get(idx)
    }

    pub fn find_all<'a>(
        &'a self,
        mut predicate: impl FnMut(&Item) -> bool + 'a,
    ) -> impl Iterator<Item = Pos> + 'a {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| (x, y)))
            .filter_map(move |(x, y)| match self.get(Pos(x, y)) {
                Some(item) if predicate(item) => Some(Pos(x, y)),
                _ => None,
            })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Start,
    Path,
    Wall,
    End,
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Self::Start),
            "." => Ok(Self::Path),
            "#" => Ok(Self::Wall),
            "E" => Ok(Self::End),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT_1: &str = "\
        ###############\n\
        #.......#....E#\n\
        #.#.###.#.###.#\n\
        #.....#.#...#.#\n\
        #.###.#####.#.#\n\
        #.#.#.......#.#\n\
        #.#.#####.###.#\n\
        #...........#.#\n\
        ###.#.#####.#.#\n\
        #...#.....#.#.#\n\
        #.#.#.###.#.#.#\n\
        #.....#...#.#.#\n\
        #.###.#.#.#.#.#\n\
        #S..#.....#...#\n\
        ###############\n\
    ";

    static TEST_INPUT_2: &str = "\
        #################\n\
        #...#...#...#..E#\n\
        #.#.#.#.#.#.#.#.#\n\
        #.#.#.#...#...#.#\n\
        #.#.#.#.###.#.#.#\n\
        #...#.#.#.....#.#\n\
        #.#.#.#.#.#####.#\n\
        #.#...#.#.#.....#\n\
        #.#.#####.#.###.#\n\
        #.#.#.......#...#\n\
        #.#.###.#####.###\n\
        #.#.#...#.....#.#\n\
        #.#.#.#####.###.#\n\
        #.#.#.........#.#\n\
        #.#.#.#########.#\n\
        #S#.............#\n\
        #################\n\
    ";

    #[test]
    fn part1test() {
        assert_eq!(day16(TEST_INPUT_1).0, 7036);
        assert_eq!(day16(TEST_INPUT_2).0, 11048);
    }

    #[test]
    fn part2test() {
        assert_eq!(day16(TEST_INPUT_1).1, 45);
        assert_eq!(day16(TEST_INPUT_2).1, 64);
    }
}

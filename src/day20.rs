use std::{fmt::Debug, ops::Add, str::FromStr};

pub fn day20part1(input: &str) -> usize {
    let racetrack = RaceTrack::trace_map(input.parse().unwrap());
    let shortcuts = racetrack.find_shortcuts();
    shortcuts.iter().filter(|s| s.distance_saved >= 100).count()
}

#[derive(Debug, Clone)]
struct RaceTrack {
    pub map: Map<RaceTrackTile>,
    pub track: Vec<Pos>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Shortcut {
    pub distance_saved: i32,
    pub from: Pos,
    pub to: Pos,
}

impl RaceTrack {
    pub fn trace_map(mut map: Map<RaceTrackTile>) -> Self {
        let mut track = vec![];

        let start = map.find_all(|&t| t == RaceTrackTile::Start).next().unwrap();

        let mut pos = start;
        let mut dist = 0;

        loop {
            track.push(pos);
            match map.get(pos).copied() {
                Some(RaceTrackTile::End) => break,
                Some(RaceTrackTile::Start) => (),
                Some(RaceTrackTile::Unexplored) => {
                    map.set(pos, RaceTrackTile::Path(dist));
                }
                _ => unreachable!(),
            }

            const DIRS: [Direction; 4] = [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ];

            let mut next_pos = None;
            for dir in DIRS {
                let candidate = pos + dir;
                if matches!(
                    map.get(candidate),
                    Some(RaceTrackTile::Unexplored) | Some(RaceTrackTile::End)
                ) {
                    assert!(next_pos.is_none());
                    next_pos = Some(candidate);
                }
            }

            assert!(next_pos.is_some());
            pos = next_pos.unwrap();
            dist += 1;
        }

        Self { map, track }
    }

    pub fn find_shortcuts(&self) -> Vec<Shortcut> {
        let mut shortcuts = vec![];

        let dist_to_end = (self.track.len() - 1) as i32;

        for (i, &pos) in self.track.iter().enumerate() {
            let orig_dist = i as i32;
            let dist_after = orig_dist + 2; // after the cheat

            let possible_destinations = [
                pos + Direction::North + Direction::North,
                pos + Direction::North + Direction::East,
                pos + Direction::East + Direction::East,
                pos + Direction::East + Direction::South,
                pos + Direction::South + Direction::South,
                pos + Direction::South + Direction::West,
                pos + Direction::West + Direction::West,
                pos + Direction::West + Direction::North,
            ];

            for target in possible_destinations {
                match self.map.get(target) {
                    Some(RaceTrackTile::Path(d2)) if (d2 - dist_after) > 0 => {
                        shortcuts.push(Shortcut {
                            distance_saved: d2 - dist_after,
                            from: pos,
                            to: target
                        });
                    }
                    Some(RaceTrackTile::End) if (dist_to_end - dist_after) > 0 => {
                        shortcuts.push(Shortcut {
                            distance_saved: dist_to_end - dist_after,
                            from: pos,
                            to: target
                        });
                    }
                    _ => {}
                }
            }
        }

        shortcuts
    }
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

    pub fn get_mut(&mut self, Pos(x, y): Pos) -> Option<&mut Item> {
        if x < 0 || y < 0 || x >= self.width() || y >= self.height() {
            return None;
        }
        let idx = (self.width() * y + x) as usize;
        self.matrix.get_mut(idx)
    }

    pub fn set(&mut self, pos: Pos, value: Item) -> Option<Item> {
        if let Some(item) = self.get_mut(pos) {
            Some(std::mem::replace(item, value))
        } else {
            None
        }
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
enum RaceTrackTile {
    Start,
    End,
    Wall,
    Unexplored,
    Path(i32),
}

impl FromStr for RaceTrackTile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Self::Start),
            "." => Ok(Self::Unexplored),
            "#" => Ok(Self::Wall),
            "E" => Ok(Self::End),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use hashbrown::HashMap;

    use super::*;

    static TEST_INPUT: &str = "
        ###############\n\
        #...#...#.....#\n\
        #.#.#.#.#.###.#\n\
        #S#...#.#.#...#\n\
        #######.#.#.###\n\
        #######.#.#...#\n\
        #######.#.###.#\n\
        ###..E#...#...#\n\
        ###.#######.###\n\
        #...###...#...#\n\
        #.#####.#.###.#\n\
        #.#...#.#.#...#\n\
        #.#.#.#.#.#.###\n\
        #...#...#...###\n\
        ###############\n\
    ";

    #[test]
    fn part1test() {
        let racetrack = RaceTrack::trace_map(TEST_INPUT.parse().unwrap());
        assert_eq!(racetrack.track.len(), 85);

        let shortcuts = racetrack.find_shortcuts();
        let mut counts = HashMap::new();
        for s in shortcuts {
            *counts.entry(s.distance_saved).or_default() += 1;
        }

        assert_eq!(counts.get(&2), Some(&14));
        assert_eq!(counts.get(&4), Some(&14));
        assert_eq!(counts.get(&6), Some(&2));
        assert_eq!(counts.get(&8), Some(&4));
        assert_eq!(counts.get(&10), Some(&2));
        assert_eq!(counts.get(&12), Some(&3));
        assert_eq!(counts.get(&20), Some(&1));
        assert_eq!(counts.get(&36), Some(&1));
        assert_eq!(counts.get(&38), Some(&1));
        assert_eq!(counts.get(&40), Some(&1));
        assert_eq!(counts.get(&64), Some(&1));
    }
}

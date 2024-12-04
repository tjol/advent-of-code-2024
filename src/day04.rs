use std::{ops::Add, str::FromStr};

struct WordSearch {
    width: usize,
    matrix: Vec<char>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos(pub i32, pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Add<Direction> for Pos {
    type Output = Self;
    fn add(self, dir: Direction) -> Self::Output {
        let Self(x, y) = self;
        match dir {
            Direction::North => Self(x, y - 1),
            Direction::NorthEast => Self(x + 1, y - 1),
            Direction::East => Self(x + 1, y),
            Direction::SouthEast => Self(x + 1, y + 1),
            Direction::South => Self(x, y + 1),
            Direction::SouthWest => Self(x - 1, y + 1),
            Direction::West => Self(x - 1, y),
            Direction::NorthWest => Self(x - 1, y - 1),
        }
    }
}

impl WordSearch {
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

    pub fn get(&self, pos: Pos) -> Option<char> {
        if self.contains(pos) {
            let Pos(x, y) = pos;
            let idx = self.width() * y as usize + x as usize;
            Some(self.matrix[idx])
        } else {
            None
        }
    }

    pub fn find_in_dir(&self, needle: &str, dir: Direction) -> Vec<Pos> {
        let mut res = vec![];
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Pos(x as i32, y as i32);
                if self.test_word(pos, needle, dir) {
                    res.push(pos);
                }
            }
        }
        res
    }

    fn test_word(&self, mut pos: Pos, needle: &str, dir: Direction) -> bool {
        for c in needle.chars() {
            if self.get(pos) != Some(c) {
                return false;
            }
            pos = pos + dir;
        }
        true
    }

    pub fn find(&self, needle: &str) -> Vec<(Pos, Direction)> {
        let mut res = vec![];
        for dir in [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ] {
            for pos in self.find_in_dir(needle, dir) {
                res.push((pos, dir));
            }
        }
        res
    }

    pub fn find_x(&self, needle: &str) -> Vec<Pos> {
        let w = (needle.len() - 1) as i32;
        let mut res = vec![];
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos1 = Pos(x as i32, y as i32);
                let pos2 = Pos(x as i32 + w, y as i32);
                let pos3 = Pos(x as i32, y as i32 + w);
                let pos4 = Pos(x as i32 + w, y as i32 + w);
                if (self.test_word(pos1, needle, Direction::SouthEast)
                    && self.test_word(pos2, needle, Direction::SouthWest))
                    || (self.test_word(pos1, needle, Direction::SouthEast)
                        && self.test_word(pos3, needle, Direction::NorthEast))
                    || (self.test_word(pos2, needle, Direction::SouthWest)
                        && self.test_word(pos4, needle, Direction::NorthWest))
                    || (self.test_word(pos3, needle, Direction::NorthEast)
                        && self.test_word(pos4, needle, Direction::NorthWest))
                {
                    res.push(pos1);
                }
            }
        }
        res
    }
}

impl FromStr for WordSearch {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let width = lines.iter().map(|s| s.len()).max().unwrap();
        let height = lines.len();
        let mut matrix = vec![' '; height * width];
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let idx = x + y * width;
                matrix[idx] = c;
            }
        }
        Ok(Self { width, matrix })
    }
}

pub fn day04part1(input: &str) -> usize {
    let ws: WordSearch = input.parse().unwrap();
    ws.find("XMAS").len()
}

pub fn day04part2(input: &str) -> usize {
    let ws: WordSearch = input.parse().unwrap();
    ws.find_x("MAS").len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1test() {
        let input = "\
            MMMSXXMASM\n\
            MSAMXMSMSA\n\
            AMXSXMAAMM\n\
            MSAMASMSMX\n\
            XMASAMXAMM\n\
            XXAMMXXAMA\n\
            SMSMSASXSS\n\
            SAXAMASAAA\n\
            MAMMMXMMMM\n\
            MXMXAXMASX\n\
            ";
        assert_eq!(day04part1(input), 18);
    }

    #[test]
    fn part2test() {
        let input = "\
            MMMSXXMASM\n\
            MSAMXMSMSA\n\
            AMXSXMAAMM\n\
            MSAMASMSMX\n\
            XMASAMXAMM\n\
            XXAMMXXAMA\n\
            SMSMSASXSS\n\
            SAXAMASAAA\n\
            MAMMMXMMMM\n\
            MXMXAXMASX\n\
            ";
        assert_eq!(day04part2(input), 9);
    }
}

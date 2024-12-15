use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

pub fn day15part1(input: &str) -> i32 {
    let (map_input, program) = input.split_once("\n\n").unwrap();

    let mut map: Map<Tile> = map_input.parse().unwrap();
    let moves = parse_moves(program);

    let (mut x, mut y) = map.find_robot().unwrap();

    for the_move in moves {
        if let Some((new_x, new_y)) = map.try_move((x, y), the_move) {
            x = new_x;
            y = new_y;
        }
    }

    map.find_all(|&obj| obj == Tile::Box)
        .map(|(x, y)| x + 100 * y)
        .sum::<i32>()
}

pub fn day15part2(input: &str) -> i32 {
    let (map_input, program) = input.split_once("\n\n").unwrap();

    let orig_map: Map<Tile> = map_input.parse().unwrap();
    let mut map = orig_map.double();
    let moves = parse_moves(program);

    let (mut x, mut y) = map.find_robot().unwrap();

    for the_move in moves {
        if let Some((new_x, new_y)) = map.try_move((x, y), the_move) {
            x = new_x;
            y = new_y;
        }
    }

    map.find_all(|&obj| obj == Tile2::LeftBox)
        .map(|(x, y)| x + 100 * y)
        .sum::<i32>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Box,
    Wall,
    Robot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile2 {
    Empty,
    LeftBox,
    RightBox,
    Wall,
    Robot,
}

impl From<Tile> for (Tile2, Tile2) {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Empty => (Tile2::Empty, Tile2::Empty),
            Tile::Box => (Tile2::LeftBox, Tile2::RightBox),
            Tile::Wall => (Tile2::Wall, Tile2::Wall),
            Tile::Robot => (Tile2::Robot, Tile2::Empty),
        }
    }
}

impl Display for Tile2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(
            &match self {
                Tile2::Empty => '.',
                Tile2::LeftBox => '[',
                Tile2::RightBox => ']',
                Tile2::Wall => '#',
                Tile2::Robot => '@',
            },
            f,
        )
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Empty),
            "O" => Ok(Self::Box),
            "#" => Ok(Self::Wall),
            "@" => Ok(Self::Robot),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Self::Up),
            'v' => Some(Self::Down),
            '<' => Some(Self::Left),
            '>' => Some(Self::Right),
            _ => None,
        }
    }

    pub fn x(self) -> i32 {
        match self {
            Self::Right => 1,
            Self::Left => -1,
            _ => 0,
        }
    }

    pub fn y(self) -> i32 {
        match self {
            Self::Down => 1,
            Self::Up => -1,
            _ => 0,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(
            &match self {
                Self::Up => '^',
                Self::Down => 'v',
                Self::Left => '<',
                Self::Right => '>',
            },
            f,
        )
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    input.chars().filter_map(Move::from_char).collect()
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

    pub fn get(&self, x: i32, y: i32) -> Option<&Item> {
        if x < 0 || y < 0 || x >= self.width() || y >= self.height() {
            return None;
        }
        let idx = (self.width() * y + x) as usize;
        self.matrix.get(idx)
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Item> {
        if x < 0 || y < 0 || x >= self.width() || y >= self.height() {
            return None;
        }
        let idx = (self.width() * y + x) as usize;
        self.matrix.get_mut(idx)
    }

    pub fn find_all<'a>(
        &'a self,
        mut predicate: impl FnMut(&Item) -> bool + 'a,
    ) -> impl Iterator<Item = (i32, i32)> + 'a {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| (x, y)))
            .filter_map(move |(x, y)| match self.get(x, y) {
                Some(item) if predicate(item) => Some((x, y)),
                _ => None,
            })
    }
}

impl Map<Tile> {
    pub fn find_robot(&self) -> Option<(i32, i32)> {
        self.find_all(|&x| x == Tile::Robot).next()
    }

    pub fn try_move(&mut self, from: (i32, i32), direction: Move) -> Option<(i32, i32)> {
        let (x0, y0) = from;

        match self.get(x0, y0).copied() {
            None | Some(Tile::Empty) | Some(Tile::Wall) => {
                // can't move this!
                None
            }
            Some(obj) => {
                // check the new location
                let x1 = x0 + direction.x();
                let y1 = y0 + direction.y();

                match self.get(x1, y1).copied() {
                    None => {
                        // out of bounds
                        return None;
                    }
                    Some(Tile::Empty) => {
                        // we can move here (fall through)
                    }
                    Some(_obstacle) => {
                        // try tot move the obstace
                        self.try_move((x1, y1), direction)?;
                    }
                }
                // we can move!
                *self.get_mut(x1, y1).unwrap() = obj;
                *self.get_mut(x0, y0).unwrap() = Tile::Empty;
                Some((x1, y1))
            }
        }
    }

    pub fn double(&self) -> Map<Tile2> {
        let width = self.width * 2;
        let mut matrix = vec![];
        for &tile in &self.matrix {
            let (l, r) = tile.into();
            matrix.push(l);
            matrix.push(r);
        }
        Map::<Tile2> { width, matrix }
    }
}

impl Map<Tile2> {
    pub fn find_robot(&self) -> Option<(i32, i32)> {
        self.find_all(|&x| x == Tile2::Robot).next()
    }

    pub fn try_move(&mut self, from: (i32, i32), direction: Move) -> Option<(i32, i32)> {
        if !self.can_move(from, direction) {
            return None;
        }

        let (x0, y0) = from;

        match self.get(x0, y0).copied() {
            None | Some(Tile2::Wall) => unreachable!(),
            Some(Tile2::Empty) => Some((x0, y0)),
            Some(Tile2::Robot) => {
                let x1 = x0 + direction.x();
                let y1 = y0 + direction.y();
                self.try_move((x1, y1), direction)?;
                *self.get_mut(x1, y1).unwrap() = Tile2::Robot;
                *self.get_mut(x0, y0).unwrap() = Tile2::Empty;
                Some((x1, y1))
            }
            Some(Tile2::LeftBox) => match direction {
                Move::Left => {
                    self.try_move((x0 - 1, y0), Move::Left)?;
                    *self.get_mut(x0 - 1, y0).unwrap() = Tile2::LeftBox;
                    *self.get_mut(x0, y0).unwrap() = Tile2::RightBox;
                    *self.get_mut(x0 + 1, y0).unwrap() = Tile2::Empty;
                    Some((x0 - 1, y0))
                }
                Move::Right => {
                    self.try_move((x0 + 2, y0), Move::Right)?;
                    *self.get_mut(x0 + 1, y0).unwrap() = Tile2::LeftBox;
                    *self.get_mut(x0 + 2, y0).unwrap() = Tile2::RightBox;
                    *self.get_mut(x0, y0).unwrap() = Tile2::Empty;
                    Some((x0 + 1, y0))
                }
                Move::Up => {
                    self.try_move((x0, y0 - 1), Move::Up)?;
                    self.try_move((x0 + 1, y0 - 1), Move::Up)?;
                    *self.get_mut(x0, y0 - 1).unwrap() = Tile2::LeftBox;
                    *self.get_mut(x0 + 1, y0 - 1).unwrap() = Tile2::RightBox;
                    *self.get_mut(x0, y0).unwrap() = Tile2::Empty;
                    *self.get_mut(x0 + 1, y0).unwrap() = Tile2::Empty;
                    Some((x0, y0 - 1))
                }
                Move::Down => {
                    self.try_move((x0, y0 + 1), Move::Down)?;
                    self.try_move((x0 + 1, y0 + 1), Move::Down)?;
                    *self.get_mut(x0, y0 + 1).unwrap() = Tile2::LeftBox;
                    *self.get_mut(x0 + 1, y0 + 1).unwrap() = Tile2::RightBox;
                    *self.get_mut(x0, y0).unwrap() = Tile2::Empty;
                    *self.get_mut(x0 + 1, y0).unwrap() = Tile2::Empty;
                    Some((x0, y0 + 1))
                }
            },
            Some(Tile2::RightBox) => self.try_move((x0 - 1, y0), direction),
        }
    }

    pub fn can_move(&self, from: (i32, i32), direction: Move) -> bool {
        let (x0, y0) = from;
        let x1 = x0 + direction.x();
        let y1 = y0 + direction.y();

        match self.get(x0, y0).copied() {
            None | Some(Tile2::Wall) => false,
            Some(Tile2::Empty) => true,
            Some(Tile2::Robot) => self.can_move((x1, y1), direction),
            Some(Tile2::LeftBox) => {
                if direction.y() == 0 {
                    self.can_move((x1, y1), direction)
                } else {
                    self.can_move((x1, y1), direction) && self.can_move((x1 + 1, y1), direction)
                }
            }
            Some(Tile2::RightBox) => {
                if direction.y() == 0 {
                    self.can_move((x1, y1), direction)
                } else {
                    self.can_move((x1, y1), direction) && self.can_move((x1 - 1, y1), direction)
                }
            }
        }
    }
}

impl Display for Map<Tile2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                Display::fmt(self.get(x, y).unwrap(), f)?;
            }
            Display::fmt(&'\n', f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "\
        ##########\n\
        #..O..O.O#\n\
        #......O.#\n\
        #.OO..O.O#\n\
        #..O@..O.#\n\
        #O#..O...#\n\
        #O..O..O.#\n\
        #.OO.O.OO#\n\
        #....O...#\n\
        ##########\n\
        \n\
        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\n\
    ";

    #[test]
    fn part1test() {
        assert_eq!(day15part1(TEST_INPUT), 10092);
    }

    #[test]
    fn part2test() {
        assert_eq!(day15part2(TEST_INPUT), 9021);
    }
}

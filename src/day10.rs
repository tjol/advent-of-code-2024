use std::{collections::VecDeque, fmt::Debug, str::FromStr};

pub fn day10part1(input: &str) -> usize {
    let map: Map<Height> = input.parse().unwrap();

    let peaks: Vec<_> = map.find_all(|&h| h == Height(9)).collect();
    let trailheads: Vec<_> = map.find_all(|&h| h == Height(0)).collect();

    let mut trailhead_scores = vec![0; trailheads.len()];

    for &(peak_x, peak_y) in &peaks {
        let peak_map = backtrack_from_peak(&map, peak_x, peak_y);
        for (i, &(th_x, th_y)) in trailheads.iter().enumerate() {
            if *peak_map.get(th_x, th_y).unwrap() > 0 {
                trailhead_scores[i] += 1;
            }
        }
    }

    trailhead_scores.iter().sum()
}

pub fn day10part2(input: &str) -> usize {
    let map: Map<Height> = input.parse().unwrap();

    let peaks: Vec<_> = map.find_all(|&h| h == Height(9)).collect();
    let trailheads: Vec<_> = map.find_all(|&h| h == Height(0)).collect();

    let mut trailhead_scores = vec![0; trailheads.len()];

    for &(peak_x, peak_y) in &peaks {
        let peak_map = backtrack_from_peak(&map, peak_x, peak_y);
        for (i, &(th_x, th_y)) in trailheads.iter().enumerate() {
            let count = *peak_map.get(th_x, th_y).unwrap();
            trailhead_scores[i] += count;
        }
    }

    trailhead_scores.iter().sum()
}

fn backtrack_from_peak(map: &Map<Height>, peak_x: usize, peak_y: usize) -> Map<usize> {
    let mut path_map = Map::default(map.width(), map.height());

    let mut queue = VecDeque::new();
    queue.push_back((peak_x, peak_y));

    loop {
        if queue.is_empty() {
            break;
        }

        let (x, y) = queue.pop_front().unwrap();
        *path_map.get_mut(x, y).unwrap() += 1;
        let height = *map.get(x, y).unwrap();

        if map.get(x + 1, y).is_some_and(|h| h.can_walk_up_to(height)) {
            queue.push_back((x + 1, y));
        }
        if x > 0 && map.get(x - 1, y).is_some_and(|h| h.can_walk_up_to(height)) {
            queue.push_back((x - 1, y));
        }
        if map.get(x, y + 1).is_some_and(|h| h.can_walk_up_to(height)) {
            queue.push_back((x, y + 1));
        }
        if y > 0 && map.get(x, y - 1).is_some_and(|h| h.can_walk_up_to(height)) {
            queue.push_back((x, y - 1));
        }
    }

    path_map
}

#[derive(Debug, Clone)]
struct Map<Item>
where
    Item: Debug + Clone + Sized,
{
    width: usize,
    matrix: Vec<Item>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Height(u8);

impl Height {
    fn can_walk_up_to(self, other: Height) -> bool {
        other.0 as i16 - self.0 as i16 == 1
    }
}

impl FromStr for Height {
    type Err = <u8 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <u8 as FromStr>::from_str(s).map(Self)
    }
}

impl<Item: FromStr + Debug + Clone + Sized> FromStr for Map<Item> {
    type Err = <Item as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.trim().lines().collect();
        let width = lines[0].len();
        assert!(!lines.iter().any(|l| l.len() != width));
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
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.matrix.len() / self.width
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Item> {
        if x >= self.width() || y >= self.height() {
            return None;
        }
        let idx = self.width() * y + x;
        self.matrix.get(idx)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Item> {
        if x >= self.width() || y >= self.height() {
            return None;
        }
        let idx = self.width() * y + x;
        self.matrix.get_mut(idx)
    }

    fn find_all<'a>(
        &'a self,
        mut predicate: impl FnMut(&Item) -> bool + 'a,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| (x, y)))
            .filter_map(move |(x, y)| match self.get(x, y) {
                Some(item) if predicate(item) => Some((x, y)),
                _ => None,
            })
    }
}

impl<Item: Default + Debug + Clone + Sized> Map<Item> {
    pub fn default(width: usize, height: usize) -> Self {
        let mut matrix = Vec::new();
        matrix.resize_with(width * height, Default::default);
        Self { width, matrix }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1test() {
        let input = "\
            89010123\n\
            78121874\n\
            87430965\n\
            96549874\n\
            45678903\n\
            32019012\n\
            01329801\n\
            10456732\n\
        ";

        assert_eq!(day10part1(input), 36);
    }

    #[test]
    fn part2test() {
        let input = "\
            89010123\n\
            78121874\n\
            87430965\n\
            96549874\n\
            45678903\n\
            32019012\n\
            01329801\n\
            10456732\n\
        ";

        assert_eq!(day10part2(input), 81);
    }
}

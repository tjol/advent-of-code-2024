use std::{
    collections::{BTreeSet, VecDeque},
    fmt::Debug,
    str::FromStr,
};

pub fn day12part1(input: &str) -> i32 {
    let mut map: Map<Plot> = input.parse().unwrap();

    let mut next_region = 1;

    let mut regions = vec![];

    for y in 0..map.height() {
        for x in 0..map.width() {
            if map.get(x, y).unwrap().region.is_none() {
                let (map_, region) = map_region(map, x, y, next_region);
                map = map_;
                regions.push(region);
                next_region += 1;
            }
        }
    }

    regions.iter().map(|r| r.area * r.perimeter).sum()
}

pub fn day12part2(input: &str) -> i32 {
    let mut map: Map<Plot> = input.parse().unwrap();

    let mut next_region = 1;

    let mut regions = vec![];

    for y in 0..map.height() {
        for x in 0..map.width() {
            if map.get(x, y).unwrap().region.is_none() {
                let (map_, region) = map_region(map, x, y, next_region);
                map = map_;
                regions.push(region);
                next_region += 1;
            }
        }
    }

    regions
        .iter()
        .map(|r| count_sides(&map, r.region) * r.area)
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RegionSize {
    pub region: i32,
    pub area: i32,
    pub perimeter: i32,
}

fn map_region(
    mut map: Map<Plot>,
    start_x: usize,
    start_y: usize,
    region: i32,
) -> (Map<Plot>, RegionSize) {
    let crop = map.get(start_x, start_y).unwrap().crop;
    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));

    let mut area = 0;
    let mut perimeter = 0;

    loop {
        if queue.is_empty() {
            break;
        }

        let (x, y) = queue.pop_front().unwrap();
        let here = map.get_mut(x, y).unwrap();
        if here.region.is_some() {
            continue;
        } else {
            here.region = Some(region);
            area += 1;
        }

        match map.get(x + 1, y) {
            Some(plot) if plot.crop == crop => {
                queue.push_back((x + 1, y));
            }
            _ => {
                perimeter += 1;
            }
        }
        if x > 0 {
            match map.get(x - 1, y) {
                Some(plot) if plot.crop == crop => {
                    queue.push_back((x - 1, y));
                }
                _ => {
                    perimeter += 1;
                }
            }
        } else {
            perimeter += 1
        }
        match map.get(x, y + 1) {
            Some(plot) if plot.crop == crop => {
                queue.push_back((x, y + 1));
            }
            _ => {
                perimeter += 1;
            }
        }
        if y > 0 {
            match map.get(x, y - 1) {
                Some(plot) if plot.crop == crop => {
                    queue.push_back((x, y - 1));
                }
                _ => {
                    perimeter += 1;
                }
            }
        } else {
            perimeter += 1
        }
    }

    (
        map,
        RegionSize {
            region,
            area,
            perimeter,
        },
    )
}

fn count_sides(map: &Map<Plot>, region: i32) -> i32 {
    // find all the top edges
    let mut top_edges = vec![];

    for y in 0..map.height() {
        for x in 0..map.width() {
            if map.get(x, y).unwrap().region == Some(region)
                && (y == 0 || map.get(x, y - 1).unwrap().region != Some(region))
            {
                top_edges.push((x, y))
            }
        }
    }

    let mut visited_top_edges = BTreeSet::new();
    let mut sides = 0;

    for (x0, y0) in top_edges {
        if visited_top_edges.contains(&(x0, y0)) {
            continue;
        }

        // Thanks to the search order I know we're at the (or *a*) top-left corner!
        let mut edge = Edge::Top;
        // walk around the bloody thing clockwise
        let (mut x, mut y) = (x0, y0);

        loop {
            match edge {
                Edge::Top => {
                    visited_top_edges.insert((x, y));
                    if y > 0
                        && map
                            .get(x + 1, y - 1)
                            .is_some_and(|p| p.region == Some(region))
                    {
                        // .2
                        // 1?
                        edge = Edge::Left;
                        x += 1;
                        y -= 1;
                        sides += 1;
                    } else if map.get(x + 1, y).is_some_and(|p| p.region == Some(region)) {
                        // . .
                        // 1 2
                        x += 1;
                    } else {
                        // . .
                        // 1 .
                        edge = Edge::Right;
                        sides += 1;
                    }
                }
                Edge::Right => {
                    if map
                        .get(x + 1, y + 1)
                        .is_some_and(|p| p.region == Some(region))
                    {
                        // 1 .
                        // ? 2
                        edge = Edge::Top;
                        x +=  1;
                        y += 1;
                        sides += 1;
                    } else if map.get(x, y + 1).is_some_and(|p| p.region == Some(region)) {
                        // 1 .
                        // 2 .
                        y += 1;
                    } else {
                        // 1 .
                        // . .
                        edge = Edge::Bottom;
                        sides += 1;
                    }
                }
                Edge::Bottom => {
                    if x > 0
                        && map
                            .get(x - 1, y + 1)
                            .is_some_and(|p| p.region == Some(region))
                    {
                        // ? 1
                        // 2 .
                        edge = Edge::Right;
                        x -= 1;
                        y += 1;
                        sides += 1;
                    } else if x > 0 && map.get(x - 1, y).is_some_and(|p| p.region == Some(region)) {
                        // 2 1
                        // . .
                        x -= 1;
                    } else {
                        // . 1
                        // . .
                        edge = Edge::Left;
                        sides += 1;
                    }
                }
                Edge::Left => {
                    if x > 0
                        && y > 0
                        && map
                            .get(x - 1, y - 1)
                            .is_some_and(|p| p.region == Some(region))
                    {
                        // 2 ?
                        // . 1
                        edge = Edge::Bottom;
                        x -= 1;
                        y -= 1;
                        sides += 1;
                    } else if y > 0 && map.get(x, y - 1).is_some_and(|p| p.region == Some(region)) {
                        // . 2
                        // . 1
                        y -= 1;
                    } else {
                        // . .
                        // . 1
                        edge = Edge::Top;
                        sides += 1;
                    }
                }
            }

            if x == x0 && y == y0 && edge == Edge::Top {
                // after moving, we're back at the start
                break;
            }
        }
    }

    sides
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Plot {
    pub crop: char,
    pub region: Option<i32>,
}

impl FromStr for Plot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            crop: s.chars().next().unwrap(),
            region: None,
        })
    }
}

#[derive(Debug, Clone)]
struct Map<Item>
where
    Item: Debug + Clone + Sized,
{
    width: usize,
    matrix: Vec<Item>,
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
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "\
        RRRRIICCFF\n\
        RRRRIICCCF\n\
        VVRRRCCFFF\n\
        VVRCCCJFFF\n\
        VVVVCJJCFE\n\
        VVIVCCJJEE\n\
        VVIIICJJEE\n\
        MIIIIIJJEE\n\
        MIIISIJEEE\n\
        MMMISSJEEE\n\
    ";

    #[test]
    fn part1test() {
        assert_eq!(day12part1(TEST_INPUT), 1930);
    }

    #[test]
    fn part2test() {
        assert_eq!(day12part2(TEST_INPUT), 1206);
    }

    #[test]
    fn enclave_test() {
        let input: &str = "\
            AAAAAAAA\n\
            AAA.A~AA\n\
            //A.AA//\n\
            //AAAA//\n\
        ";
        let map = input.parse().unwrap();
        let (map, _) = map_region(map, 0, 0, 1);
        assert_eq!(count_sides(&map, 1), 16);
    }
}

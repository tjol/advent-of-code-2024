use hashbrown::HashMap;

pub fn day21part1(input: &str) -> usize {
    let mut complexity = 0;

    let codes = input.trim().lines();

    for code in codes {
        let numeric_part: usize = code[..code.len() - 1].parse().unwrap();

        let cost = cost_to_enter_code(code, 2);

        complexity += cost * numeric_part;
    }

    complexity
}

pub fn day21part2(input: &str) -> usize {
    let mut complexity = 0;

    let codes = input.trim().lines();

    for code in codes {
        let numeric_part: usize = code[..code.len() - 1].parse().unwrap();

        let cost = cost_to_enter_code(code, 25);

        complexity += cost * numeric_part;
    }

    complexity
}

fn cost_to_enter_code(code: &str, n_robots: usize) -> usize {
    let robot_stack = RobotStack::new(n_robots);
    let kp = Keypad::numeric();
    let mut prev = 'A';
    let mut cost = 0;
    for c in code.chars() {
        cost += robot_stack.cost_to_move_and_press(&kp, prev, c);
        prev = c;
    }
    cost
}

struct RobotStack {
    /// (last thing we did, thing we want to do) -> cost of action
    costs: HashMap<(char, char), usize>,
}

impl RobotStack {
    pub fn empty() -> Self {
        let costs = "><^vA"
            .chars()
            .flat_map(|from| "><^vA".chars().map(move |to| ((from, to), 1)))
            .collect();

        Self { costs }
    }

    pub fn add_robot(implement: &RobotStack) -> Self {
        let keypad = Keypad::directional();
        let mut costs = HashMap::new();
        for from in "><^vA".chars() {
            for to in "><^vA".chars() {
                let cost = implement.cost_to_move_and_press(&keypad, from, to);
                costs.insert((from, to), cost);
            }
        }

        Self { costs }
    }

    pub fn new(depth: usize) -> Self {
        let mut stack = RobotStack::empty();
        for _ in 0..depth {
            let next_layer = RobotStack::add_robot(&stack);
            stack = next_layer;
        }
        stack
    }

    // we're hovering at "from" and want to press "sym"
    pub fn cost_to_move_and_press(&self, kp: &Keypad, from: char, sym: char) -> usize {
        kp.get_sequences(from, sym)
            .into_iter()
            .map(|s| self.cost_of_seq(&s))
            .min()
            .unwrap()
    }

    /// the cost to perform a sequence of actions
    pub fn cost_of_seq(&self, seq: &str) -> usize {
        let mut prev = 'A';
        let mut cost = 0;
        for c in seq.chars() {
            cost += self.costs.get(&(prev, c)).unwrap();
            prev = c;
        }
        cost
    }
}

#[derive(Debug, Clone)]
struct Keypad {
    keys: HashMap<char, (i8, i8)>,
    blank: (i8, i8),
}

impl Keypad {
    pub fn numeric() -> Self {
        let keys = [
            ('0', (1, 0)),
            ('A', (2, 0)),
            ('1', (0, 1)),
            ('2', (1, 1)),
            ('3', (2, 1)),
            ('4', (0, 2)),
            ('5', (1, 2)),
            ('6', (2, 2)),
            ('7', (0, 3)),
            ('8', (1, 3)),
            ('9', (2, 3)),
        ]
        .into_iter()
        .collect();
        let blank = (0, 0);

        Self { keys, blank }
    }

    pub fn directional() -> Self {
        let keys = [
            ('<', (0, 0)),
            ('v', (1, 0)),
            ('>', (2, 0)),
            ('^', (1, 1)),
            ('A', (2, 1)),
        ]
        .into_iter()
        .collect();
        let blank = (0, 1);

        Self { keys, blank }
    }

    /// Assuming your robot is pointing at `from`, what would you have to enter
    /// in the d-pad to press `key`
    ///
    /// Returns all direct L-shaped options
    pub fn get_sequences(&self, from: char, key: char) -> Vec<String> {
        let from = *self.keys.get(&from).unwrap();
        let dest = *self.keys.get(&key).unwrap();
        let delta = (dest.0 - from.0, dest.1 - from.1);

        let mut paths = vec![];

        let x_sym = ['<', '>'][if delta.0 >= 0 { 1 } else { 0 }];
        let y_sym = ['v', '^'][if delta.1 >= 0 { 1 } else { 0 }];
        let x_subseq: String = std::iter::repeat_n(x_sym, delta.0.abs() as usize).collect();
        let y_subseq: String = std::iter::repeat_n(y_sym, delta.1.abs() as usize).collect();

        if self.blank.0 == from.0 && self.blank.1 == dest.1 {
            // must move x first
            paths.push(format!("{}{}A", &x_subseq, &y_subseq));
        } else if self.blank.1 == from.1 && self.blank.0 == dest.0 {
            // must move y first
            paths.push(format!("{}{}A", &y_subseq, &x_subseq));
        } else {
            // either way is fine
            paths.push(format!("{}{}A", &y_subseq, &x_subseq));
            if delta.0 != 0 && delta.1 != 0 {
                paths.push(format!("{}{}A", &x_subseq, &y_subseq));
            }
        }

        paths
    }

    #[allow(unused)]
    pub fn replay(&self, moves: &str) -> Option<String> {
        let keys_by_pos: HashMap<_, _> = self.keys.iter().map(|(&k, &v)| (v, k)).collect();

        let mut output = String::new();
        let mut pos = *self.keys.get(&'A').unwrap();

        for m in moves.chars() {
            if m == 'A' {
                output.push(*keys_by_pos.get(&pos)?);
            } else {
                let unit_vec = match m {
                    '>' => (1, 0),
                    '<' => (-1, 0),
                    '^' => (0, 1),
                    'v' => (0, -1),
                    _ => return None,
                };
                pos = (pos.0 + unit_vec.0, pos.1 + unit_vec.1);
                if !keys_by_pos.contains_key(&pos) {
                    return None;
                }
            }
        }
        Some(output)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "
        029A\n\
        980A\n\
        179A\n\
        456A\n\
        379A\n\
    ";

    #[test]
    fn part1test() {
        assert_eq!(day21part1(TEST_INPUT), 126384);
    }

    #[test]
    fn get_sequences_test() {
        let numpad = Keypad::numeric();
        let dpad = Keypad::directional();

        let paths_to_3 = numpad.get_sequences('A', '3');
        assert_eq!(&paths_to_3, &["^A".to_string()]);
        let mut paths_to_2 = numpad.get_sequences('A', '2');
        paths_to_2.sort();
        assert_eq!(&paths_to_2, &["<^A".to_string(), "^<A".to_string()]);
        let paths_to_1 = numpad.get_sequences('A', '1');
        assert_eq!(&paths_to_1, &["^<<A".to_string()]);
        let mut paths_1_to_9 = numpad.get_sequences('1', '9');
        paths_1_to_9.sort();
        assert_eq!(&paths_1_to_9, &[">>^^A".to_string(), "^^>>A".to_string()]);

        let paths_to_left = dpad.get_sequences('A', '<');
        assert_eq!(&paths_to_left, &["v<<A".to_string()]);
    }

    #[test]
    fn robot_stack_test() {
        let solo = RobotStack::new(1);
        assert_eq!(solo.cost_of_seq("^"), "<A".len());
        assert_eq!(solo.cost_of_seq("^^"), "<AA".len());
        assert_eq!(solo.cost_of_seq("^A"), "<A>A".len());
        assert_eq!(solo.cost_of_seq("vA"), "<vA>^A".len());
        assert_eq!(solo.cost_of_seq("<A"), "v<<A>>^A".len());

        let duo = RobotStack::new(2);
        assert_eq!(duo.cost_of_seq("^"), "v<<A>>^A".len());
        assert_eq!(duo.cost_of_seq("<A"), "v<A<AA>>^A<A>vAA^A".len());
    }

    #[test]
    fn part1_examples_1() {
        assert_eq!(cost_to_enter_code("029A", 0), "<A^A>^^AvvvA".len());
        assert_eq!(
            cost_to_enter_code("029A", 1),
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len()
        );
        assert_eq!(
            cost_to_enter_code("029A", 2),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn part1_examples_2() {
        let examples = [
            (
                "029A",
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
            ),
            (
                "980A",
                "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A",
            ),
            (
                "179A",
                "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            ),
            (
                "456A",
                "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A",
            ),
            (
                "379A",
                "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            ),
        ];

        for (code, ref_seq) in examples {
            let cost = cost_to_enter_code(code, 2);
            assert_eq!(cost, ref_seq.len());
        }
    }
}

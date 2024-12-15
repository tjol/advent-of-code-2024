use std::{fmt::Display, fs::File, io::Read, path::Path};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let day: i32 = args[1].parse().unwrap();
    match day {
        1 => run_puzzle((day01::day01part1, day01::day01part2), &args[2..]),
        2 => run_puzzle((day02::day02part1, day02::day02part2), &args[2..]),
        3 => run_puzzle((day03::day03part1, day03::day03part2), &args[2..]),
        4 => run_puzzle((day04::day04part1, day04::day04part2), &args[2..]),
        5 => run_puzzle((day05::day05part1, day05::day05part2), &args[2..]),
        6 => run_puzzle((day06::day06part1, day06::day06part2), &args[2..]),
        7 => run_puzzle((day07::day07part1, day07::day07part2), &args[2..]),
        8 => run_puzzle((day08::day08part1, day08::day08part2), &args[2..]),
        9 => run_puzzle((day09::day09part1, day09::day09part2), &args[2..]),
        10 => run_puzzle((day10::day10part1, day10::day10part2), &args[2..]),
        11 => run_puzzle((day11::day11part1, day11::day11part2), &args[2..]),
        12 => run_puzzle((day12::day12part1, day12::day12part2), &args[2..]),
        13 => run_puzzle((day13::day13part1, day13::day13part2), &args[2..]),
        14 => run_puzzle((day14::day14part1, day14::day14part2), &args[2..]),
        15 => run_puzzle((day15::day15part1, day15::day15part2), &args[2..]),
        _ => panic!("no such day"),
    }
}

trait AdventPuzzle: Sized {
    fn run(self, input: &str) -> String;

    fn run_from_file(self, mut file: impl Read) -> String {
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        self.run(&s)
    }

    fn run_from_path(self, path: impl AsRef<Path>) -> String {
        let file = File::open(path).unwrap();
        self.run_from_file(file)
    }

    fn run_from_stdin(self) -> String {
        let stdin = std::io::stdin();
        self.run_from_file(stdin)
    }
}

impl<F, R> AdventPuzzle for F
where
    F: FnOnce(&str) -> R,
    R: Display,
{
    fn run(self, input: &str) -> String {
        self(input).to_string()
    }
}

impl<P1, P2> AdventPuzzle for (P1, P2)
where
    P1: AdventPuzzle,
    P2: AdventPuzzle,
{
    fn run(self, input: &str) -> String {
        let (p1, p2) = self;
        format!("{}\n{}", p1.run(input), p2.run(input))
    }
}

fn run_puzzle(solution: impl AdventPuzzle, args: &[String]) {
    let answer = if args.is_empty() || args[0] == "-" {
        solution.run_from_stdin()
    } else {
        solution.run_from_path(&args[0])
    };
    println!("{}", &answer);
}

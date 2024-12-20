use std::{fmt::Display, fs::File, io::Read, path::Path, time::Instant};

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
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;

fn solution(
    solution: impl AdventPuzzleSolution + 'static,
    path: &'static str,
) -> Option<(Box<dyn AdventPuzzleSolution>, &'static str)> {
    Some((Box::new(solution), path))
}

fn get_solution(day: i8) -> Option<(Box<dyn AdventPuzzleSolution>, &'static str)> {
    match day {
        1 => solution((day01::day01part1, day01::day01part2), "inputs/day01.txt"),
        2 => solution((day02::day02part1, day02::day02part2), "inputs/day02.txt"),
        3 => solution((day03::day03part1, day03::day03part2), "inputs/day03.txt"),
        4 => solution((day04::day04part1, day04::day04part2), "inputs/day04.txt"),
        5 => solution((day05::day05part1, day05::day05part2), "inputs/day05.txt"),
        6 => solution((day06::day06part1, day06::day06part2), "inputs/day06.txt"),
        7 => solution((day07::day07part1, day07::day07part2), "inputs/day07.txt"),
        8 => solution((day08::day08part1, day08::day08part2), "inputs/day08.txt"),
        9 => solution((day09::day09part1, day09::day09part2), "inputs/day09.txt"),
        10 => solution((day10::day10part1, day10::day10part2), "inputs/day10.txt"),
        11 => solution((day11::day11part1, day11::day11part2), "inputs/day11.txt"),
        12 => solution((day12::day12part1, day12::day12part2), "inputs/day12.txt"),
        13 => solution((day13::day13part1, day13::day13part2), "inputs/day13.txt"),
        14 => solution((day14::day14part1, day14::day14part2), "inputs/day14.txt"),
        15 => solution((day15::day15part1, day15::day15part2), "inputs/day15.txt"),
        16 => solution(CombinedSolution { func: day16::day16 }, "inputs/day16.txt"),
        17 => solution((day17::day17part1, day17::day17part2), "inputs/day17.txt"),
        18 => solution((day18::day18part1, day18::day18part2), "inputs/day18.txt"),
        19 => solution(CombinedSolution { func: day19::day19 }, "inputs/day19.txt"),
        20 => solution((day20::day20part1, day20::day20part2), "inputs/day20.txt"),
        _ => None,
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() >= 2 {
        let day = args[1].parse().unwrap();
        let (solution, default_input) = get_solution(day).unwrap();
        let input = args.get(2).map(|s| s.as_str()).unwrap_or(default_input);
        run_puzzle(&*solution, input);
    } else {
        let mut day = 1;
        while let Some((solution, input)) = get_solution(day) {
            println!(" 🎄 DAY {:2} 🎄", day);
            run_puzzle(&*solution, input);
            day += 1;
        }
    }
}

trait AdventPuzzleSolution {
    fn run(&self, input: &str) -> String;
}

fn run_from_file(solution: &dyn AdventPuzzleSolution, mut file: impl Read) -> String {
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    solution.run(&s)
}

fn run_from_path(solution: &dyn AdventPuzzleSolution, path: impl AsRef<Path>) -> String {
    let file = File::open(path).unwrap();
    run_from_file(solution, file)
}

fn run_from_stdin(solution: &dyn AdventPuzzleSolution) -> String {
    let stdin = std::io::stdin();
    run_from_file(solution, stdin)
}

impl<F, R> AdventPuzzleSolution for F
where
    F: Fn(&str) -> R,
    R: Display,
{
    fn run(&self, input: &str) -> String {
        self(input).to_string()
    }
}

impl<P1, P2> AdventPuzzleSolution for (P1, P2)
where
    P1: AdventPuzzleSolution,
    P2: AdventPuzzleSolution,
{
    fn run(&self, input: &str) -> String {
        let (p1, p2) = self;
        format!("{}\n{}", p1.run(input), p2.run(input))
    }
}

struct CombinedSolution<F, R1, R2>
where
    F: Fn(&str) -> (R1, R2),
    R1: Display,
    R2: Display,
{
    pub func: F,
}

impl<F, R1, R2> AdventPuzzleSolution for CombinedSolution<F, R1, R2>
where
    F: Fn(&str) -> (R1, R2),
    R1: Display,
    R2: Display,
{
    fn run(&self, input: &str) -> String {
        let f = &self.func;
        let (r1, r2) = f(input);
        format!("{}\n{}", r1, r2)
    }
}

fn run_puzzle(solution: &dyn AdventPuzzleSolution, input: &str) {
    let t0 = Instant::now();
    let answer = if input == "-" {
        run_from_stdin(solution)
    } else {
        run_from_path(solution, input)
    };
    let t1 = Instant::now();
    let dt = t1 - t0;
    println!("{}", &answer);
    if dt.as_millis() >= 10 {
        println!("⏱️  {} ms", dt.as_millis());
    } else {
        println!("⏱️  {:.2} ms", dt.as_micros() as f64 * 1e-3);
    }
    println!();
}

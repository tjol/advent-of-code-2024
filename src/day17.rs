use itertools::Itertools;

pub fn day17part1(input: &str) -> String {
    let problem = Problem::new_from_input(input).unwrap();
    let computer = Computer::new(&problem);
    let output = computer.run().unwrap();
    output.into_iter().map(|i| i.to_string()).join(",")
}

pub fn day17part2(input: &str) -> i64 {
    let problem = Problem::new_from_input(input).unwrap();

    let mut a = 0;

    // The trick is that the final parts of the output are only ever influenced
    // by the most significant bits in A (A only ever gets smaller; in my input
    // only ever by 3 bits at a time)
    for tail_len in 1..=problem.program.len() {
        a <<= 3;

        let required_output = &problem.program[problem.program.len() - tail_len..];

        let mut delta_a = 0;
        loop {
            let mut computer = Computer::new(&problem);
            computer.a = a + delta_a;
            let output = computer.run().unwrap();
            if output == required_output {
                a += delta_a;
                break;
            } else if output.len() > required_output.len() {
                panic!("program does not behave as expected");
            }

            delta_a += 1;
        }
    }

    a
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug)]
struct InvalidOpcode(u8);

impl TryFrom<u8> for Opcode {
    type Error = InvalidOpcode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Adv),
            1 => Ok(Self::Bxl),
            2 => Ok(Self::Bst),
            3 => Ok(Self::Jnz),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out),
            6 => Ok(Self::Bdv),
            7 => Ok(Self::Cdv),
            _ => Err(InvalidOpcode(value)),
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    a: i64,
    b: i64,
    c: i64,
    program: Vec<u8>,
}

struct Computer<'a> {
    ip: usize,
    a: i64,
    b: i64,
    c: i64,
    program: &'a [u8],
}

#[derive(Debug)]
enum TerminationReason {
    EndOfProgram,
    InvalidOpcode,
    InvalidProgram,
}

impl From<InvalidOpcode> for TerminationReason {
    fn from(InvalidOpcode(_opcode): InvalidOpcode) -> Self {
        Self::InvalidOpcode
    }
}

#[derive(Debug)]
struct ParseError;

impl Problem {
    pub fn new_from_input(input: &str) -> Result<Self, ParseError> {
        let lines = input.lines().collect_vec();
        if lines.len() < 5 {
            return Err(ParseError);
        }

        let a = lines[0]
            .starts_with("Register A: ")
            .then(|| lines[0][12..].parse().ok())
            .flatten()
            .ok_or(ParseError)?;
        let b = lines[1]
            .starts_with("Register B: ")
            .then(|| lines[1][12..].parse().ok())
            .flatten()
            .ok_or(ParseError)?;
        let c = lines[2]
            .starts_with("Register C: ")
            .then(|| lines[2][12..].parse().ok())
            .flatten()
            .ok_or(ParseError)?;

        let program = lines[4]
            .starts_with("Program: ")
            .then(|| {
                lines[4][9..]
                    .split(',')
                    .map(|n| n.parse())
                    .collect::<Result<Vec<_>, _>>()
                    .ok()
            })
            .flatten()
            .ok_or(ParseError)?;

        Ok(Self { a, b, c, program })
    }
}

impl<'a> Computer<'a> {
    pub fn new(problem: &'a Problem) -> Self {
        Self {
            ip: 0,
            a: problem.a,
            b: problem.b,
            c: problem.c,
            program: &problem.program,
        }
    }

    fn decode_combo(&self, value: u8) -> Result<i64, TerminationReason> {
        match value {
            0..=3 => Ok(value as i64),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            _ => Err(TerminationReason::InvalidProgram),
        }
    }

    fn decode_literal(&self, value: u8) -> Result<i64, TerminationReason> {
        Ok(value as i64)
    }

    fn take_value(&mut self) -> Result<u8, TerminationReason> {
        if self.ip < self.program.len() {
            let val = self.program[self.ip];
            self.ip += 1;
            Ok(val)
        } else {
            Err(TerminationReason::EndOfProgram)
        }
    }

    pub fn step(&mut self, output: &mut Vec<u8>) -> Result<(), TerminationReason> {
        let opcode: Opcode = self.take_value()?.try_into()?;
        let operand = self.take_value()?;

        match opcode {
            Opcode::Adv => {
                self.a >>= self.decode_combo(operand)?;
            }
            Opcode::Bxl => self.b ^= self.decode_literal(operand)?,
            Opcode::Bst => {
                self.b = self.decode_combo(operand)? & 0b111;
            }
            Opcode::Jnz => {
                if self.a != 0 {
                    self.ip = self.decode_literal(operand)? as usize;
                }
            }
            Opcode::Bxc => {
                self.b ^= self.c;
            }
            Opcode::Out => {
                output.push((self.decode_combo(operand)? & 0b111) as u8);
            }
            Opcode::Bdv => {
                self.b = self.a >> self.decode_combo(operand)?;
            }
            Opcode::Cdv => {
                self.c = self.a >> self.decode_combo(operand)?;
            }
        }

        Ok(())
    }

    pub fn run(mut self) -> Result<Vec<u8>, TerminationReason> {
        let mut output = vec![];
        loop {
            match self.step(&mut output) {
                Ok(()) => (),
                Err(TerminationReason::EndOfProgram) => return Ok(output),
                Err(e) => return Err(e),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT_1: &str = "\
        Register A: 729\n\
        Register B: 0\n\
        Register C: 0\n\
        \n\
        Program: 0,1,5,4,3,0\n\
    ";

    static TEST_INPUT_2: &str = "\
        Register A: 2024\n\
        Register B: 0\n\
        Register C: 0\n\
        \n\
        Program: 0,3,5,4,3,0\n\
    ";

    #[test]
    fn part1test() {
        assert_eq!(&day17part1(TEST_INPUT_1), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2test() {
        assert_eq!(day17part2(TEST_INPUT_2), 117440);
    }
}

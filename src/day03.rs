use regex::Regex;

pub fn day03part1(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result = 0;
    
    for m in re.captures_iter(input) {
        let a: i64 = m.get(1).unwrap().as_str().parse().unwrap();
        let b: i64 = m.get(2).unwrap().as_str().parse().unwrap();
        result += a*b;
    }

    result
}

pub fn day03part2(input: &str) -> i64 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    let mut result = 0;
    let mut enabled = true;
    
    for m in re.captures_iter(input) {
        let txt = m.get(0).unwrap().as_str();
        if txt == "do()" {
            enabled = true;
        } else if txt == "don't()" {
            enabled = false;
        } else if enabled {
            let a: i64 = m.get(1).unwrap().as_str().parse().unwrap();
            let b: i64 = m.get(2).unwrap().as_str().parse().unwrap();
            result += a*b;
        }
    }

    result
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1test() {
        assert_eq!(day03part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 161);
    }

    #[test]
    fn part2test() {
        assert_eq!(day03part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"), 48);
    }

    
}
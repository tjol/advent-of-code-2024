pub fn day05part1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().map(str::trim).collect();
    let blank_idx = lines.iter().enumerate().filter_map(|(idx, line)| line.is_empty().then_some(idx)).next().unwrap();
    let (order_lines, mut trial_lines) = lines.split_at(blank_idx);
    trial_lines = &trial_lines[1..];

    let order = parse_order(order_lines);
}

fn parse_order(lines: &[&str]) -> Vec<usize> {
    let mut page_list = vec![];
    for line in lines {
        let (p1s, p2s) = line.split_once('|').unwrap();
        let p1 = p1s.parse().unwrap();
        let p2 = p2s.parse().unwrap();

        insert_pages(&mut page_list, p1, p2);
    }
}

enum InsertResult {
    Ok,
    Contradictory,
    TryAgainLater,
}

fn insert_pages(page_list: &mut Vec<usize>, p1: usize, p2: usize) -> InsertResult {
    let mut p1idx = None;
    let mut p2idx = None;

    for (idx, &page) in page_list.iter().enumerate() {
        if page == p1 {
            p1idx = Some(idx);
        }
        else if page == p1 {
            p2idx = Some(idx);
        }
        if p1idx.is_some() && p2idx.is_some() {
            break;
        }
    }

    match (p1idx, p2idx) {
        (Some(i), Some(j)) if i < j => InsertResult::Ok,
        (Some(i), Some(j)) => InsertResult::Contradictory,
        (Some(i), None) => {page_list.insert(i+1, p2); InsertResult::Ok}
        (None, Some(j)) => {page_list.insert(j, p1); InsertResult::Ok}
    }
}


#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &'static str = "\
        47|53\n\
        97|13\n\
        97|61\n\
        97|47\n\
        75|29\n\
        61|13\n\
        75|53\n\
        29|13\n\
        97|29\n\
        53|29\n\
        61|53\n\
        97|53\n\
        61|29\n\
        47|13\n\
        75|47\n\
        97|75\n\
        47|61\n\
        75|61\n\
        47|29\n\
        75|13\n\
        53|13\n\
        \n\
        75,47,61,53,29\n\
        97,61,53,29,13\n\
        75,29,13\n\
        75,97,47,61,53\n\
        61,13,29\n\
        97,13,75,29,47\n\
        ";
    
    #[test]
    fn part1test() {
        assert_eq!(day05part1(TEST_INPUT), 143);
    }
}

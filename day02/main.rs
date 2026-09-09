use fancy_regex::Regex;
use std::fs;
use std::sync::LazyLock;

fn is_id_invalid(id: u64) -> bool {
    let str = id.to_string();
    let (b, e) = str.split_at(str.len() / 2);
    b == e
}

static ID_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+)\1+$").unwrap());

fn is_id_invalid_part2(id: u64) -> bool {
    ID_REGEX.is_match(&id.to_string()).unwrap()
}

fn part1(input: &str) -> u64 {
    input
        .split(",")
        .map(|range| {
            let mut product = range.split("-");
            let first = product.next().unwrap().parse::<u64>().unwrap();
            let last = product.next().unwrap().parse::<u64>().unwrap();
            (first..=last)
                .map(|id| if is_id_invalid(id) { id } else { 0 })
                .sum::<u64>()
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .split(",")
        .map(|range| {
            let mut product = range.split("-");
            let first = product.next().unwrap().parse::<u64>().unwrap();
            let last = product.next().unwrap().parse::<u64>().unwrap();
            (first..=last)
                .map(|id| if is_id_invalid_part2(id) { id } else { 0 })
                .sum::<u64>()
        })
        .sum()
}

fn run_day(file: &str) {
    println!("run_day for {}", file);
    let f = fs::read_to_string(file);
    match f {
        Ok(s) => {
            println!("> part1 {}", part1(&s));
            println!("> part2 {}", part2(&s));
        }
        Err(_) => println!("> unable to load file: {file}"),
    }
}

#[cfg(test)]
mod part2 {
    use crate::is_id_invalid_part2;

    #[test]
    fn part2() {
        assert!(is_id_invalid_part2(11));

        for x in 12..=21 {
            assert!(!is_id_invalid_part2(x));
        }

        assert!(is_id_invalid_part2(22));
    }
}

fn main() {
    run_day("day02/test.txt");
    run_day("day02/input.txt");
}

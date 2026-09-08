use std::fs;

fn is_id_invalid(id: u64) -> u64 {
    let str = id.to_string();
    let (b, e) = str.split_at(str.len() / 2);
    if b == e {
        id
    } else {
        0
    }
}

fn part1(input: &str) -> u64 {
    input
        .split(",")
        .map(|range| {
            let mut product = range.split("-");
            let first = product.next().unwrap().parse::<u64>().unwrap();
            let last = product.next().unwrap().parse::<u64>().unwrap();
            (first..=last).map(|id| is_id_invalid(id)).sum::<u64>()
        })
        .sum()
}

fn part2(_input: &str) -> i32 {
    0
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

fn main() {
    run_day("day02/test.txt");
    run_day("day02/input.txt");
}

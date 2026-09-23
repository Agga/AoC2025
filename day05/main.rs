use std::time::Instant;

#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");

#[allow(unused_variables)]
pub fn do_part1(input: &str) -> usize {
    let (ranges, ingredient) = input.split_once("\r\n\r\n").unwrap();

    // grab rules
    let rules: Vec<(u64, u64)> = ranges
        .lines()
        .map(|line| {
            let (min, max) = line.split_once("-").unwrap();
            (min.parse().unwrap(), max.parse().unwrap())
        })
        .collect();

    // grab numbers
    let numbers: Vec<u64> = ingredient
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let fresh_items = numbers
        .iter()
        .filter(|&num| {
            for (a, b) in rules.iter() {
                if num >= a && num <= b {
                    return true;
                }
            }
            return false;
        })
        .count();

    fresh_items
}

pub fn intersect(a: (u64, u64), b: (u64, u64)) -> bool {
    u64::max(a.0, b.0) <= u64::min(a.1, b.1)
}

pub fn merge(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
    (u64::min(a.0, b.0), u64::max(a.1, b.1))
}

pub fn count_interval(v: (u64, u64)) -> u64 {
    v.1 - v.0 + 1
}

#[allow(unused_variables)]
pub fn do_part2(input: &str) -> u64 {
    let (ranges, ingredient) = input.split_once("\r\n\r\n").unwrap();

    // grab rules
    let mut rules: Vec<(u64, u64)> = ranges
        .lines()
        .map(|line| {
            let (min, max) = line.split_once("-").unwrap();
            (min.parse().unwrap(), max.parse().unwrap())
        })
        .collect();

    // sort ranges
    rules.sort();

    let mut total_numbers = 0;

    let mut prev: Option<(u64, u64)> = None;
    for curr in rules {
        match prev {
            None => prev = Some(curr),
            Some(value) => {
                if intersect(value, curr) {
                    prev = Some(merge(value, curr));
                } else {
                    total_numbers += count_interval(value);
                    prev = Some(curr);
                }
            }
        }
    }

    total_numbers += count_interval(prev.unwrap());

    total_numbers
}

fn main() {
    let mut now = Instant::now();
    println!("part1 {}", do_part1(INPUT_DATA));
    println!("{:?}", now.elapsed());

    now = Instant::now();
    println!("part2 {}", do_part2(INPUT_DATA));
    println!("{:?}", now.elapsed());
}

#[test]
fn part1() {
    assert_eq!(3, do_part1(TEST_DATA));
}

#[test]
fn part2() {
    assert!(intersect((0, 10), (5, 10)));
    assert!(intersect((5, 10), (0, 10)));

    assert!(intersect((0, 10), (10, 15)));
    assert!(intersect((10, 15), (0, 10)));

    assert!(!intersect((0, 10), (11, 15)));
    assert!(!intersect((11, 15), (0, 10)));

    assert!(intersect((0, 10), (1, 9)));
    assert!(intersect((1, 9), (0, 10)));

    assert_eq!(14, do_part2(TEST_DATA));
}

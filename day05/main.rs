use itertools::Itertools;
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

#[allow(unused_variables)]
pub fn do_part2(input: &str) -> usize {
    let (ranges, ingredient) = input.split_once("\r\n\r\n").unwrap();

    // grab rules
    let rules: Vec<(u64, u64)> = ranges
        .lines()
        .map(|line| {
            let (min, max) = line.split_once("-").unwrap();
            (min.parse().unwrap(), max.parse().unwrap())
        })
        .collect();

    let all_numbers = rules
        .iter()
        .flat_map(|(min, max)| (min..=max).iter().collect())
        .count();

    all_numbers
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
    assert_eq!(0, do_part2(TEST_DATA));
}

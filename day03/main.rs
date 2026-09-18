use std::{
    cmp::Ordering::{self, Equal, Greater, Less},
    time::Instant,
};

#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");

#[allow(unused_variables)]
pub fn do_part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let nums: Vec<u64> = line.chars().map(|c| c as u64 - 48).collect();

            let left = nums
                .iter()
                .take(nums.len() - 1)
                .enumerate()
                .max_by(|a, b| match a.1.cmp(b.1) {
                    Ordering::Less => Less,
                    _ => Greater,
                })
                .unwrap();

            let right = nums
                .iter()
                .enumerate()
                .skip(left.0 + 1)
                .max_by(|a, b| a.1.cmp(b.1))
                .unwrap();

            let joltage = left.1 * 10 + right.1;
            joltage
        })
        .sum()
}

#[allow(unused_variables)]
pub fn do_part2(input: &str) -> u64 {
    0
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
    assert_eq!(do_part1(TEST_DATA), 357);
}

#[test]
fn part2() {
    assert_eq!(do_part2(TEST_DATA), 0);
}

use std::time::Instant;

#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");

pub fn calc_joltage(line: &str, digits: usize) -> u64 {
    let mut joltage = 0;
    let mut start = 0;
    for i in (0..digits).rev() {
        let end = line.len() - i;

        // max_by_key finds the last occurance but we are interested in the first so we reverse the range
        let (idx, &num) = line.as_bytes()[start..end]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|k| k.1)
            .unwrap();

        // convert to int
        let num = (num - b'0') as u64;

        start = start + idx + 1;
        joltage = joltage * 10 + num;
    }

    joltage
}

#[allow(unused_variables)]
pub fn do_part1(input: &str) -> u64 {
    input.lines().map(|line| calc_joltage(line, 2)).sum()
}

#[allow(unused_variables)]
pub fn do_part2(input: &str) -> u64 {
    input.lines().map(|line| calc_joltage(line, 12)).sum()
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
    assert_eq!(do_part2(TEST_DATA), 3121910778619);
}

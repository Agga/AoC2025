use std::time::Instant;

#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");

#[allow(unused_variables)]
pub fn do_part1(input: &str) -> usize {
    let mut line = input.lines();

    // grab rules
    let mut rules: Vec<(u64, u64)> = vec![];
    while let Some(l) = line.next() {
        if l.is_empty() {
            break;
        }

        let mut iter = l.split("-");
        if let (Some(from), Some(to), None) = (iter.next(), iter.next(), iter.next()) {
            rules.push((from.parse().unwrap(), to.parse().unwrap()))
        }
    }

    // grab numbers
    let mut numbers: Vec<u64> = vec![];
    while let Some(l) = line.next() {
        if l.is_empty() {
            break;
        }

        numbers.push(l.parse().unwrap());
    }

    let fresh_items = numbers
        .iter()
        .filter(|&num| {
            for (a, b) in rules.iter() {
                if num >= a && num <= b {
                    return false;
                }
            }
            return true;
        })
        .count();

    fresh_items
}

#[allow(unused_variables)]
pub fn do_part2(input: &str) -> usize {
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
    assert_eq!(3, do_part1(TEST_DATA));
}

#[test]
fn part2() {
    assert_eq!(0, do_part2(TEST_DATA));
}

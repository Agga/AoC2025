use std::time::Instant;

#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");

#[derive(Copy, Clone)]
pub enum Op {
    Mul,
    Add,
}

#[allow(unused_variables)]
pub fn do_part1(input: &str) -> usize {
    let mut lines = input.lines().rev();

    let operations = lines.next().unwrap();

    let mut ops: Vec<Op> = vec![];
    let mut val: Vec<u64> = vec![];

    // populate with ops + initial values
    operations.split_whitespace().for_each(|c| match c {
        "+" => {
            ops.push(Op::Add);
            val.push(0);
        }
        "*" => {
            ops.push(Op::Mul);
            val.push(1);
        }
        _ => panic!(),
    });

    for line in lines {
        line.split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .enumerate()
            .for_each(|(i, num)| match ops[i] {
                Op::Mul => {
                    val[i] *= num;
                }
                Op::Add => {
                    val[i] += num;
                }
            });
    }

    val.iter().sum::<u64>() as usize
}

#[allow(unused_variables)]
pub fn do_part2(input: &str) -> usize {
    // transpose the entire document
    let mut columns: Vec<String> = vec![];
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            match columns.get_mut(i) {
                Some(string) => string.push(c),
                None => columns.push(c.to_string()),
            }
        }
    }

    let mut curr_op = Op::Add;
    let mut curr_val = 0;
    let mut curr_sum = 0;

    for str in columns.iter() {
        let line = str.trim();
        if line.is_empty() {
            continue;
        }

        // change current operation and trim the string
        let mut change_op = |op: Op| -> &str {
            curr_op = op;
            curr_sum += curr_val;
            curr_val = match op {
                Op::Add => 0,
                Op::Mul => 1,
            };
            line[0..line.len() - 1].trim()
        };

        let value = match line.chars().last().unwrap() {
            '+' => change_op(Op::Add),
            '*' => change_op(Op::Mul),
            _ => line,
        }
            .trim()
            .parse::<u64>()
            .unwrap();

        match curr_op {
            Op::Add => {
                curr_val += value;
            }
            Op::Mul => {
                curr_val *= value;
            }
        }
    }
    curr_sum += curr_val;
    curr_sum as usize
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
    assert_eq!(4277556, do_part1(TEST_DATA));
}

#[test]
fn part2() {
    assert_eq!(3263827, do_part2(TEST_DATA));
}

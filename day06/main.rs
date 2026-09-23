use std::time::Instant;

#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");

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

    let mut a = lines.next().unwrap().split_whitespace();
    let mut b = lines.next().unwrap().split_whitespace();
    let mut c = lines.next().unwrap().split_whitespace();
    let mut d = lines.next().unwrap().split_whitespace();
    let all = a.zip(b).zip(c).zip(d);

    for v in all {
        v.
    }

    for op in ops.iter().enumerate() {
        let a = a.next().unwrap().parse::<u64>().unwrap();
        let b = b.next().unwrap().parse::<u64>().unwrap();
        let c = c.next().unwrap().parse::<u64>().unwrap();
        let d = d.next().unwrap().parse::<u64>().unwrap();
        println!("{a} + {b} + {c} + {d} ");
    }

    // for line in lines {
    //     line.split_whitespace()
    //         .map(|s| s.parse::<u64>().unwrap())
    //         .enumerate()
    //         .for_each(|(i, num)| match ops[i] {
    //             Op::Mul => {
    //                 val[i] *= num;
    //             }
    //             Op::Add => {
    //                 val[i] += num;
    //             }
    //         });
    // }
    //
    // val.iter().sum::<u64>() as usize
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
    assert_eq!(4277556, do_part1(TEST_DATA));
}

#[test]
fn part2() {
    assert_eq!(3263827, do_part2(TEST_DATA));
}

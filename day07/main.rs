use std::fmt::Debug;
use std::time::Instant;

use aoc::Grid;

#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");

pub enum Block {
    Empty,
    Splitter,
    Start,
}

impl Debug for Block{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Block::Empty => write!(f, "."),
            Block::Splitter => write!(f, "^"),
            Block::Start => write!(f, "S"),
        }
    }
}

#[allow(unused_variables)]
pub fn do_part1(input: &str) -> usize {
    let grid = Grid::from_file(input, |c| match c {
        'S' => Block::Start,
        '^' => Block::Splitter,
        _ => Block::Empty,
    });


    // grid.rows();
    // grid.columns();



    println!("{grid:?}");

    0
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
    assert_eq!(0, do_part1(TEST_DATA));
}

#[test]
fn part2() {
    assert_eq!(0, do_part2(TEST_DATA));
}

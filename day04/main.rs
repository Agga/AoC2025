// use aoc::{Grid, Vec2};
use aoc::{Grid, Vec2};
use std::time::Instant;

#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Block {
    Empty,
    Paper,
}

pub const ADJACENT: [Vec2; 8] = [
    Vec2::new(-1, -1),
    Vec2::new(0, -1),
    Vec2::new(1, -1),
    Vec2::new(-1, 0),
    Vec2::new(1, 0),
    Vec2::new(-1, 1),
    Vec2::new(0, 1),
    Vec2::new(1, 1),
];

pub fn can_remove_paper(grid: &Grid<Block>, pos: &Vec2) -> bool {
    let num_papers: usize = ADJACENT
        .iter()
        .map(|&adj| match grid.value_for(&(*pos + adj)) {
            Some(Block::Paper) => 1,
            _ => 0,
        })
        .sum();

    num_papers < 4
}

#[allow(unused_variables)]
pub fn do_part1(input: &str) -> usize {
    0
}

#[allow(unused_variables)]
pub fn do_part2(input: &str) -> usize {
    let mut grid = Grid::<Block>::from_file(input, |c| match c {
        '@' => Block::Paper,
        _ => Block::Empty,
    });

    // create vec of all papers
    let mut all_paper: Vec<(Vec2, Block)> = grid
        .iter()
        .filter(|&f| f.1 == Block::Paper)
        .cloned()
        .collect();

    let mut total_papers = 0;

    loop {
        let before = all_paper.len();
        all_paper.retain(|(pos, _)| {
            if can_remove_paper(&grid, pos) {
                grid.set_value_for(pos, Block::Empty);
                false
            } else {
                true
            }
        });
        if all_paper.len() == before {
            break;
        } else {
            total_papers += before - all_paper.len();
        }
    }

    total_papers
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
    assert_eq!(13, do_part1(TEST_DATA));
}

#[test]
fn part2() {
    assert_eq!(43, do_part2(TEST_DATA));
}

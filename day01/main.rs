use std::env;
use std::fs;
use std::io;

fn part1(input: &str) -> i32 {
    let directions = input
        .lines()
        .map(|line| {
            let mut c = line.chars();
            let d = c
                .next()
                .map(|d| match d {
                    'L' => -1,
                    'R' => 1,
                    _ => panic!(),
                })
                .unwrap();

            let n: i32 = line[1..].parse().unwrap();
            d * n
        })
        .collect::<Vec<i32>>();

    let mut c = 0;
    let mut result = 50;
    for d in directions {
        result += d;

        // below 0
        while result < 0 {
            result += 100;
        }

        // above 99
        result %= 100;
        if result == 0 {
            c += 1;
        }
    }

    c
}

fn part2(input: &str) -> i32 {
    let mut current: i32 = 50;
    let mut visited_zeroes = 0;

    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let mut value: i32 = line[1..].parse().unwrap();
        if direction == 'L' {
            value *= -1;
        }

        let previous = current;
        current += value;

        let mut crossings = (current.div_euclid(100) - previous.div_euclid(100)).abs();
        if direction == 'L' {
            if previous == 0 {
                crossings -= 1
            }
            if current.rem_euclid(100) == 0 {
                crossings += 1;
            }
        }
        visited_zeroes += crossings;

        current = current.rem_euclid(100);
    }
    visited_zeroes
}

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("Current working directory: {}", current_dir.display());

    let input = "day01/input.txt";
    let contents = fs::read_to_string(input)?;

    println!("part1 {}", part1(&contents));
    println!("part2 {}", part2(&contents));

    Ok(())
}

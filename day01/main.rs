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

    let mut clicks = 0;
    let mut result = 50;
    for d in directions {
        let b = result;
        let c_before = clicks;
        result += d;

        // below 0
        while result < 0 {
            result += 100;
            clicks += 1;
        }

        // above 99
        while result > 99 {
            result -= 100;
            clicks += 1;
        }

        println!(
            "before {} + {} = {}. clicks: {}",
            b,
            d,
            result,
            clicks - c_before
        );

        if (clicks - c_before) > 1 {
            clicks = clicks;
        }
    }

    clicks
}

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("Current working directory: {}", current_dir.display());

    let input = "day01/test_input.txt";
    let contents = fs::read_to_string(input)?;

    // println!("test part1 {}", part1(&contents));
    // println!("test part2 {}", part2(&contents));

    let input = "day01/input.txt";
    let contents = fs::read_to_string(input)?;

    //println!("part1 {}", part1(&contents));
    println!("part2 {}", part2(&contents));
    // 6649 too high

    Ok(())
}

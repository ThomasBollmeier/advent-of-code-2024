use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let stats = blink_n_times(&read_stone_stats(reader), 25);
        let answer = stats.values().sum::<i64>();
        Ok(answer as usize)
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let stats = blink_n_times(&read_stone_stats(reader), 75);
        let answer = stats.values().sum::<i64>();
        Ok(answer as usize)
    }

    //assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

type StoneStats = HashMap<i64, i64>;

fn blink_n_times(stone_stats: &StoneStats, n: usize) -> StoneStats {
    let mut stats = stone_stats.clone();
    for _ in 0..n {
        stats = blink(&stats);
    }
    stats
}

fn blink(stone_stats: &StoneStats) -> StoneStats {
    let mut ret = HashMap::new();

    for (stone, cnt) in stone_stats {
        let (left, right) = transform(*stone);
        *ret.entry(left).or_default() += cnt;

        if let Some(right) = right {
            *ret.entry(right).or_default() += cnt;
        }
    }

    ret
}

fn transform(stone: i64) -> (i64, Option<i64>) {
    if stone == 0 {
        return (1, None);
    }

    let stone_str = stone.to_string();

    if stone_str.len() % 2 == 0 {
        let n = stone_str.len() / 2;
        let left = stone_str.chars().take(n).collect::<String>();
        let left = left.parse::<i64>().unwrap();

        let right = stone_str.chars().skip(n).collect::<String>();
        let mut right = right.trim_start_matches('0');
        if right.len() == 0 {
            right = "0";
        }
        let right = right.parse::<i64>().unwrap();

        return (left, Some(right));
    }

    (stone * 2024, None)
}

fn read_stone_stats(reader: impl BufRead) -> StoneStats {
    let lines = read_lines(reader);
    let line = lines[0].trim();
    let numbers = line
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut stats = HashMap::new();
    for number in numbers {
        *stats.entry(number).or_default() += 1;
    }

    stats
}

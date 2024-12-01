use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (mut left, mut right) = parse_input(reader)?;
        left.sort();
        right.sort();
        let pairs = left.iter().zip(right.iter()).collect::<Vec<(&i32, &i32)>>();
        let total = pairs.iter().map(|(a, b)| (*a - *b).abs()).sum::<i32>();
        Ok(total as usize)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (left, right) = parse_input(reader)?;
        let right_freq = calc_frequencies(&right);
        let mut result = 0;

        for left_num in left {
            let factor = right_freq.get(&left_num).unwrap_or(&0);
            result += left_num as usize * factor;
        }

        Ok(result)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn parse_input(input_file: impl BufRead) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input_file.lines() {
        let line = line?;
        let nums = line.split(" ")
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let nums: Vec<i32> = nums.iter()
            .map(|s| { s.parse::<i32>().unwrap() })
            .collect();
        left.push(nums[0]);
        right.push(nums[1]);
    }

    Ok((left, right))
}

fn calc_frequencies(numbers: &[i32]) -> HashMap<i32, usize> {
    let mut frequencies = HashMap::new();
    for num in numbers.iter() {
        let new_count = {
            let count = frequencies.entry(*num).or_insert(0);
            *count + 1
        };
        frequencies.insert(*num, new_count);
    }

    frequencies
}

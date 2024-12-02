use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let reports = read_input(reader)?;
        let num_safe = reports.iter()
            .filter(|r| is_safe(r) )
            .count();

        Ok(num_safe)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let reports = read_input(reader)?;
        let num_safe = reports.iter()
            .filter(|r| is_safe_with_dampener(r) )
            .count();

        Ok(num_safe)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn read_input(reader: impl BufRead) -> Result<Vec<Vec<i32>>> {

    let mut ret = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let nums = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        ret.push(nums);
    }

    Ok(ret)
}

fn is_safe(report: &Vec<i32>) -> bool {
    let increasing = report[0] < report[1];
    let mut delta = (report[1] - report[0]).abs();

    if delta < 1 || delta > 3 {
        return false;
    }

    for i in 1..report.len() - 1 {
        delta = (report[i+1] - report[i]).abs();
        if delta < 1 || delta > 3 {
            return false;
        }
        if increasing {
            if report[i] >= report[i+1] {
                return false;
            }
        } else {
            if report[i+1] >= report[i] {
                return false;
            }
        }
    }

    true
}

fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len()  {
        let mut r = report.clone();
        r.remove(i);
        if is_safe(&r) {
            return true;
        }
    }

    false
}

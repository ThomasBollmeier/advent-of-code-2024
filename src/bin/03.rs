use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
        let lines = read_lines(reader);
        let mut total = 0;

        for line in lines {
            for cap in re.captures_iter(&line) {
                let first = cap.get(1).unwrap().as_str().parse::<i32>()?;
                let second = cap.get(2).unwrap().as_str().parse::<i32>()?;
                total += first * second;
            }
        }

        Ok(total as usize)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))")?;
        let lines = read_lines(reader);
        let mut total = 0;
        let mut enabled = true;

        for line in lines {
            for cap in re.captures_iter(&line) {
                let command = cap.get(1).unwrap().as_str();
                if command.starts_with("mul") {
                    if enabled {
                        let first = cap.get(2).unwrap().as_str().parse::<i32>()?;
                        let second = cap.get(3).unwrap().as_str().parse::<i32>()?;
                        total += first * second;
                    }
                } else {
                    enabled = !command.starts_with("don");
                }
            }
        }
        
        Ok(total as usize)
    }
    
    let test2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(48, part2(BufReader::new(test2.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn read_lines(reader: impl BufRead) -> Vec<String> {
    let mut ret = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        ret.push(line);
    }

    ret
}
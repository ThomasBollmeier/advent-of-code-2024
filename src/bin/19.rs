use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use trie_rs::{Trie, TrieBuilder};

const DAY: &str = "19";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let problem = read_problem(reader);
        let mut cache = HashMap::new();

        let count = problem
            .designs
            .iter()
            .filter(|design| problem.can_create_design(design, &mut cache))
            .count();

        Ok(count)
    }

    assert_eq!(6, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let problem = read_problem(reader);
        let mut cache = HashMap::new();

        let count = problem
            .designs
            .iter()
            .map(|design| problem.count_possible_designs(design, &mut cache))
            .sum();

        Ok(count)
    }

    assert_eq!(16, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

struct Problem {
    patterns_trie: Trie<u8>,
    designs: Vec<String>,
}

impl Problem {
    fn new(patterns: Vec<String>, designs: Vec<String>) -> Self {
        let mut builder = TrieBuilder::new();

        for pattern in &patterns {
            builder.push(pattern);
        }

        Self {
            patterns_trie: builder.build(),
            designs,
        }
    }

    fn count_possible_designs(&self, design: &str, cache: &mut HashMap<String, usize>) -> usize {
        if cache.contains_key(design) {
            return cache[design];
        }

        if design.is_empty() {
            return 1;
        }

        let patterns = self
            .patterns_trie
            .common_prefix_search(design)
            .collect::<Vec<String>>();

        let mut count = 0;

        for pattern in patterns {
            count += self.count_possible_designs(&design[pattern.len()..], cache);
        }

        cache.insert(design.to_string(), count);
        count
    }

    fn can_create_design(&self, design: &str, cache: &mut HashMap<String, bool>) -> bool {
        if cache.contains_key(design) {
            return cache[design];
        }

        if design.is_empty() {
            return true;
        }

        let patterns = self
            .patterns_trie
            .common_prefix_search(design)
            .collect::<Vec<String>>();

        for pattern in patterns {
            if self.can_create_design(&design[pattern.len()..], cache) {
                cache.insert(design.to_string(), true);
                return true;
            }
        }

        cache.insert(design.to_string(), false);
        false
    }
}

fn read_problem(reader: impl BufRead) -> Problem {
    let mut designs = Vec::new();

    let lines = read_lines(reader);
    let patterns = lines[0]
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    for line in &lines[2..] {
        designs.push(line.to_string());
    }

    Problem::new(patterns, designs)
}

use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (rules, page_sequences) = read_rules_and_pages(reader);
        let mut total = 0;
        for page_seq in page_sequences {
            if is_valid(&page_seq, &rules) {
                total += page_seq[page_seq.len() / 2];
            }
        }

        Ok(total as usize)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (rules, page_sequences) = read_rules_and_pages(reader);
        let mut total = 0;
        for page_seq in page_sequences {
            if !is_valid(&page_seq, &rules) {
                let sorted_pages = sort_pages(&page_seq, &rules.predecessors);
                total += sorted_pages[page_seq.len() / 2];
            }
        }

        Ok(total as usize) 
    }
    
    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug)]
struct PageRules {
    predecessors: HashMap<i32, HashSet<i32>>,
    successors: HashMap<i32, HashSet<i32>>,
}

fn sort_pages(page_sequence: &[i32], predecessors: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut ret = Vec::new();
    let mut pages: HashSet<i32> = HashSet::from_iter(page_sequence.iter().cloned());
    let mut relations = prune_relations(predecessors, page_sequence, false);
    
    while !relations.is_empty() {
        let mut page_wo_pred: Option<i32> = None;
        for page in &pages {
            if !relations.contains_key(page) { 
                page_wo_pred = Some(*page);
                break;
            }
        }
        if let Some(page) = page_wo_pred {
            ret.push(page);
            pages.remove(&page);
            relations = prune_relations(&relations, &[page], true);
        }
    }
    
    if !pages.is_empty() {
        ret.extend(pages);
    }
    
    ret
}

fn prune_relations(relations: &HashMap<i32, HashSet<i32>>, page_sequence: &[i32], exclude: bool) -> HashMap<i32, HashSet<i32>> {
    let pages = HashSet::<i32>::from_iter(page_sequence.iter().cloned());
    let mut ret = HashMap::new();

    for (key, values) in relations {
        if !exclude && !pages.contains(key) || exclude && pages.contains(key) {
            continue;
        }
        let mut pruned_values = HashSet::new();
        for value in values {
            if !exclude && pages.contains(value) || exclude && !pages.contains(value) {
                pruned_values.insert(*value);  
            }
        }
        if !pruned_values.is_empty() {
            ret.insert(*key, pruned_values);    
        }
    }
    
    ret
}


fn is_valid(page_sequence: &Vec<i32>, rules: &PageRules) -> bool {
    let mut predecessors: HashSet<i32> = HashSet::new();
    let mut successors: HashSet<i32> = HashSet::new();
    for page in &page_sequence[1..] {
        successors.insert(*page);
    }

    for page in page_sequence {
        if let Some(expected_successors) = rules.successors.get(page) {
            for pred in predecessors.iter() {
                if expected_successors.contains(pred) {
                    return false;
                }
            }
        }
        if let Some(expected_predecessors) = rules.predecessors.get(page) {
            for succ in successors.iter() {
                if expected_predecessors.contains(succ) {
                    return false;
                }
            }
        }
        predecessors.insert(*page);
        successors.remove(page);
    }

    true
}

fn read_rules_and_pages(reader: impl BufRead) -> (PageRules, Vec<Vec<i32>>) {
    let mut predecessors = HashMap::new();
    let mut successors = HashMap::new();
    let mut page_sequences = Vec::new();

    let lines = read_lines(reader);
    let mut parse_rules = true;
    for line in lines {
        if line.is_empty() {
            parse_rules = false;
            continue;
        }
        if parse_rules {
            let pages: Vec<i32> = line.split("|")
                .map(|s| { s.parse::<i32>().unwrap() })
                .collect();
            let predecessor = pages[0];
            let successor = pages[1];
            let mut entry: &mut HashSet<i32> = successors.entry(predecessor).or_default();
            entry.insert(successor);
            entry = predecessors.entry(successor).or_default();
            entry.insert(predecessor);
        } else {
            let page_sequence: Vec<i32> = line.split(",")
                .map(|s| { s.parse::<i32>().unwrap() })
                .collect();
            page_sequences.push(page_sequence);
        }
    }

    (PageRules{predecessors, successors}, page_sequences)
}
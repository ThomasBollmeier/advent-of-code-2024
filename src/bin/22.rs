use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "22";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
1
10
100
2024
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let secrets = read_lines(reader)
            .iter()
            .map(|s| s.parse::<Secret>().unwrap())
            .collect::<Vec<_>>();
        
        let answer = secrets
            .iter()
            .map(|secret| calc_nth_secret(*secret, 2000))
            .sum::<Secret>();
      
        Ok(answer as usize)
    }

    assert_eq!(37327623, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let secrets = read_lines(reader)
            .iter()
            .map(|s| s.parse::<Secret>().unwrap())
            .collect::<Vec<_>>();
        
        let answer = find_best_total_price(&secrets, 2000);
        Ok(answer as usize)
    }
    
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

type Secret = u64;
type Sequence = [i32; 4];

fn find_best_total_price(secrets: &[Secret], n: usize) -> Secret {
    let sequences = vec![-9..=9;4]
        .into_iter()
        .multi_cartesian_product()
        .collect_vec();
    
    let price_maps = all_prices_per_change_sequence(secrets, n);
    
    let mut prices = vec![];
    for sequence in sequences {
        let change_sequence = [
            sequence[0],
            sequence[1],
            sequence[2],
            sequence[3]];
        prices.push(calc_total_price(&price_maps, &change_sequence));
    }
    
    *prices.iter().max().unwrap()
}

fn calc_total_price(price_maps: &[HashMap<Sequence, Secret>], change_sequence: &Sequence) -> Secret {
    price_maps
        .iter()
        .flat_map(|price_map| find_price(price_map, change_sequence))
        .sum()
}

fn find_price(price_map: &HashMap<Sequence, Secret>, change_sequence: &Sequence) -> Option<Secret> {
    price_map.get(change_sequence).copied()
}

fn all_prices_per_change_sequence(secrets: &[Secret], n: usize) -> Vec<HashMap<Sequence, Secret>> {
    calc_all_values_and_changes(secrets, n)
        .iter()
        .map(|(secrets, changes)| prices_per_change_sequence(secrets, changes))
        .collect()
}

fn calc_all_values_and_changes(secrets: &[Secret], n: usize) -> Vec<(Vec<Secret>, Vec<i32>)> {
    secrets
        .iter()
        .map(|secret| calc_values_and_changes(*secret, n))
        .collect::<Vec<_>>()
}

fn prices_per_change_sequence(secrets: &[Secret], changes: &[i32]) -> HashMap<Sequence, Secret> {
    let mut ret = HashMap::new();
    for (i, sequence) in changes.windows(4).enumerate() {
        let price = secrets[i + 3];
        let key = [sequence[0], sequence[1], sequence[2], sequence[3]];
        ret.entry(key).or_insert(price);
    }

    ret
}

fn calc_values_and_changes(secret: Secret, n: usize) -> (Vec<Secret>, Vec<i32>) {
    let mut secrets = vec![secret % 10];
    let mut cur = secret;
    for _ in 0..n {
        cur = next_secret(cur);
        secrets.push(cur % 10);
    }

    let changes = secrets
        .windows(2)
        .map(|w| w[1] as i32 - w[0] as i32)   
        .collect::<Vec<_>>();
    
    (secrets[1..].to_vec(), changes)
}

fn calc_nth_secret(secret: Secret, n: usize) -> Secret {
    let mut ret = secret;
    for _ in 0..n {
        ret = next_secret(ret);
    }
    
    ret
}

fn next_secret(secret: Secret) -> Secret {
    const PRUNE_NUMBER: Secret = 16777216;
    
    let mut ret = secret * 64;
    ret ^= secret;
    ret %= PRUNE_NUMBER;
    
    ret ^= ret / 32;
    ret %= PRUNE_NUMBER;
    
    ret ^= ret * 2048;
    ret %= PRUNE_NUMBER;
    
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_next_secret() {
        let secret = 123;
        assert_eq!(next_secret(secret), 15887950);
    }
    
    #[test]
    fn test_calc_nth_secret() {
        let secret = 123;
        assert_eq!(calc_nth_secret(secret, 10), 5908254);
    }
    
    #[test]
    fn test_calc_nth_changes() {   
        let secret = 123;
        let changes = calc_values_and_changes(secret, 6).1.to_vec();
        assert_eq!(changes, [-3, 6, -1, -1, 0, 2]);
    }
    
    #[test]
    fn test_find_best_total_price() {
        let secrets = [1, 2, 3, 2024];
        assert_eq!(find_best_total_price(&secrets, 2000), 23);
    }
    
}

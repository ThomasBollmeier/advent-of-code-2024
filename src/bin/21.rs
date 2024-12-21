use std::result::Result::Ok;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;

use adv_code_2024::*;
use adv_code_2024::grid::{Direction, Position};

const DAY: &str = "21";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
029A
980A
179A
456A
379A
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;
        
        for code in read_lines(reader) {
            let (factor1, factor2) = complexity_factors(&code, 2);
            result += factor1 * factor2;
        }
        
        Ok(result)
    }

    assert_eq!(126384, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    //println!("\n=== Part 2 ===");
    //
    //fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //    Ok(0)
    //}
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    //let input_file = BufReader::new(File::open(INPUT_FILE)?);
    //let result = time_snippet!(part2(input_file)?);
    //println!("Result = {}", result);
    //endregion

    Ok(())
}

type KeyPad = HashMap<char, Position>;

fn complexity_factors(code: &str, num_directional: usize) -> (usize, usize) {
    let mut digits = String::new();
    for (pos, chr) in code.chars().enumerate() {
        if pos == 0 && chr == '0' {
            continue;
        }
        if chr.is_ascii_digit() {
            digits.push(chr);
        }
    }
    
    let factor1 = digits.parse::<usize>().unwrap();
    let factor2 = determine_human_moves(code, num_directional).unwrap().len();

    (factor1, factor2)
}

fn determine_human_moves(code: &str, num_directional: usize) -> Result<String> {
    let mut keypads = vec![init_directional_keypad();num_directional];
    keypads.insert(0, init_numerical_keypad());
    
    let mut possible_moves = determine_moves(&keypads[0], code)?;
    
    for keypad in &keypads[1..] {
        let mut next_possible_moves = vec![];
        possible_moves = get_best_moves(keypad, &possible_moves);
        for possible_mov in &possible_moves {
            let moves = determine_moves(keypad, possible_mov)?;
            next_possible_moves.extend(moves);
        }
        possible_moves = next_possible_moves;
    }
    
    Ok(possible_moves.iter().min_by_key(|mov| mov.len()).unwrap().to_string())
}

fn get_best_moves(keypad: &KeyPad, moves: &[String]) -> Vec<String> {
    let min_distance = moves
        .iter()
        .map(|mov| manhattan_distance(keypad, mov))
        .min()
        .unwrap();
    
    moves
        .iter()
        .filter(|&mov| manhattan_distance(keypad, mov) == min_distance)
        .cloned()
        .collect::<Vec<_>>()
}

fn determine_moves(keypad: &KeyPad, code: &str) -> Result<Vec<String>> {
    let mut moves = vec!["".to_string()];
    let mut from = 'A';

    for to in code.chars() {
        let moves_per_step = determine_moves_per_step(keypad, from, to)?;
        let mut next_moves = vec![];
        for mov in &moves {
            for mov_step in &moves_per_step {
                next_moves.push(mov.to_string() + mov_step + "A");
            }
        }
        moves = next_moves;
        from = to;
    }

    Ok(moves)
}

fn manhattan_distance(keypad: &KeyPad, code: &str) -> usize {
    let mut s = String::from("A");
    s.push_str(code);
    
    let chars = s.chars().collect::<Vec<char>>();
    chars
        .windows(2)
        .flat_map(|pair| {
            manhattan_distance_per_step(keypad, pair[0], pair[1])
        })
    .sum::<usize>()
}

fn manhattan_distance_per_step(keypad: &KeyPad, from: char, to: char) -> Result<usize> {

    let start = keypad.get(&from).ok_or_else(|| anyhow!("invalid from"))?;
    let start_row = start.row();
    let start_col = start.col();

    let end = keypad.get(&to).ok_or_else(|| anyhow!("invalid to"))?;
    let end_row = end.row();
    let end_col = end.col();
    
    Ok(
        (end_row - start_row).unsigned_abs() as usize + 
        (end_col - start_col).unsigned_abs() as usize 
    )
}

fn determine_moves_per_step(keypad: &KeyPad, from: char, to: char) -> Result<Vec<String>> {
    let mut valid_positions = HashSet::new();
    for pos in keypad.values() {
        valid_positions.insert(pos.clone());
    }
    
    let start = keypad.get(&from).ok_or_else(|| anyhow!("invalid from"))?;
    let start_row = start.row();
    let start_col = start.col();
    
    let end = keypad.get(&to).ok_or_else(|| anyhow!("invalid to"))?;
    let end_row = end.row();
    let end_col = end.col();
    
    let (vert_dir, n_vert) = get_dir_steps(
        start_row,
        end_row,
        Direction::South,
        Direction::North);

    let (horiz_dir, n_horiz) = get_dir_steps(
        start_col,
        end_col,
        Direction::East,
        Direction::West);

    let possible_dir_seqs = get_dir_sequences(
        &horiz_dir, &vert_dir, n_horiz, n_vert, start, &valid_positions);

    let mut ret = vec![];

    for dir_seq in possible_dir_seqs {
        let dir_seq_str = dir_seq
            .iter()
            .map(|dir    | {
                match dir {
                    Direction::North => '^',
                    Direction::South => 'v',
                    Direction::East => '>',
                    Direction::West => '<',
                }})
            .collect::<String>();
        ret.push(dir_seq_str);
    }

    Ok(ret)
}

fn get_dir_sequences(horiz_dir: &Direction,
                     vert_dir: &Direction,
                     n_horiz: i32,
                     n_vert: i32,
                     pos: &Position,
                     valid_positions: &HashSet<Position>) -> Vec<Vec<Direction>> {

    if n_horiz == 0 && n_vert == 0 {
        return vec![vec![]];
    }
    let mut ret = vec![];

    if n_horiz > 0 {
        let next = pos.make_step(horiz_dir);
        if valid_positions.contains(&next) {
            let dir_seqs = get_dir_sequences(
                horiz_dir, vert_dir, n_horiz - 1, n_vert,
                &next, valid_positions);
            for dir_seq in dir_seqs {
                let mut new_seq = vec![horiz_dir.clone()];
                new_seq.extend(dir_seq);
                ret.push(new_seq);
            }
        }
    }

    if n_vert > 0 {
        let next = pos.make_step(vert_dir);
        if valid_positions.contains(&next) {
            let dir_seqs = get_dir_sequences(
                horiz_dir, vert_dir, n_horiz, n_vert - 1,
                &next, valid_positions);
            for dir_seq in dir_seqs {
                let mut new_seq = vec![vert_dir.clone()];
                new_seq.extend(dir_seq);
                ret.push(new_seq);
            }
        }
    }

    ret
}

fn get_dir_steps(start: i32, end: i32, dir1: Direction, dir2: Direction) -> (Direction, i32) {
    match start.cmp(&end) {
        Ordering::Greater => (dir2, start - end),
        Ordering::Less => (dir1, end - start),
        Ordering::Equal => (dir2, 0),
    }
}

fn init_numerical_keypad() -> KeyPad {
    HashMap::from([
        ('7', Position::new(0, 0)),
        ('8', Position::new(0, 1)),
        ('9', Position::new(0, 2)),
        ('4', Position::new(1, 0)),
        ('5', Position::new(1, 1)),
        ('6', Position::new(1, 2)),
        ('1', Position::new(2, 0)),
        ('2', Position::new(2, 1)),
        ('3', Position::new(2, 2)),
        ('0', Position::new(3, 1)),
        ('A', Position::new(3, 2)),
    ])
}

fn init_directional_keypad() -> KeyPad {
    HashMap::from([
        ('^', Position::new(0, 1)),
        ('A', Position::new(0, 2)),
        ('<', Position::new(1, 0)),
        ('v', Position::new(1, 1)),
        ('>', Position::new(1, 2)),
    ])
}


#[cfg(test)]
mod tests {
    use crate::{init_directional_keypad, manhattan_distance};

    #[test]
    fn test_manhattan_distance() {
        let code = "^v<";
        let keypad = init_directional_keypad();
        
        assert_eq!(manhattan_distance(&keypad, code), 3);

        let code = "<^v";
        let keypad = init_directional_keypad();

        assert_eq!(manhattan_distance(&keypad, code), 6);
    }
    
    
}
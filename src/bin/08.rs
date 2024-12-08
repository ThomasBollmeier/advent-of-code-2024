use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul, Sub};
use itertools::Itertools;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        
        let grid = read_grid(reader)?;
        let result = grid.count_antinodes();
        
        Ok(result)
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = read_grid(reader)?;
        let result = grid.count_antinodes_2();

        Ok(result)
    }
    
    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct Position(i32, i32);

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<i32> for Position {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Position(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<Position> for i32 {
    type Output = Position;
    fn mul(self, rhs: Position) -> Self::Output {
        Position(self * rhs.0, self * rhs.1)
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: i32,
    height: i32,
    antennas: HashMap<char, Vec<Position>>,
}

impl Grid {
    
    fn count_antinodes(&self) -> usize {
        
        let mut locations: HashSet<Position> = HashSet::new();
        
        for (_, positions) in &self.antennas {
            if positions.len() < 2 {
                continue;
            }
            for pairs in positions.iter().combinations(2) {
                for node in self.determine_antinodes(pairs[0], pairs[1]) {
                    locations.insert(node);
                }
            }
        }
        
        locations.len()
    }
    
    fn determine_antinodes(&self, a: &Position, b: &Position) -> Vec<Position> {
        let nodes = vec![
            2 * b.clone() - a.clone(), 
            2 * a.clone() - b.clone()];
        
        nodes.into_iter().filter(|p| self.is_valid_position(p)).collect()
    }

    fn count_antinodes_2(&self) -> usize {

        let mut locations: HashSet<Position> = HashSet::new();

        for (_, positions) in &self.antennas {
            if positions.len() < 2 {
                continue;
            }
            for pairs in positions.iter().combinations(2) {
                for node in self.determine_antinodes_2(pairs[0], pairs[1]) {
                    locations.insert(node);
                }
            }
        }

        locations.len()
    }
    
    fn determine_antinodes_2(&self, a: &Position, b: &Position) -> Vec<Position> {
        let delta = b.clone() - a.clone();
        let mut nodes = vec![];
        
        let mut i = 0;
        loop {
            let node = a.clone() + delta.clone() * i;
            if self.is_valid_position(&node) {
                nodes.push(node);
                i += 1;
            } else {
                break;
            }
        }
        i = -1;
        loop {
            let node = a.clone() + delta.clone() * i;
            if self.is_valid_position(&node) {
                nodes.push(node);
                i -= 1;
            } else {
                break;
            }
        }
        
        nodes
    }
    
    fn is_valid_position(&self, position: &Position) -> bool {
        (0..self.height).contains(&position.0) && (0..self.width).contains(&position.1)
    }
    
}


fn read_grid<R: BufRead>(reader: R) -> Result<Grid> {
    let lines = read_lines(reader);
    let height = lines.len() as i32;
    let width = lines[0].chars().count() as i32;
    let mut antennas = HashMap::new();
    
    for (row, line) in lines.into_iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char != '.' {
                let entry = antennas.entry(char).or_insert_with(Vec::new);
                entry.push(Position(row as i32, col as i32));
            }
        }
    }
    
    Ok(Grid{width, height, antennas})
}
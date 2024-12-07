use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use strum_macros::EnumIter;
use adv_code_2024::*;
use itertools::Itertools;
use strum::IntoEnumIterator;

const DAY: &str = "07"; 
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let equations = read_equations(reader)?;

        let result: i64 = equations.iter()
            .filter(|eq| eq.is_valid() )
            .map(|eq| eq.lhs)
            .sum();

        Ok(result as usize)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let equations = read_equations(reader)?;

        let result: i64 = equations.iter()
            .filter(|eq| eq.is_valid2() )
            .map(|eq| eq.lhs)
            .sum();

        Ok(result as usize)         
    }
    
    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug, EnumIter, Clone)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn cartesian_product(n: usize) -> Vec<Vec<Operator>> {
        let operators = Operator::iter().collect_vec();
        cartesian_product(n, &operators)
    }
}

#[derive(Debug, EnumIter, Clone)]
enum Operator2 {
    Add,
    Multiply,
    Concatenation,
}

impl Operator2 {
    fn cartesian_product(n: usize) -> Vec<Vec<Operator2>> {
        let operators = Operator2::iter().collect_vec();
        cartesian_product(n, &operators)
    }
}

fn cartesian_product<T: Clone>(n: usize, operators: &Vec<T>) -> Vec<Vec<T>> {
    let operators_list = vec![operators.clone(); n];
    let multi_product = operators_list.iter().multi_cartesian_product();
    let mut ret = Vec::new();
    for item in multi_product {
        let mut ops = vec![];
        for op in item {
            ops.push(op.clone());
        }
        ret.push(ops);
    }
    ret
}

#[derive(Debug, Clone)]
struct Equation {
    lhs: i64,
    operands: Vec<i64>,
}

impl Equation {

    fn is_valid(&self) -> bool {
        let n = self.operands.len() - 1;
        let operator_products = Operator::cartesian_product(n);
        for operator_product in operator_products {
            if self.lhs == self.evaluate_rhs(&operator_product) {
                return true;
            }
        }
        false
    }

    fn evaluate_rhs(&self, operators: &[Operator]) -> i64 {
        let mut ret: i64 = self.operands[0];

        for (i, operand) in self.operands.iter().skip(1).enumerate() {
            let op = &operators[i];
            match *op {
                Operator::Add => { ret += operand }
                Operator::Multiply => { ret *= operand }
            }
        }

        ret
    }

    fn is_valid2(&self) -> bool {
        let n = self.operands.len() - 1;
        let operator_products = Operator2::cartesian_product(n);
        for operator_product in operator_products {
            if self.lhs == self.evaluate_rhs2(&operator_product) {
                return true;
            }
        }
        false
    }

    fn evaluate_rhs2(&self, operators: &[Operator2]) -> i64 {
        let mut ret: i64 = self.operands[0];

        for (i, operand) in self.operands.iter().skip(1).enumerate() {
            let op = &operators[i];
            match *op {
                Operator2::Add => { ret += operand }
                Operator2::Multiply => { ret *= operand }
                Operator2::Concatenation => {
                    let a = ret.to_string();
                    let b = operand.to_string();
                    let ab = a + b.as_str();
                    ret = ab.parse::<i64>().unwrap();
                }
            }
        }

        ret
    }
}

fn read_equations<R: BufRead>(reader: R) -> Result<Vec<Equation>> {
    let mut ret = Vec::new();
    let lines = read_lines(reader);

    for line in lines {
        let parts = line.split(":").collect_vec();
        let lhs = parts[0].parse::<i64>()?;
        let operands = parts[1].split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect_vec();
        ret.push(Equation { lhs, operands });
    }

    Ok(ret)
}
use std::fmt::{Debug, Display, Formatter};
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use crate::Instruction::{Adv, Bdv, Bst, Bxl, Bxz, Cdv, Jnz, Out};

const DAY: &str = "17";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<String> {
        let (mut interpreter, program) = read_evaluation_setup(reader)?;
        let output = interpreter.run(&program)?;

        Ok(output.iter().join(","))
    }

    assert_eq!(
        "4,6,3,5,6,3,5,2,1,0",
        part1(BufReader::new(TEST.as_bytes()))?
    );

    //let input_file = BufReader::new(File::open(INPUT_FILE)?);
    //let result = time_snippet!(part1(input_file)?);
    //println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        use Instruction::*;
        let program = vec![Bst(4),
            Bxl(7),
            Cdv(5),
            Adv(3),
            Bxz(4),
            Bxl(7),
            Out(5)];

        let _expected_outputs = [2,4,1,7,7,5,0,3,4,4,1,7,5,5,3,0];

        dbg!(find_as_with_output(&program, 2));
        
        //let (_, program) = read_evaluation_setup(reader)?;

        Ok(0)
    }
    //
    //assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn find_as_with_output(program: &Program, expected: usize) -> Vec<(usize, usize)> {
    let mut ret = vec![]; 
    for a in 0..0b1_000_000_000_usize {
        let mut interpreter = Interpreter::new(a, 0, 0);
        let outputs = interpreter.run(program).unwrap();
        if outputs[0] == expected {
            ret.push((a % 8, a / 8));
        }
    }
    
    ret
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Adv(usize),
    Bxl(usize),
    Bst(usize),
    Jnz(usize),
    Bxz(usize),
    Out(usize),
    Bdv(usize),
    Cdv(usize),
}

impl Instruction {
    fn to_opcode(&self) -> (usize, usize) {
        match self {
            Adv(i) => (0, *i),
            Bxl(i) => (1, *i),
            Bst(i) => (2, *i),
            Jnz(i) => (3, *i),
            Bxz(i) => (4, *i),
            Out(i) => (5, *i),
            Bdv(i) => (6, *i),
            Cdv(i) => (7, *i),
        }
    }
}

type Program = Vec<Instruction>;

#[derive(Clone)]
struct Interpreter {
    a: usize,
    b: usize,
    c: usize,
    output: Vec<usize>,
    debug: bool
}

impl Interpreter {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            a,
            b,
            c,
            output: vec![],
            debug: false
        }
    }

    fn run(&mut self, program: &Program) -> Result<Vec<usize>> {
        self.clear_output();

        let mut ip = 0;
        while ip < program.len() {
            let instruction = &program[ip];
            ip = self.apply_instruction(ip, instruction)?;
            if self.debug {
                dbg!(instruction, &self);
            }
        }

        Ok(self.output.clone())
    }

    fn apply_instruction(&mut self, ip: usize, instruction: &Instruction) -> Result<usize> {
        use Instruction::*;
        match instruction {
            Adv(operand) => {
                self.a = self.divide(*operand)?;
                Ok(ip + 1)
            }
            Bxl(operand) => {
                self.b ^= self.literal(*operand)?;
                Ok(ip + 1)
            }
            Bst(operand) => {
                self.b = self.combo(*operand)? % 8;
                Ok(ip + 1)
            }
            Jnz(operand) => {
                if self.a != 0 {
                    Ok(self.literal(*operand)?)
                } else {
                    Ok(ip + 1)
                }
            }
            Bxz(_) => {
                self.b ^= self.c;
                Ok(ip + 1)
            }
            Out(operand) => {
                self.write(self.combo(*operand)? % 8);
                Ok(ip + 1)
            }
            Bdv(operand) => {
                self.b = self.divide(*operand)?;
                Ok(ip + 1)
            }
            Cdv(operand) => {
                self.c = self.divide(*operand)?;
                Ok(ip + 1)
            }
        }
    }

    fn divide(&mut self, operand: usize) -> Result<usize> {
        let numerator = self.a as f64;
        let denominator = 2_usize.pow(self.combo(operand)? as u32) as f64;
        Ok((numerator / denominator) as usize)
    }

    fn clear_output(&mut self) {
        self.output.clear();
    }

    fn write(&mut self, value: usize) {
        self.output.push(value);
    }

    fn literal(&self, value: usize) -> Result<usize> {
        if (0..=7).contains(&value) {
            Ok(value)
        } else {
            Err(anyhow!("invalid literal value"))
        }
    }

    fn combo(&self, value: usize) -> Result<usize> {
        match value {
            0..=3 => Ok(value),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            _ => Err(anyhow!("invalid combo value")),
        }
    }
}

fn read_evaluation_setup(reader: impl BufRead) -> Result<(Interpreter, Program)> {
    let lines = read_lines(reader);
    let mut registers = [0_usize; 3];

    for (i, line) in lines[..3].iter().enumerate() {
        let parts = line.split(':').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(anyhow!("invalid program line"));
        }
        registers[i] = parts[1].trim().parse::<usize>()?;
    }

    let [a, b, c] = registers;
    let interpreter = Interpreter::new(a, b, c);

    let segments = &lines[4].split(':').collect::<Vec<&str>>();
    let values = segments[1]
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut program = Vec::new();
    for chunk in values.chunks(2) {
        let op = &chunk[0];
        let operand = &chunk[1];

        let instruction = match op {
            0 => Adv(*operand),
            1 => Bxl(*operand),
            2 => Bst(*operand),
            3 => Jnz(*operand),
            4 => Bxz(*operand),
            5 => Out(*operand),
            6 => Bdv(*operand),
            7 => Cdv(*operand),
            _ => return Err(anyhow!("invalid operator")),
        };
        program.push(instruction);
    }

    Ok((interpreter, program))
}

impl Display for Interpreter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "a={:b}\nb={:b}\nc={:b}", self.a, self.b, self.c)
    }
}

impl Debug for Interpreter  {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "a={:b}\nb={:b}\nc={:b}", self.a, self.b, self.c)
    }
}
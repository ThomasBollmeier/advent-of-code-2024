use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

        Ok(output)
    }

    assert_eq!(
        "4,6,3,5,6,3,5,2,1,0",
        part1(BufReader::new(TEST.as_bytes()))?
    );

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
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

type Program = Vec<Instruction>;

#[derive(Debug, Clone)]
struct Interpreter {
    a: usize,
    b: usize,
    c: usize,
    output: String,
}

impl Interpreter {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            a,
            b,
            c,
            output: String::new(),
        }
    }

    fn run(&mut self, program: &Program) -> Result<String> {
        self.clear_output();

        let mut ip = 0;
        while ip < program.len() {
            let instruction = &program[ip];
            ip = self.apply_instruction(ip, instruction)?;
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
                self.b = self.literal(*operand)? ^ self.b;
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
                self.b = self.b ^ self.c;
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
        let value_str = value.to_string();
        if !self.output.is_empty() {
            self.output.push(',');
        }
        self.output.push_str(&value_str);
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
            0 => Instruction::Adv(*operand),
            1 => Instruction::Bxl(*operand),
            2 => Instruction::Bst(*operand),
            3 => Instruction::Jnz(*operand),
            4 => Instruction::Bxz(*operand),
            5 => Instruction::Out(*operand),
            6 => Instruction::Bdv(*operand),
            7 => Instruction::Cdv(*operand),
            _ => return Err(anyhow!("invalid operator")),
        };
        program.push(instruction);
    }

    Ok((interpreter, program))
}

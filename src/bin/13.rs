use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use regex::Regex;

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let machines = read_machines(reader);
        let mut total = 0_usize;

        for machine in machines {
            if let Some(costs) = machine.optimize_winning_costs() {
                total += costs;
            }
        }

        Ok(total)
    }

    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut machines = read_machines(reader);
        let mut total = 0_usize;

        for machine in machines.iter_mut() {
            machine.increase_prize_coords(10000000000000);
            if let Some(costs) = machine.optimize_winning_costs_2() {
                total += costs;
            }
        }

        Ok(total)
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug, Clone)]
struct Button {
    x: usize,
    y: usize,
    cost: usize,
}

impl Button {
    fn new(x: usize, y: usize, cost: usize) -> Button {
        Button { x, y, cost }
    }
}

#[derive(Debug, Clone)]
struct Prize {
    x: usize,
    y: usize,

}

impl Prize {
    fn new(x: usize, y: usize) -> Prize {
        Prize { x, y }
    }
}

#[derive(Debug, Clone)]
struct Machine {
    a: Button,
    b: Button,
    prize: Prize,
}

impl Machine {
    fn new(a: &Button, b: &Button, prize: &Prize) -> Machine {
        Machine { a: a.clone(), b: b.clone(), prize: prize.clone() }
    }

    fn increase_prize_coords(&mut self, amount: usize) {
        self.prize.x += amount;
        self.prize.y += amount;
    }

    fn optimize_winning_costs_2(&self) -> Option<usize> {
        let mut solutions = Vec::new();
        let mut cnt_a = 0_usize;
        let a = &self.a;
        let b = &self.b;
        let prize = &self.prize;

        loop {
            let x = cnt_a * a.x;
            let y = cnt_a * a.y;
            if x > prize.x || y > prize.y {
                break;
            }

            let cnt_b_x = ((prize.x - x) as f64 / b.x as f64).floor() as usize;
            let cnt_b_y = ((prize.y - y) as f64 / b.y as f64).floor() as usize;

            if cnt_b_x == cnt_b_y && cnt_a * a.x + cnt_b_x * b.x == prize.x
            && cnt_a * a.y + cnt_b_y * b.y == prize.y {
                solutions.push((cnt_a, cnt_b_x, cnt_a * a.cost + cnt_b_x * b.cost));
            }

            cnt_a += 1;
        }

        if !solutions.is_empty() {
            solutions.sort_by(|a, b| a.2.cmp(&b.2));
            Some(solutions[0].2)
        } else {
            None
        }
    }

    fn optimize_winning_costs(&self) -> Option<usize> {
        let mut solutions = Vec::new();
        let mut cnt_a = 0_usize;
        let mut cnt_b;
        let a = &self.a;
        let b = &self.b;
        let prize = &self.prize;

        loop {
            let (x, y) = (cnt_a * a.x, cnt_a * a.y);
            if x > prize.x || y > prize.y {
                break;
            }
            cnt_b = 0;
            loop {
                let (x, y) = (cnt_a * a.x + cnt_b * b.x, cnt_a * a.y + cnt_b * b.y);
                if x > prize.x || y > prize.y {
                    break;
                }
                if x == prize.x && y == prize.y {
                    solutions.push((cnt_a, cnt_b, cnt_a * a.cost + cnt_b * b.cost));
                }
                cnt_b += 1;
            }
            cnt_a += 1;
        }

        if !solutions.is_empty() {
            solutions.sort_by(|a, b| a.2.cmp(&b.2));
            Some(solutions[0].2)
        } else {
            None
        }
    }
}

fn read_machines(reader: impl BufRead) -> Vec<Machine> {
    let mut buttons = Vec::new();
    let mut prize_opt: Option<Prize> = None;
    let mut machines = Vec::new();
    let re = Regex::new(r"\d+").unwrap();

    for line in read_lines(reader) {
        if line.starts_with("Button") {
            let mut steps = Vec::new();
            for cap in re.captures_iter(&line) {
                let step = cap.get(0).unwrap().as_str().parse::<usize>().unwrap();
                steps.push(step);
            }
            if steps.len() == 2 {
                let cost = if buttons.is_empty() { 3 } else { 1 };
                let button = Button::new(steps[0], steps[1], cost);
                buttons.push(button);
            }
        } else if line.starts_with("Prize") {
            let mut coords = Vec::new();
            for cap in re.captures_iter(&line) {
                let coord = cap.get(0).unwrap().as_str().parse::<usize>().unwrap();
                coords.push(coord);
            }
            if coords.len() == 2 {
                prize_opt = Some(Prize::new(coords[0], coords[1]));
            }
        }
        if buttons.len() == 2 {
            if let Some(price) = &prize_opt {
                let machine = Machine::new(&buttons[0], &buttons[1], price);
                machines.push(machine);
                buttons.clear();
                prize_opt = None;
            }
        }
    }

    machines
}
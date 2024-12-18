use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;
use adv_code_2024::grid::{Grid, Position};

const DAY: &str = "18";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, num_rows: usize, num_cols: usize, time: usize) -> Result<usize> {
        let memory = read_memory(reader, num_rows, num_cols);
        
        memory.print(time);
        
        Ok(0)
    }

    assert_eq!(22, part1(BufReader::new(TEST.as_bytes()), 7, 7, 12)?);

    //let input_file = BufReader::new(File::open(INPUT_FILE)?);
    //let result = time_snippet!(part1(input_file)?);
    //println!("Result = {}", result);
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

#[derive(Debug, Clone)]
struct Memory {
    grid: Grid<usize>
}

impl Memory {
    fn new(grid: Grid<usize>) -> Self {
        Self { grid }
    }
    
    fn print(&self, time: usize) 
    {
        for y in 0..self.grid.num_rows {
            for x in 0..self.grid.num_cols {
                let value = self.grid.value_at(&Position::new(y, x)).unwrap();
                if time < *value {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }
}

fn read_memory<R: BufRead>(reader: R, num_rows: usize, num_cols: usize) -> Memory {
    let mut cells = vec![vec![0_usize; num_cols]; num_rows];
    let lines = read_lines(reader);

    for (t, line) in lines.iter().enumerate() {
        let coords = line
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        cells[coords[1]][coords[0]] = t + 1;
        dbg!(coords);
    }
    
    let grid = Grid::new(num_rows as i32, num_cols as i32, cells);
    
    Memory::new(grid)    
}
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "09"; 
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let blocks = read_disk_map(reader);
        let compressed_blocks = compress(&blocks);
        let result = checksum(&compressed_blocks);
        
        Ok(result)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

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

fn checksum(blocks: &[Block]) -> usize {
    let mut sum = 0;
    let mut idx = 0_usize;
    
    for block in blocks {
        match block.block_type {
            BlockType::File {id} => {
                for _ in 0..block.size {
                    sum += id * idx;
                    idx += 1;
                }
            }
            BlockType::Empty => {}
        }
    } 
    
    sum
}

fn compress(blocks: &Vec<Block>) -> Vec<Block> {
    let mut new_blocks: Vec<Block> = blocks.clone();
    
    loop {
        if let Some(compressed) = compress_step(&new_blocks) {
            new_blocks = compressed;
        } else {
            return new_blocks;
        }
    }    
}

fn compress_step(blocks: &Vec<Block>) -> Option<Vec<Block>> {
    let mut new_blocks: Vec<Block> = Vec::new();
    let mut idx_first_empty : i32 = -1;
    let mut idx_last_file : i32 = -1;
    
    for (idx, block) in blocks.iter().enumerate() {
        match block.block_type {
            BlockType::Empty => if idx_first_empty == -1 && block.size > 0 {
                idx_first_empty = idx as i32;
            }
            BlockType::File{id:_}=> {
                idx_last_file = idx as i32
            },
        }
    }
    
    if idx_first_empty == -1 {
        return None;
    }
    if idx_last_file == -1 {
        return None
    }
    if idx_first_empty > idx_last_file {
        return None
    }
    
    let first_empty = &blocks[idx_first_empty as usize];
    let last_file = &blocks[idx_last_file as usize];
    let last_file_id = if let BlockType::File{id} = last_file.block_type {
        id
    } else {
        panic!("Expected file block type")
    };
    
    for (idx, block) in blocks.iter().enumerate() {
        if idx_first_empty == idx as i32 {
            if last_file.size >= first_empty.size {
                new_blocks.push(Block{
                    block_type: BlockType::File {id: last_file_id},
                    size: first_empty.size,
                });
            } else {
                new_blocks.push(Block{
                    block_type: BlockType::File {id: last_file_id},
                    size: last_file.size,
                });
                new_blocks.push(Block{
                    block_type: BlockType::Empty,
                    size: first_empty.size - last_file.size,
                });
            }
        } else if idx_last_file == idx as i32 {
            if last_file.size > first_empty.size {
                new_blocks.push(Block{
                    block_type: BlockType::File {id: last_file_id},
                    size: last_file.size - first_empty.size,
                });
            }
        } else {
            new_blocks.push(block.clone());    
        }
    }
    
    Some(new_blocks)
}

fn read_disk_map<R: BufRead>(reader: R) -> Vec<Block> {
    let mut ret = Vec::new();
    let lines: Vec<String> = read_lines(reader);
    let mut next_id = 0;
    
    for (i, ch) in lines[0].chars().enumerate() {
        let size = ch.to_digit(10).unwrap() as i32;
        let block = if i % 2 == 0 {
            let id = next_id;
            next_id += 1;
            Block {
                block_type: BlockType::File { id },
                size,
            }
        } else {
            Block {
                block_type: BlockType::Empty,
                size,
            }
        };
        ret.push(block);
    } 
    
    ret
}

#[derive(Debug, Clone)]
enum BlockType { 
    File{id: usize}, 
    Empty,
}

#[derive(Debug, Clone)]
struct Block {
    block_type: BlockType,
    size: i32,
}

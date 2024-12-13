use adv_code_2024::grid::{Direction, Grid, Position};
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use strum::IntoEnumIterator;

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = read_grid(reader);
        let regions = find_regions(&grid);
        let answer = regions
            .regions
            .values()
            .map(|r| r.area * r.perimeter)
            .sum::<usize>();

        Ok(answer)
    }

    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = read_grid(reader);
        let mut regions = find_regions(&grid);
        update_corner_counts(&mut regions, &grid);

        let answer = regions
            .regions
            .values()
            .map(|r| r.area * r.num_corners)
            .sum::<usize>();

        Ok(answer)
    }

    assert_eq!(1206, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug)]
struct Region {
    _value: char,
    positions: Vec<Position>,
    area: usize,
    perimeter: usize,
    num_corners: usize,
}

#[derive(Debug)]
struct Regions {
    regions: HashMap<usize, Region>,
}

fn update_corner_counts(regions: &mut Regions, grid: &Grid<char>) {
    use Direction::*;

    let mut region_cells = vec![vec![0_usize; grid.num_cols as usize]; grid.num_rows as usize];
    for (id, region) in &regions.regions {
        for pos in region.positions.iter() {
            region_cells[pos.row() as usize][pos.col() as usize] = *id;
        }
    }
    let region_grid = Grid {
        num_rows: grid.num_rows,
        num_cols: grid.num_cols,
        cells: region_cells,
    };

    for row in 0..=grid.num_rows {
        for col in 0..=grid.num_cols {
            let pos = Position::new(row, col);
            let north = pos.make_step(&North);
            let west = pos.make_step(&West);
            let nw = north.make_step(&West);
            let corner_regions: Vec<usize> = [pos.clone(), north.clone(), west.clone(), nw.clone()]
                .iter()
                .filter(|p| region_grid.is_valid_position(p))
                .map(|p| region_grid.value_at(p).unwrap())
                .cloned()
                .collect();
            let mut corner_stats = HashMap::new();
            for id in corner_regions {
                *corner_stats.entry(id).or_insert(0usize) += 1;
            }
            for (id, cnt) in corner_stats {
                if cnt == 1 || cnt == 3 {
                    let region = regions.regions.get_mut(&id).unwrap();
                    region.num_corners += 1;
                } else if cnt == 2 && (region_grid.value_at(&pos).is_some_and(|x| *x == id)
                        && region_grid.value_at(&nw).is_some_and(|x| *x == id) || region_grid.value_at(&west).is_some_and(|x| *x == id)
                            && region_grid.value_at(&north).is_some_and(|x| *x == id)) {
                    let region = regions.regions.get_mut(&id).unwrap();
                    region.num_corners += 2;
                }
            }
        }
    }
}

fn find_regions(grid: &Grid<char>) -> Regions {
    let mut next_id = 1_usize;
    let mut regions = HashMap::new();
    let mut visited = HashSet::new();

    while let Some(start) = find_unvisited_pos(grid, &visited) {
        let id = next_id;
        next_id += 1;
        let region = find_region(grid, &start, &mut visited);
        regions.insert(id, region);
    }

    Regions { regions }
}

fn find_region(grid: &Grid<char>, start: &Position, visited: &mut HashSet<Position>) -> Region {
    let mut todo: Vec<Position> = vec![start.clone()];
    let mut area = 0;
    let mut perimeter = 0;
    let mut positions = Vec::new();

    let value = grid.cells[start.row() as usize][start.col() as usize];

    while let Some(current) = todo.pop() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());
        area += 1;
        positions.push(current.clone());

        let mut other_dirs = HashSet::new();
        for dir in Direction::iter() {
            let neighbor = current.make_step(&dir);
            if !grid.is_valid_position(&neighbor) {
                perimeter += 1;
                other_dirs.insert(dir);
                continue;
            }
            let nb_value = grid.cells[neighbor.row() as usize][neighbor.col() as usize];
            if nb_value != value {
                perimeter += 1;
                other_dirs.insert(dir);
                continue;
            }
            if visited.contains(&neighbor) {
                continue;
            }
            todo.push(neighbor);
        }
    }

    Region {
        _value: value,
        positions,
        area,
        perimeter,
        num_corners: 0,
    }
}

fn find_unvisited_pos(grid: &Grid<char>, visited: &HashSet<Position>) -> Option<Position> {
    for row in 0..grid.num_rows {
        for col in 0..grid.num_cols {
            let pos = Position::new(row, col);
            if !visited.contains(&pos) {
                return Some(pos);
            }
        }
    }

    None
}

fn read_grid(reader: impl BufRead) -> Grid<char> {
    let lines = read_lines(reader);
    let mut cells = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for ch in line.chars() {
            row.push(ch);
        }
        cells.push(row);
    }

    Grid {
        num_rows: cells.len() as i32,
        num_cols: cells[0].len() as i32,
        cells,
    }
}

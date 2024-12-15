use std::collections::{HashSet, VecDeque};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use adv_code_2024::grid::{Direction, Grid, Position};

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (mut warehouse, movements) = read_warehouse(reader)?;
        warehouse.move_robot(&movements);
        
        Ok(warehouse.gps_box_sum())
    }

    assert_eq!(10092, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (warehouse, movements) = read_warehouse(reader)?;
        let mut warehouse = Warehouse2::from_warehouse(&warehouse);
        
        warehouse.move_robot(&movements);
        Ok(warehouse.gps_box_sum())
    }
    
    assert_eq!(9021, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Cell2 {
    Robot,
    BoxLeft,
    BoxRight,
    Wall,
    Empty,
}

#[derive(Debug)]
struct Warehouse2 {
    grid: Grid<Cell2>,
    robot_pos: Position,
}

impl Warehouse2 {
    fn from_warehouse(warehouse: &Warehouse) -> Self {
        let grid = &warehouse.grid;
        let num_rows = grid.num_rows;
        let num_cols = grid.num_cols * 2;
        let robot_pos = Position::new(warehouse.robot_pos.row(), 
                                      warehouse.robot_pos.col() * 2);
        let mut cells = Vec::new();
        
        for row in &grid.cells {
            let mut row_cells = Vec::new();
            for cell in row {
                match cell {
                    Cell::Empty => {
                        row_cells.push(Cell2::Empty);
                        row_cells.push(Cell2::Empty);
                    }
                    Cell::Robot => {
                        row_cells.push(Cell2::Robot);
                        row_cells.push(Cell2::Empty);
                    }
                    Cell::Box => {
                        row_cells.push(Cell2::BoxLeft);
                        row_cells.push(Cell2::BoxRight);
                    }
                    Cell::Wall => {
                        row_cells.push(Cell2::Wall);
                        row_cells.push(Cell2::Wall);
                    }
                }
            }
            cells.push(row_cells);
        }
        
        Warehouse2 { grid: Grid::new(num_rows, num_cols, cells), robot_pos }
    }

    fn gps_box_sum(&self) -> usize {
        let mut total = 0;
        for (y, row) in self.grid.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Cell2::BoxLeft => {
                        total += 100 * y + x;
                    }
                    _ => continue,
                }
            }
        }

        total
    }
    
    fn move_robot(&mut self, movements: &Movements) {
        movements.iter().for_each(|movement| {
            self.move_robot_one_step(movement);
        });
    }
    
    fn move_robot_one_step(&mut self, direction: &Direction) {
        let target_pos = self.robot_pos.make_step(direction);
        let target_value = self.grid.value_at(&target_pos);
        if target_value.is_none() {
            return;
        }
        let target_value = target_value.unwrap();

        match target_value {
            Cell2::Empty => {
            }
            Cell2::BoxLeft | Cell2::BoxRight => {
                if self.can_box_move(&target_pos, direction) {
                    let box_move_positions = self.determine_box_move_order(&target_pos, direction);
                    box_move_positions.iter().for_each(|position| {
                        self.move_box(position, direction);
                    });
                } else {
                    return;
                }
            }
            _ => { return; }
        }

        self.grid.set_value_at(&self.robot_pos, Cell2::Empty);
        self.grid.set_value_at(&target_pos, Cell2::Robot);
        self.robot_pos = target_pos;
    }
    
    fn can_box_move(&self, from_pos: &Position, direction: &Direction) -> bool {
        let (left_pos, right_pos) = self.get_all_box_positions(from_pos);
        let to_positions = self.get_box_destinations(&left_pos, &right_pos, direction);

        for to_pos in &to_positions {
            let to_value = self.grid.value_at(to_pos);
            if to_value.is_none() {
                return false;
            }
            let to_value = to_value.unwrap();

            match to_value {
                Cell2::Empty => continue,
                Cell2::BoxLeft | Cell2::BoxRight => {
                    if !self.can_box_move(to_pos, direction) {
                        return false;
                    }
                }
                _ => { return false; }
            }
        }
        
        true
    }
    
    fn determine_box_move_order(&self, 
                                box_pos: &Position, 
                                direction: &Direction) -> Vec<Position> {

        use Cell2::*;
        
        let mut box_moves = Vec::new();
        let mut todos = VecDeque::new();
        let mut processed: HashSet<Position> = HashSet::new();
        todos.push_back(box_pos.clone());
        
        while let Some(current) = todos.pop_front() {
            if processed.contains(&current) {
                continue;
            }
            processed.insert(current.clone());
            box_moves.insert(0, current.clone());

            let (left_pos, right_pos) = self.get_all_box_positions(&current);
            let destinations = self.get_box_destinations(&left_pos, &right_pos, direction);
            
            for destination in &destinations {
                let destination_value = self.grid.value_at(destination);
                match destination_value {
                    Some(destination_value) => {
                        let next_box = match destination_value {
                            BoxLeft => destination.clone(),
                            BoxRight => destination.make_step(&Direction::West),
                            _ => continue,
                        };
                        if processed.contains(&next_box) {
                            continue;
                        }
                        todos.push_back(next_box.clone());
                    }
                    None => continue,
                }
            }
        }
        
        box_moves
    }

    fn move_box(&mut self, from_pos: &Position, direction: &Direction) {
        let (left_pos, right_pos) = self.get_all_box_positions(from_pos);
        let destinations = self.get_box_destinations(&left_pos, &right_pos, direction);

        match direction {
            Direction::North | Direction::South=> {
                let new_left = &destinations[0];
                let new_right = &destinations[1];
                self.grid.set_value_at(&left_pos, Cell2::Empty);
                self.grid.set_value_at(&right_pos, Cell2::Empty);
                self.grid.set_value_at(new_left, Cell2::BoxLeft);
                self.grid.set_value_at(new_right, Cell2::BoxRight);
            }
            Direction::East => {
                let new_right = &destinations[0];
                self.grid.set_value_at(new_right, Cell2::BoxRight);
                self.grid.set_value_at(&right_pos, Cell2::BoxLeft);
                self.grid.set_value_at(&left_pos, Cell2::Empty);
            }
            Direction::West => {
                let new_left = &destinations[0];
                self.grid.set_value_at(new_left, Cell2::BoxLeft);
                self.grid.set_value_at(&left_pos, Cell2::BoxRight);
                self.grid.set_value_at(&right_pos, Cell2::Empty);
            }
        }
    }
    
    fn get_box_destinations(&self, 
                            left_pos: &Position, 
                            right_pos: &Position, 
                            direction: &Direction) -> Vec<Position> {
        match direction {
            Direction::North | Direction::South => {
                vec![
                    left_pos.make_step(direction),
                    right_pos.make_step(direction),
                ]
            }
            Direction::East => {
                vec![right_pos.make_step(direction)]
            }
            Direction::West => {
                vec![left_pos.make_step(direction)]
            }
        }
    }
    
    fn get_all_box_positions(&self, position: &Position) -> (Position, Position) {
        let box_value = self.grid.value_at(position).unwrap();
        let left_pos: Position;
        let right_pos: Position;

        match box_value {
            Cell2::BoxLeft => {
                left_pos = position.clone();
                right_pos = left_pos.make_step(&Direction::East);
            }
            Cell2::BoxRight => {
                right_pos = position.clone();
                left_pos = right_pos.make_step(&Direction::West);
            }
            _ => panic!("must not happen!")
        }
        
        (left_pos, right_pos)
    }

    #[allow(dead_code)]
    fn print(&self) {
        for line in &self.grid.cells {
            for cell in line {
                match cell {
                    Cell2::Robot => { print!("@"); }
                    Cell2::BoxLeft => { print!("["); }
                    Cell2::BoxRight => { print!("]"); }
                    Cell2::Wall => { print!("#"); }
                    Cell2::Empty => { print!("."); }
                }
            }
            println!();
        }
        println!();
    }

}

#[derive(Debug)]
enum Cell {
    Robot,
    Box,
    Wall,
    Empty,
}

#[derive(Debug)]
struct Warehouse {
    grid: Grid<Cell>,
    robot_pos: Position,
}

impl Warehouse {
    fn new(grid: Grid<Cell>, robot_pos: Position) -> Self {
        Self { grid, robot_pos }
    }
    
    fn gps_box_sum(&self) -> usize {
        let mut total = 0;
        for (y, row) in self.grid.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Box => {
                        total += 100 * y + x;               
                    }
                    _ => continue,
                }
            }
        }
        
        total
    }

    #[allow(dead_code)]
    fn print(&self) {
        for line in &self.grid.cells {
            for cell in line {
                match cell {
                    Cell::Robot => { print!("@"); }
                    Cell::Box => { print!("O"); }
                    Cell::Wall => { print!("#"); }
                    Cell::Empty => { print!("."); }
                }
            }
            println!();
        }
        println!();
    }
    
    fn move_robot(&mut self, movements: &Movements) {
        movements.iter().for_each(|movement| {
            self.move_robot_one_step(movement);
        })
    }
    
    fn move_robot_one_step(&mut self, direction: &Direction) {
        let target_pos = self.robot_pos.make_step(direction);
        let target = self.grid.value_at(&target_pos);
        
        if target.is_none() {
            return;
        }
        let target = target.unwrap();
        
        match target {
            Cell::Empty => {},
            Cell::Box => {
                if let Some(empty_pos) = self.find_empty_cell(&target_pos, direction) {
                    self.grid.set_value_at(&empty_pos, Cell::Box);
                } else {
                    return;
                }
            }
            _ => { return; }
        }

        self.grid.set_value_at(&self.robot_pos, Cell::Empty);
        self.grid.set_value_at(&target_pos, Cell::Robot);
        self.robot_pos = target_pos;
    }
    
    fn find_empty_cell(&self, start_pos: &Position, direction: &Direction) -> Option<Position> {
        let mut pos = start_pos.clone();
        while self.grid.is_valid_position(&pos) {
            let cell = self.grid.value_at(&pos).unwrap();
            match cell {
                Cell::Empty => return Some(pos),
                Cell::Box => {
                    pos = pos.make_step(direction);
                }
                _ => return None,
            }
        }
        None
    }
}

type Movements = Vec<Direction>;

fn read_warehouse<R: BufRead>(reader: R) -> Result<(Warehouse, Movements)> {
    let lines = read_lines(reader); 
    let mut warehouse_lines = Vec::new();
    let mut movement_lines = Vec::new();
    let mut parse_warehouse = true;
    
    for line in lines {
        if line.is_empty() {
            parse_warehouse = false;
            continue;
        }
        if parse_warehouse {
            warehouse_lines.push(line);
        } else {
            movement_lines.push(line);
        }
    }
    
    let num_rows = warehouse_lines.len() as i32;
    let num_columns = warehouse_lines[0].len() as i32;
    let mut cells = Vec::new();
    let mut robot_pos: Option<Position> = None;
    
    for (r, line) in warehouse_lines.iter().enumerate() {
        let mut row = Vec::new();
        for (c, ch) in line.chars().enumerate() {
            let cell = match ch {
              '@' => {
                  robot_pos = Some(Position::new(r as i32, c as i32));
                  Cell::Robot
              },
              'O' => Cell::Box,
              '#' => Cell::Wall,
              _ => Cell::Empty,
            };
            row.push(cell);
        }
        cells.push(row);
    }
    
    if robot_pos.is_none() {
        return Err(Error::msg("no robot given"));
    }
    let robot_pos = robot_pos.unwrap();
    let grid = Grid::new(num_rows, num_columns, cells);
    let warehouse = Warehouse::new(grid, robot_pos);

    let mut movements = Vec::new();
    for line in movement_lines {
        for ch in line.chars() {
            match ch {
                '^' => movements.push(Direction::North),
                '>' => movements.push(Direction::East),
                'v' => movements.push(Direction::South),
                '<' => movements.push(Direction::West),
                _ => return Err(Error::msg("invalid movement")),
            }
        }
    }    

    Ok((warehouse, movements))
}

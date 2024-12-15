use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position(i32, i32);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn row(&self) -> i32 {
        self.0
    }

    pub fn col(&self) -> i32 {
        self.1
    }

    pub fn make_step(&self, direction: &Direction) -> Self {
        use Direction::*;
        match direction {
            North => Self(self.0 - 1, self.1),
            South => Self(self.0 + 1, self.1),
            East => Self(self.0, self.1 + 1),
            West => Self(self.0, self.1 - 1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub num_rows: i32,
    pub num_cols: i32,
    pub cells: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(num_rows: i32, num_cols: i32, cells: Vec<Vec<T>>) -> Self {
        Self { num_rows, num_cols, cells }
    }
    
    pub fn is_valid_position(&self, position: &Position) -> bool {
        (0..self.num_rows).contains(&position.0) && (0..self.num_cols).contains(&position.1)
    }

    pub fn value_at(&self, position: &Position) -> Option<&T> {
        if self.is_valid_position(position) {
            Some(&self.cells[position.row() as usize][position.col() as usize])
        } else {
            None
        }
    }
    
    pub fn set_value_at(&mut self, position: &Position, value: T) {
        if self.is_valid_position(position) {
            self.cells[position.row() as usize][position.col() as usize] = value;
        }
    }
}

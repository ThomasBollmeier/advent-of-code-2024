use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone)]
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
    
    pub fn is_valid_position(&self, position: &Position) -> bool {
        (0..self.num_rows).contains(&position.0) && (0..self.num_cols).contains(&position.1)
    }
    
}
 
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::{Captures, Regex};
use adv_code_2024::*;

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, width: usize, height: usize) -> Result<usize> {
        let robots = read_robots(reader);
        let area = Area::new(width, height, robots);
        area.move_robots(100);
        
        let counts = area.count_robots_per_quadrant();
        let mut answer = 1;
        for quadrant in 1..=4_usize {
            match counts.get(&quadrant) {
                Some(count) => { answer *= count; },
                None => { answer = 0 }
            }
        }
        
        Ok(answer)
    }

    assert_eq!(12, part1(BufReader::new(TEST.as_bytes()), 11, 7)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 101, 103)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R, width: usize, height: usize) -> Result<usize> {
        let robots = read_robots(reader);
        let area = Area::new(width, height, robots);
        let mut time = 0;
        const MIN_CLUSTER_SIZE: usize = 20;
        
        loop {
            println!("time: {}", time);
            let max_cluster_size = area.find_max_cluster_size();
            
            if max_cluster_size >= MIN_CLUSTER_SIZE {
                println!("max cluster size: {}", max_cluster_size);
                area.print_robots();
                return Ok(time);
            }
            
            area.move_robots(1);
            time += 1;
            println!();
        }
    }
    
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 101, 103)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

type Number = i32;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Vec2 {
    x: Number, 
    y: Number,
}

impl Vec2 {
    fn new(x: Number, y: Number) -> Vec2 {
        Vec2 { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Vec2,
    vel: Vec2,
}

impl Robot {
    fn new(pos: Vec2, vel: Vec2) -> Robot {
        Robot { pos, vel }
    }
}

#[derive(Debug, Clone)]
struct Area {
    width: usize,
    height: usize,
    robots: RefCell<Vec<Robot>>,
}

impl Area {
    fn new(width: usize, height: usize, robots: Vec<Robot>) -> Area {
        Area { width, height, robots: RefCell::new(robots) }
    }
    
    fn get_robots(&self) -> Vec<Robot> {
        self.robots.borrow().clone()
    }
    
    fn find_max_cluster_size(&self) -> usize {
        *self.find_clusters().iter().max().unwrap()
    }
    
    fn find_clusters(&self) -> Vec<usize> {
        let mut ret = Vec::new();
        
        let mut robot_positions = HashSet::new();
        for robot in self.get_robots() {
            robot_positions.insert(robot.pos);
        }
        
        let mut visited: HashSet<Vec2> = HashSet::new();
        
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Vec2::new(x as i32, y as i32);
                if visited.contains(&pos) {
                    continue;
                }
                if !robot_positions.contains(&pos) {
                    continue;
                }
                let cluster = self.find_cluster(&pos, &robot_positions, &mut visited);
                ret.push(cluster.len());
            }
        }
        
        ret
    }
    
    fn find_cluster(&self, 
                    start_pos: &Vec2, 
                    robot_positions: &HashSet<Vec2>, 
                    visited: &mut HashSet<Vec2>) -> Vec<Vec2> {
        
        let mut ret = Vec::new();
        let mut todo = vec![*start_pos];
        
        while let Some(pos) = todo.pop() {
            if visited.contains(&pos) {
                continue;
            }
            
            visited.insert(pos);
            ret.push(pos);
            
            let mut neighbors = Vec::new();
            if pos.x > 0 {
                neighbors.push(Vec2::new(pos.x - 1, pos.y));
            }
            if pos.x < self.width as Number - 1 {
                neighbors.push(Vec2::new(pos.x + 1, pos.y));
            }
            if pos.y > 0 {
                neighbors.push(Vec2::new(pos.x, pos.y - 1));
            }
            if pos.y < self.height as Number - 1 {
                neighbors.push(Vec2::new(pos.x, pos.y + 1));
            }
            
            for neighbor in neighbors.iter()
                .filter(|nb| !visited.contains(*nb) && robot_positions.contains(*nb)) {
                todo.push(*neighbor);
            }
            
        }
        
        ret
    }
    
    
    
    fn print_robots(&self) {
        let mut counts: HashMap<Vec2, usize> = HashMap::new();
        for robot in self.get_robots() {
            *counts.entry(robot.pos).or_default() += 1;
        }
        
        for y in 0..self.height {
            for x in 0..self.width {
                let cnt = counts.get(&Vec2::new(x as Number, y as Number));
                match cnt {
                    Some(_) => { print!("#"); }
                    None => { print!("."); } 
                }
            }
            println!();
        }
    }
    
    fn count_robots_per_quadrant(&self) -> HashMap<usize, usize> {
        let mut counts = HashMap::new();
        
        for robot in self.robots.borrow().iter() {
            match self.get_quadrant(&robot.pos) {
                Some(q) => *counts.entry(q).or_insert(0) += 1,
                None => continue,
            }
        }
        
        counts
    }
    
    fn get_quadrant(&self, position: &Vec2) -> Option<usize> {
        let center_x = self.width as i32 / 2;
        let center_y = self.height as i32 / 2;
        
        match position.x.cmp(&center_x) {
            std::cmp::Ordering::Less => match position.y.cmp(&center_y) {
                std::cmp::Ordering::Less => Some(1),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => Some(4),
            },
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => match position.y.cmp(&center_y) {  
                std::cmp::Ordering::Less => Some(2),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => Some(3),
            },
        }
    }
    
    fn move_robots(&self, time: Number) {
        for robot in self.robots.borrow_mut().iter_mut() {
            self.move_robot(robot, time);
        }
    }
    
    fn move_robot(&self, robot: &mut Robot, time: Number) {
        robot.pos.x = Self::mov(robot.pos.x, robot.vel.x, time, self.width);
        robot.pos.y = Self::mov(robot.pos.y, robot.vel.y, time, self.height);
    }
    
    fn mov(x: Number, v: Number, t: Number, limit: usize) -> Number {
        let mut ret = x + t * v;
        ret %= limit as Number;
        if ret < 0 {
            ret += limit as Number;
        }
        ret
    }
}

fn read_robots(reader: impl BufRead) -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();
    let re = Regex::new(r"p=(.+),(.+)\s+v=(.+),(.+)").unwrap();
    
    for line in read_lines(reader) {
        for cap in re.captures_iter(&line) {
            let x = to_number(&cap, 1);
            let y = to_number(&cap, 2);
            let vx = to_number(&cap, 3);
            let vy = to_number(&cap, 4);
            robots.push(Robot::new(Vec2::new(x, y), Vec2::new(vx, vy)));
        }
    }
    
    robots
}

fn to_number(cap: &Captures, i: usize) -> Number {
    cap.get(i).unwrap().as_str().parse::<Number>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_robot_move() {
        let robot = Robot::new(Vec2::new(2, 4), Vec2::new(2, -3));
        let area = Area::new(11, 7, vec![robot]);
        area.move_robots(5);
        let new_robots = area.get_robots();
        assert_eq!(new_robots.len(), 1);
        assert_eq!(new_robots[0].pos, Vec2::new(1, 3));
    }
    
}
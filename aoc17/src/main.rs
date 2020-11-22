use intcomputer::{IntComputer, ProgramState};
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
enum AbsDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

enum RelDirection {
    LEFT,
    RIGHT,
    STRAIGHT,
}

struct Robot<'a> {
    current_position: (isize, isize),
    current_direction: AbsDirection,
    scaffolding: &'a HashSet<(usize, usize)>,
}

impl Robot<'_> {
    pub fn tile_ahead(&self) -> (isize, isize) {
        let (x, y) = self.current_position;
        match self.current_direction {
            AbsDirection::UP => (x, y - 1),
            AbsDirection::DOWN => (x, y + 1),
            AbsDirection::LEFT => (x - 1, y),
            AbsDirection::RIGHT => (x + 1, y),
        }
    }

    pub fn tile_left(&self) -> (isize, isize) {
        let (x, y) = self.current_position;
        match self.current_direction {
            AbsDirection::UP => (x - 1, y),
            AbsDirection::DOWN => (x + 1, y),
            AbsDirection::LEFT => (x, y + 1),
            AbsDirection::RIGHT => (x, y - 1),
        }
    }

    pub fn tile_right(&self) -> (isize, isize) {
        let (x, y) = self.current_position;
        match self.current_direction {
            AbsDirection::UP => (x + 1, y),
            AbsDirection::DOWN => (x - 1, y),
            AbsDirection::LEFT => (x, y - 1),
            AbsDirection::RIGHT => (x, y + 1),
        }
    }

    pub fn turn_left(&mut self) {
        self.current_direction = match self.current_direction {
            AbsDirection::UP => AbsDirection::LEFT,
            AbsDirection::LEFT => AbsDirection::DOWN,
            AbsDirection::DOWN => AbsDirection::RIGHT,
            AbsDirection::RIGHT => AbsDirection::UP,
        }
    }

    pub fn turn_right(&mut self) {
        self.current_direction = match self.current_direction {
            AbsDirection::UP => AbsDirection::RIGHT,
            AbsDirection::RIGHT => AbsDirection::DOWN,
            AbsDirection::DOWN => AbsDirection::LEFT,
            AbsDirection::LEFT => AbsDirection::UP,
        }
    }

    pub fn step(&mut self) {
        self.current_position = self.tile_ahead();
    }

    pub fn is_valid_move(&self, tile: (isize, isize)) -> bool {
        let (x,y) = tile;
        if x < 0 || y < 0 {
            return false
        }
        self.scaffolding.contains(&(x as usize, y as usize))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();

    let program: Vec<i64> = contents
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut computer = IntComputer::new(program);
    while computer.state == ProgramState::Running {
        computer.run();
    }

    // Draw to console
    let output: String = computer
        .output
        .clone()
        .iter()
        .map(|&x| (x as u8) as char)
        .collect();
    println!("{}", output);

    // Store walkable tiles
    let mut scaffolding: HashSet<(usize, usize)> = HashSet::new();
    let output_slice = computer.output.as_slices().0;

    // Split along newlines
    let rows: Vec<&[i64]> = output_slice.split(|&x| x == 10).collect();
    let num_rows = rows.len();
    let num_cols = rows[0].len();

    let mut starting_position = (0, 0);
    for (y, row) in rows.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            // '#' => insert in Hashset
            if c == 35 {
                scaffolding.insert((x, y));
            // '^' => starting position
            } else if c == 94 {
                starting_position = (x, y);
            }
        }
    }

    let is_intersection = |&(x, y)| {
        if x == 0 || x == num_cols - 1 {
            return false;
        }
        if y == 0 || y == num_rows - 1 {
            return false;
        }
        scaffolding.contains(&(x - 1, y))
            && scaffolding.contains(&(x + 1, y))
            && scaffolding.contains(&(x, y - 1))
            && scaffolding.contains(&(x, y + 1))
    };

    let part1 = scaffolding
        .iter()
        .filter(|&x| is_intersection(x))
        .fold(0, |acc, &(x, y)| acc + x * y);
    println!("Solution Part 1: {}", part1);

    let mut robot = Robot {
        current_position: (starting_position.0 as isize, starting_position.1 as isize),
        current_direction: AbsDirection::UP,
        scaffolding: &scaffolding,
    };

    let mut path: Vec<String> = vec![];
    loop {
        let mut steps = 0;
        while robot.is_valid_move(robot.tile_ahead()) {
            robot.step();
            steps +=1;
        }
        if steps != 0 {
            path.push(steps.to_string());
        }
        if robot.is_valid_move(robot.tile_left()) {
            robot.turn_left();
            path.push("L".into());
            continue
        }
        if robot.is_valid_move(robot.tile_right()) {
            robot.turn_right();
            path.push("R".into());
            continue
        }
        break
    }
    println!("{:?}", path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {}
}

use intcomputer::*;
use std::collections::HashMap;
use std::env;
use std::fs;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
enum Color {
    Black = 0,
    White = 1,
}

#[derive(Debug, PartialEq, Hash, Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl Eq for Position {}

struct PaintingRobot {
    computer: IntComputer,
    pos: Position,
    dir: Direction,
    painted: HashMap<Position, Color>,
}

impl PaintingRobot {
    pub fn new(program: Vec<i64>) -> PaintingRobot {
        PaintingRobot {
            computer: IntComputer::new(program),
            pos: Position { x: 0, y: 0 },
            dir: Direction::Up,
            painted: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.computer.run();
            match self.computer.state {
                ProgramState::Finished => break,
                ProgramState::Running => continue,
                ProgramState::WaitingForInput => {
                    self.consume_output();
                    self.computer.input.push_back(self.read_color() as i64);
                }
            }
        }
    }

    pub fn turn_left(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Left,
            Direction::Down => self.dir = Direction::Right,
            Direction::Left => self.dir = Direction::Down,
            Direction::Right => self.dir = Direction::Up,
        }
    }

    pub fn turn_right(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Right,
            Direction::Down => self.dir = Direction::Left,
            Direction::Left => self.dir = Direction::Up,
            Direction::Right => self.dir = Direction::Down,
        }
    }

    pub fn move_forward(&mut self) {
        match self.dir {
            Direction::Up => self.pos.y += 1,
            Direction::Down => self.pos.y -= 1,
            Direction::Left => self.pos.x -= 1,
            Direction::Right => self.pos.x += 1,
        }
    }

    pub fn consume_output(&mut self) {
        while self.computer.output.len() > 0 {
            let col = match self.computer.output.pop_front() {
                Some(0) => Color::Black,
                Some(1) => Color::White,
                _ => panic!("Unexpected Output"),
            };
            self.paint(col);
            match self.computer.output.pop_front() {
                Some(0) => self.turn_left(),
                Some(1) => self.turn_right(),
                _ => panic!("Unexpected Output"),
            }
            self.move_forward();
        }
    }

    pub fn paint(&mut self, c: Color) {
        self.painted.insert(self.pos.clone(), c);
    }

    pub fn read_color(&self) -> i64 {
        match self.painted.get(&self.pos) {
            Some(c) => (*c).clone() as i64,
            None => Color::Black as i64,
        }
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

    let mut robot = PaintingRobot::new(program.clone());
    robot.run();
    println!("Solution Part 1: {:?}", robot.painted.len());

    let mut robot = PaintingRobot::new(program);
    robot.painted.insert(Position { x: 0, y: 0 }, Color::White);
    robot.run();

    let x_min = robot.painted.keys().map(|p| p.x).min().unwrap();
    let x_max = robot.painted.keys().map(|p| p.x).max().unwrap();
    let y_min = robot.painted.keys().map(|p| p.y).min().unwrap();
    let y_max = robot.painted.keys().map(|p| p.y).max().unwrap();

    println!("Solution Part 2:");
    for y in (y_min..=y_max).rev() {
        let mut line = String::new();
        for x in x_min..=x_max {
            let c = match robot.painted.get(&Position { x: x, y: y }) {
                Some(Color::White) => '█',
                _ => ' ',
            };
            line.push(c);
        }
        println!("{}", line);
    }
}

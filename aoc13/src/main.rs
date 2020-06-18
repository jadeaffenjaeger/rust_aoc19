use intcomputer::*;
use display::*;

use std::env;
use std::fs;

use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

const WIDTH: usize = 44;
const HEIGHT: usize = 20;
const SCALE: usize = 25;

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive)]
#[repr(i64)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

struct Arcade<'a> {
    screen: [Tile; WIDTH * HEIGHT],
    display: &'a mut Display,
    computer: IntComputer,
    score: u32,
}

impl<'a> Arcade<'a> {
    pub fn new(program: Vec<i64>, display: &'a mut Display) -> Self {
        Self {
            display: display,
            screen: [Tile::Empty; WIDTH * HEIGHT],
            computer: IntComputer::new(program),
            score: 0,
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
                    break;
                }
            }
        }
    }

    pub fn left(&mut self) {
        self.computer.input.push_back(-1);
    }

    pub fn right(&mut self) {
        self.computer.input.push_back(1);
    }

    pub fn neutral(&mut self) {
        self.computer.input.push_back(0);
    }

    pub fn consume_output(&mut self) {
        while self.computer.output.len() > 0 {
            let x = self.computer.output.pop_front().unwrap();
            let y = self.computer.output.pop_front().unwrap();
            if x == -1 && y == 0 {
                let score = self.computer.output.pop_front().unwrap() as u32;
                self.score = score;
            } else {
                let tile = Tile::try_from(self.computer.output.pop_front().unwrap()).unwrap();
                self.screen[(x as usize) + (y as usize) * WIDTH] = tile;
                self.display.set_pixel(x as usize, y as usize, tile as u32);
            }
        }
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();

    let mut display= Display::new(WIDTH, HEIGHT, SCALE, "Aoc Day 13");

    let mut program: Vec<i64> = contents
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

        let mut arcade = Arcade::new(program.clone(), &mut display);
        arcade.run();
        arcade.consume_output();
        let blocktiles = arcade.screen.iter().filter(|&&t| t == Tile::Block).count();
        println!("Solution Part 1: {:?}", blocktiles);

    program[0] = 2;
    let mut arcade = Arcade::new(program, &mut display);

    loop {
        arcade.run();
        if arcade.computer.state == ProgramState::Finished || arcade.display.update() == false {
            break;
        }

        // Control paddle movement according to ball position
        let get_pos = |tiletype| {
            arcade
                .screen
                .iter()
                .enumerate()
                .filter(|(_, &t)| t == tiletype)
                .next()
                .unwrap()
        };
        let ball_x = get_pos(Tile::Ball).0 % WIDTH;
        let paddle_x = get_pos(Tile::Paddle).0 % WIDTH;
        if ball_x < paddle_x {
            arcade.left()
        }
        if ball_x > paddle_x {
            arcade.right()
        }
        if ball_x == paddle_x {
            arcade.neutral()
        }
    }

    // Update final score
    arcade.consume_output();
    println!("Solution Part 2: {}", arcade.score);
    Ok(())
}

use intcomputer::*;

use std::env;
use std::fs;

use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

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

struct Arcade {
    screen: [Tile; WIDTH * HEIGHT],
    computer: IntComputer,
    score: u32,
}

impl Arcade {
    pub fn new(program: Vec<i64>) -> Self {
        Self {
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
            }
        }
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();

    let mut program: Vec<i64> = contents
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut arcade = Arcade::new(program.clone());
    arcade.run();
    arcade.consume_output();
    let blocktiles = arcade.screen.iter().filter(|&&t| t == Tile::Block).count();
    println!("Solution Part 1: {:?}", blocktiles);

    program[0] = 2;
    let mut arcade = Arcade::new(program);
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(
            "Rust AoC 2019 Day 13",
            (WIDTH * SCALE) as u32,
            (HEIGHT * SCALE) as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        arcade.run();
        if arcade.computer.state == ProgramState::Finished {
            break 'running;
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

        canvas.set_draw_color(Color::RGB(10, 10, 10));
        canvas.clear();

        for (xy, tile) in arcade.screen.iter().enumerate() {
            if *tile == Tile::Empty {
                continue;
            }

            match *tile {
                Tile::Empty => continue,
                _ => canvas.set_draw_color(Color::RGB(20, 220, 20)),
            }
            let x = ((xy % WIDTH) * SCALE) as i32;
            let y = ((xy / WIDTH) * SCALE) as i32;
            let _ = canvas.fill_rect(Rect::new(x, y, SCALE as u32, SCALE as u32));
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    // Update final score
    arcade.consume_output();
    println!("Solution Part 2: {}", arcade.score);
    Ok(())
}

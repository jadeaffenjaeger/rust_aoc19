use intcomputer::IntComputer;
use std::env;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    West,
    East,
    North,
    South,
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Oxygen,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    pub fn all_neighbors(&self) -> Vec<Pos> {
        vec![
            self.neighbor(Direction::North),
            self.neighbor(Direction::South),
            self.neighbor(Direction::East),
            self.neighbor(Direction::West),
        ]
    }

    pub fn unknown_neighbors(&self, walls: &Vec<Pos>, empty: &Vec<Pos>) -> Vec<Pos> {
        self.all_neighbors()
            .into_iter()
            .filter(|&t| walls.iter().all(|&s| t != s))
            .filter(|&t| empty.iter().all(|&s| t != s))
            .collect()
    }

    pub fn neighbor(&self, dir: Direction) -> Pos {
        let Pos(x, y) = *self;
        match dir {
            Direction::North => Pos(x + 1, y),
            Direction::South => Pos(x - 1, y),
            Direction::West => Pos(x, y - 1),
            Direction::East => Pos(x, y + 1),
        }
    }

    pub fn dist(&self, other: &Pos) -> u32 {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as u32
    }

    pub fn direction(&self, other: &Pos) -> Result<Direction, ()> {
        if self.dist(other) != 1 {
            return Err(());
        }
        if self.0 < other.0 {
            return Ok(Direction::West);
        }
        if self.0 > other.0 {
            return Ok(Direction::East);
        }
        if self.1 < other.1 {
            return Ok(Direction::North);
        }
        if self.1 > other.1 {
            return Ok(Direction::South);
        }
        return Err(());
    }
}

#[derive(Debug)]
struct Robot {
    position: Pos,
    computer: IntComputer,
    empty: Vec<Pos>,
    unknown: Vec<Pos>,
    walls: Vec<Pos>,
}

impl Robot {
    pub fn new(program: Vec<i64>) -> Self {
        Self {
            position: Pos(0, 0),
            computer: IntComputer::new(program),
            empty: vec![Pos(0, 0)],
            unknown: vec![Pos(-1, 0), Pos(1, 0), Pos(0, 1), Pos(0, -1)],
            walls: vec![],
        }
    }

    pub fn step(&mut self, dir: Direction) -> Tile {
        match dir {
            Direction::North => self.computer.input.push_back(1),
            Direction::South => self.computer.input.push_back(2),
            Direction::West => self.computer.input.push_back(3),
            Direction::East => self.computer.input.push_back(4),
        }
        self.computer.run();
        let ret = match self.computer.output.pop_front().unwrap() {
            0 => Tile::Wall,
            1 => Tile::Empty,
            2 => Tile::Oxygen,
            _ => panic! {"Unknown Tile type"},
        };

        let dst = self.position.neighbor(dir);

        match ret {
            Tile::Wall => self.walls.push(dst),
            Tile::Oxygen => self.position = dst,
            Tile::Empty => {
                self.position = dst;
                self.empty.push(dst);
                self.unknown
                    .append(&mut self.position.unknown_neighbors(&self.walls, &self.empty))
            }
        }
        ret
    }

    fn update_unknowns(&mut self) {}
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

    let mut comp = IntComputer::new(program.clone());
    comp.input.push_back(1);
    comp.run();
    println!("Solution Part 1: {:?}", comp.output);

    let mut comp = IntComputer::new(program);
    // comp.input.push_back(2);
    // comp.run();
    println!("Solution Part 2: {:?}", 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {}
}

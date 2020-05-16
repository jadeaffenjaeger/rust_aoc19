use intcomputer::{IntComputer, ProgramState};
use std::env;
use std::fs;

use itertools::Itertools;

fn run_pipeline(program: &Vec<i32>, sequence: Vec<i32>) -> i32 {
    let mut io = 0;
    for i in 0..=4 {
        let mut comp = IntComputer::new(program.clone());
        comp.input.push_back(sequence[i]);
        comp.input.push_back(io);
        comp.run();
        io = comp.output[0]
    }
    io
}

fn run_pipeline_feedback(program: &Vec<i32>, sequence: Vec<i32>) -> i32 {
    let mut computers: Vec<IntComputer> = vec![];
    for i in 0..=4 {
        let mut comp = IntComputer::new(program.clone());
        comp.input.push_back(sequence[i]);
        computers.push(comp);
    }

    computers[0].input.push_back(0);

    let mut idx = 0;
    loop {
        let c = &mut computers[idx];
        c.run();
        let io = c.output.pop_front().unwrap();

        if computers.iter().all(|c| c.state == ProgramState::Finished) {
            return io;
        }

        idx += 1;
        idx %= 5;

        let c = &mut computers[idx];
        c.input.push_back(io);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();

    let program: Vec<i32> = contents
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let max_thrust = (0..=4)
        .into_iter()
        .permutations(5)
        .map(|perm| run_pipeline(&program, perm))
        .max()
        .unwrap();
    println!("Solution Part 1: {:?}", max_thrust);

    let max_thrust = (5..=9)
        .into_iter()
        .permutations(5)
        .map(|perm| run_pipeline_feedback(&program, perm))
        .max()
        .unwrap();
    println!("Solution Part 2: {:?}", max_thrust);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(run_pipeline(&program, vec![4, 3, 2, 1, 0]), 43210);
    }

    #[test]
    fn test_p2() {
        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(run_pipeline(&program, vec![0, 1, 2, 3, 4]), 54321);
    }

    #[test]
    fn test_p3() {
        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(run_pipeline(&program, vec![1, 0, 4, 3, 2]), 65210);
    }

    #[test]
    fn test_p4() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(
            run_pipeline_feedback(&program, vec![9, 8, 7, 6, 5]),
            139629729
        );
    }

    #[test]
    fn test_p5() {
        let program = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(run_pipeline_feedback(&program, vec![9, 7, 8, 5, 6]), 18216);
    }
}

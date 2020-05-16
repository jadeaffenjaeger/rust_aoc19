use intcomputer;
use std::env;
use std::fs;

fn run_program(program: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut prog = program.clone();
    prog[1] = noun;
    prog[2] = verb;
    let mut comp = intcomputer::IntComputer::new(prog);
    comp.run();
    comp.program[0]
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

    println!("Solution Part 1: {}", run_program(&program, 12, 2));

    for noun in 0..=99 {
        for verb in 0..=100 {
            let result = run_program(&program, noun, verb);
            if result == 19690720 {
                println!("Solution Part 2: {}{}", noun, verb);
                break;
            }
        }
    }
}

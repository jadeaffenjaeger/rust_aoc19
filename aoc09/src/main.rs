use intcomputer::IntComputer;
use std::env;
use std::fs;

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
    comp.input.push_back(2);
    comp.run();
    println!("Solution Part 2: {:?}", comp.output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {}
}

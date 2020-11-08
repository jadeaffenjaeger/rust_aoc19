use intcomputer::{IntComputer, ProgramState};
use std::collections::HashSet;
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

    let mut computer = IntComputer::new(program);
    while computer.state == ProgramState::Running {
        computer.run();
    }

    let mut scaffolding: HashSet<(usize, usize)> = HashSet::new();
    let output_slice = computer.output.as_slices().0;
    
    // Split into lines
    let rows: Vec<&[i64]> = output_slice.split(|&x| x == 10).collect();
    let num_rows = rows.len();
    let num_cols = rows[0].len();

    for (y, row) in rows.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 35 {
                scaffolding.insert((x, y));
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {}
}

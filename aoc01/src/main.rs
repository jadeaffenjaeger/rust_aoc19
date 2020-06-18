use std::env;
use std::fs;

fn fuel_required_simple(mass: i32) -> i32 {
    0.max((mass / 3) - 2)
}

fn fuel_required_complex(mass: i32) -> i32 {
    let fuel = fuel_required_simple(mass);
    if fuel <= 0 {
        0
    } else {
        fuel + fuel_required_complex(fuel)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let masses: Vec<i32> = contents.lines().map(|x| x.parse().unwrap()).collect();

    let total_fuel_simple: i32 = masses
        .iter()
        .fold(0, |acc, x| acc + fuel_required_simple(*x));
    println!("Solution Part 1: {}", total_fuel_simple);

    let total_fuel_complex: i32 = masses
        .iter()
        .fold(0, |acc, x| acc + fuel_required_complex(*x));
    println!("Solution Part 2: {}", total_fuel_complex);
}

#[test]
fn test_fuel_simple() {
    assert_eq!(fuel_required_simple(12), 2);
    assert_eq!(fuel_required_simple(14), 2);
    assert_eq!(fuel_required_simple(1969), 654);
    assert_eq!(fuel_required_simple(100756), 33583);
}

#[test]
fn test_fuel_complex() {
    assert_eq!(fuel_required_complex(12), 2);
    assert_eq!(fuel_required_complex(14), 2);
    assert_eq!(fuel_required_complex(1969), 966);
    assert_eq!(fuel_required_complex(100756), 50346);
}

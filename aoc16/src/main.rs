use std::char;
use std::env;
use std::fs;
use std::iter;

fn generate_pattern(len: usize, pos: usize) -> Vec<i32> {
    let zeroes = iter::repeat(0).take(pos);
    let pos_ones = iter::repeat(1).take(pos);
    let neg_ones = iter::repeat(-1).take(pos);

    zeroes
        .clone()
        .chain(pos_ones)
        .chain(zeroes)
        .chain(neg_ones)
        .cycle()
        .skip(1)
        .take(len)
        .collect()
}

fn fft(input: &Vec<i32>) -> Vec<i32> {
    let fft_single = |pos| {
        generate_pattern(input.len(), pos)
            .iter()
            .zip(input.iter())
            .filter(|(&p, _)| p != 0)
            .fold(0, |acc, (&p, &i)| if p < 0 { acc - i } else { acc + i })
            .abs()
            % 10
    };
    (1..=input.len())
        .into_iter()
        .map(|pos| fft_single(pos))
        .collect()
}

fn phases(input: &mut Vec<i32>, num: usize) {
    for _ in 0..num {
        *input = fft(&input);
    }
}

fn num_to_vec(numstr: &str) -> Vec<i32> {
    numstr
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect()
}

fn part_two(input: &Vec<i32>) -> Vec<i32> {
    let cumsum_rev: Vec<i32> = input
        .iter()
        .rev()
        .scan(0, |state, x| {
            *state = (*state + x) % 10;
            Some(*state)
        })
        .collect();
    cumsum_rev.into_iter().rev().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
    let input = num_to_vec(&contents);

    let mut num = input.clone();
    phases(&mut num, 100);

    let to_str = |num: Vec<i32>| -> String {
        num[0..8]
            .iter()
            .map(|&x| char::from_digit(x as u32, 10).unwrap())
            .collect()
    };
    println!("Solution Part 1: {}", to_str(num));

    let offset = input[0..7].iter().fold(0, |acc, &x| acc * 10 + x) as usize;
    let size = input.len() * 10000;
    let mut num_large: Vec<_> = input.into_iter().cycle().take(size).skip(offset).collect();
    for _ in 0..100 {
        num_large = part_two(&num_large);
    }
    println!("Solution Part 2: {}", to_str(num_large));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        assert_eq!(num_to_vec(&"12345678"), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
    #[test]
    fn test_fft() {
        assert_eq!(
            fft(&vec![1, 2, 3, 4, 5, 6, 7, 8]),
            vec![4, 8, 2, 2, 6, 1, 5, 8]
        );
    }
    #[test]
    fn test_phases() {
        let mut num = num_to_vec(&"80871224585914546619083218645595");
        phases(&mut num, 100);
        assert_eq!(num[0..8], [2, 4, 1, 7, 6, 1, 7, 6]);
    }
    #[test]
    fn test_part_two() {
        let num = num_to_vec(&"54321");
        assert_eq!(part_two(&num), [5, 0, 6, 3, 1]);
    }
}

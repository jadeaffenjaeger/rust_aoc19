use float_cmp::*;
use std::env;
use std::fs;

// Parse a star field into a list of coordinates
fn star_coords(field: String) -> Vec<(f32, f32)> {
    let mut coords: Vec<(f32, f32)> = vec![];
    for (row_num, line) in field.lines().enumerate() {
        for (col_num, val) in line.chars().enumerate() {
            if val == '#' {
                coords.push((row_num as f32, col_num as f32));
            }
        }
    }
    coords
}

fn count_visible(stars: &Vec<(f32, f32)>, s: &(f32, f32)) -> u32 {
    // let mut vis: u32 = stars.len() as u32 - 1;
    let is_self = |p: &(f32, f32)| approx_eq!(f32, p.0, 0.0) && approx_eq!(f32, p.1, 0.0);
    let mut vis: Vec<_> = stars
        .iter()
        .map(|s1| (s1.0 - s.0, s1.1 - s.1))
        .filter(|p| !is_self(p))
        .map(|p| p.0.atan2(p.1))
        .collect();
    vis.sort_by(|a,b| a.partial_cmp(b).unwrap());
    vis.dedup_by(|a,b| approx_eq!(f32, *a,*b));
    vis.len() as u32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
    let stars = star_coords(contents);
    let max_visible = stars.iter().map(|s| count_visible(&stars, s)).max().unwrap();
    println!("Solution Part 1: {}", max_visible);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let vec = vec![0, 1, 1, 2, 1, 0];
        assert_eq!(count_digits(&vec, 0), 2);
        assert_eq!(count_digits(&vec, 1), 3);
        assert_eq!(count_digits(&vec, 2), 1);
        assert_eq!(count_digits(&vec[0..2], 1), 1);
    }
}

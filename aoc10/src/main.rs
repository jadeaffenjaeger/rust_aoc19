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

// Convert all stars into angle and distance representation from a given grid location 's'
fn get_angles_dists(stars: &Vec<(f32, f32)>, s: &(f32, f32)) -> Vec<(f32, f32)> {
    let is_self = |p: &(f32, f32)| approx_eq!(f32, p.0, 0.0) && approx_eq!(f32, p.1, 0.0);
    stars
        .iter()
        .map(|s1| (s1.0 - s.0, s1.1 - s.1))
        .filter(|p| !is_self(p))
        .map(|p| (p.1.atan2(p.0), (p.0.powi(2) + p.1.powi(2)).sqrt()))
        .collect()
}

// Retrieve number of unique angles (aka visible stars) from a given grid location
fn count_visible(stars: &Vec<(f32, f32)>, s: &(f32, f32)) -> u32 {
    let mut angles: Vec<_> = get_angles_dists(stars, s).iter().map(|p| p.0).collect();
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    angles.dedup_by(|a, b| approx_eq!(f32, *a, *b));
    angles.len() as u32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
    let stars = star_coords(contents);
    let max_visible = stars
        .iter()
        .map(|s| (s, count_visible(&stars, s)))
        .max_by_key(|s| s.1)
        .unwrap();
    println!("Solution Part 1: {}", max_visible.1);

    let mut remaining: Vec<_> = get_angles_dists(&stars, &max_visible.0);

    // Flip angles for clockwise rotation
    remaining = remaining.iter().map(|x| (-x.0, x.1)).collect();

    // Sort by distance first, then angle to encounter the nearest star at any angle first
    remaining.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    remaining.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Remove vaporized stars one by one
    let mut i = 0;
    loop {
        // Vaporize but remember its angle
        let angle = remaining[i].0;
        remaining.remove(i);
        i %= remaining.len();

        if stars.len() - remaining.len() == 200 {
            print_solution(remaining, max_visible.0, i);
            break;
        }

        // Skip all further stars at the current angle
        while approx_eq!(f32, remaining[i].0, angle) {
            i += 1;
            // Round complete, start next round
            if i >= remaining.len() {
                i = 0;
                break;
            }
        }
    }
}

fn print_solution(remaining: Vec<(f32, f32)>, location: &(f32, f32), idx: usize) {
    // Recover cartesian offset from angle and distance representation
    let offset = (
        ((-remaining[idx].0).cos() * remaining[idx].1).round(),
        ((-remaining[idx].0).sin() * remaining[idx].1).round(),
    );
    println!(
        "Solution Part 2: {}",
        (location.1 + offset.1) * 100.0 + location.0 + offset.0
    );
}

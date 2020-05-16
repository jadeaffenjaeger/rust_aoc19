use std::collections::HashSet;
use std::env;
use std::fs;

struct Grid {
    points: HashSet<(i32, i32)>,
    pt_queue: Vec<(i32, i32, i32)>,
    head: (i32, i32),
    steps: i32,
}

impl Grid {
    fn new(raw_tokens: &str) -> Grid {
        let tokens: Vec<&str> = raw_tokens.split(',').collect();

        let mut capacity: usize = 0;
        for token in &tokens {
            let amt: usize = token[1..token.len()].parse().unwrap();
            capacity += amt;
        }

        let mut g = Grid {
            points: HashSet::with_capacity(capacity),
            pt_queue: Vec::with_capacity(capacity),
            head: (0, 0),
            steps: 0,
        };

        for token in tokens {
            let dir: &str = &token[0..1];
            let amt: i32 = token[1..token.len()].parse().unwrap();
            Grid::add_points(&mut g, dir, amt);
        }
        g
    }

    fn add_points(g: &mut Grid, dir: &str, amt: i32) {
        match dir {
            "L" => {
                for i in (-amt..0).rev() {
                    g.steps += 1;
                    g.points.insert((g.head.0 + i, g.head.1));
                    g.pt_queue.push((g.head.0 + i, g.head.1, g.steps));
                }
                g.head = (g.head.0 - amt, g.head.1);
            }
            "R" => {
                for i in 1..=amt {
                    g.steps += 1;
                    g.points.insert((g.head.0 + i, g.head.1));
                    g.pt_queue.push((g.head.0 + i, g.head.1, g.steps));
                }
                g.head = (g.head.0 + amt, g.head.1);
            }
            "U" => {
                for i in 1..=amt {
                    g.steps += 1;
                    g.points.insert((g.head.0, g.head.1 + i));
                    g.pt_queue.push((g.head.0, g.head.1 + i, g.steps));
                }
                g.head = (g.head.0, g.head.1 + amt);
            }
            "D" => {
                for i in (-amt..0).rev() {
                    g.steps += 1;
                    g.points.insert((g.head.0, g.head.1 + i));
                    g.pt_queue.push((g.head.0, g.head.1 + i, g.steps));
                }
                g.head = (g.head.0, g.head.1 - amt);
            }
            _ => {}
        }
    }

    fn get_overlap(&self, g2: &Grid) -> Vec<(i32, i32)> {
        self.points.intersection(&g2.points).cloned().collect()
    }

    fn smallest_distance(points: &Vec<(i32, i32)>) -> i32 {
        let mut dists: Vec<i32> = points.iter().map(|x| x.0.abs() + x.1.abs()).collect();
        dists.sort();
        dists[0]
    }

    fn smallest_delay(points: &Vec<(i32, i32)>, g1: &Grid, g2: &Grid) -> i32 {
        let mut delays = vec![];
        for p in points {
            let delay1: i32 = g1
                .pt_queue
                .iter()
                .filter(|x| x.0 == p.0 && x.1 == p.1)
                .map(|x| x.2)
                .next()
                .unwrap();
            let delay2: i32 = g2
                .pt_queue
                .iter()
                .filter(|x| x.0 == p.0 && x.1 == p.1)
                .map(|x| x.2)
                .next()
                .unwrap();
            delays.push(delay1 + delay2);
        }
        delays.sort();
        delays[0]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = contents.lines().collect();

    let g1 = Grid::new(lines[0]);
    let g2 = Grid::new(lines[1]);

    let overlap = g1.get_overlap(&g2);
    let dist = Grid::smallest_distance(&overlap);
    let delay = Grid::smallest_delay(&overlap, &g1, &g2);
    println!("Solution Part 1: {}", dist);
    println!("Solution Part 2: {}", delay);
}

#[test]
fn test_up() {
    let grid = Grid::new("U2");
    assert!(grid.points.contains(&(0, 1)));
    assert!(grid.points.contains(&(0, 2)));
}

#[test]
fn test_down() {
    let grid = Grid::new("D2");
    assert!(grid.points.contains(&(0, -1)));
    assert!(grid.points.contains(&(0, -2)));
}

#[test]
fn test_left() {
    let grid = Grid::new("L2");
    assert!(grid.points.contains(&(-1, 0)));
    assert!(grid.points.contains(&(-2, 0)));
}

#[test]
fn test_right() {
    let grid = Grid::new("R2");
    assert!(grid.points.contains(&(1, 0)));
    assert!(grid.points.contains(&(2, 0)));
}

#[test]
fn test_right_up() {
    let grid = Grid::new("R2,U2");
    println!("{:?}", grid.points);
    assert!(grid.points.contains(&(1, 0)));
    assert!(grid.points.contains(&(2, 0)));
    assert!(grid.points.contains(&(2, 1)));
    assert!(grid.points.contains(&(2, 2)));
}

#[test]
fn test_overlap() {
    let grid1 = Grid::new("R2,U2");
    let grid2 = Grid::new("U2,R2");
    assert_eq!(grid1.get_overlap(&grid2), vec![(2, 2)]);
}

#[test]
fn test_ex1() {
    let grid1 = Grid::new("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let grid2 = Grid::new("U62,R66,U55,R34,D71,R55,D58,R83");
    let overlap = grid1.get_overlap(&grid2);
    assert_eq!(Grid::smallest_distance(&overlap), 159);
    assert_eq!(Grid::smallest_delay(&overlap, &grid1, &grid2), 610);
}

#[test]
fn test_ex2() {
    let grid1 = Grid::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let grid2 = Grid::new("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    let overlap = grid1.get_overlap(&grid2);
    assert_eq!(Grid::smallest_distance(&overlap), 135);
    assert_eq!(Grid::smallest_delay(&overlap, &grid1, &grid2), 410);
}

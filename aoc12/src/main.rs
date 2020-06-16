use num_integer::Integer;
use std::env;
use std::fs;

#[derive(Debug, PartialEq, Clone, Hash)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    pub fn energy(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Eq for Vec3 {}

#[derive(Debug, PartialEq, Clone, Hash)]
struct Body {
    position: Vec3,
    velocity: Vec3,
}

impl Eq for Body {}

impl Body {
    pub fn new(position: &str) -> Body {
        let position: Vec<_> = position
            .trim_start_matches('<')
            .trim_end_matches('>')
            .split(',')
            .collect();

        let parse_coord = |coord: &str| coord.split('=').nth(1).unwrap().parse::<i64>().unwrap();

        Body {
            position: Vec3 {
                x: parse_coord(position[0]),
                y: parse_coord(position[1]),
                z: parse_coord(position[2]),
            },
            velocity: Vec3 { x: 0, y: 0, z: 0 },
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "px: {:3} py: {:3} pz: {:3} | vx: {:3} vy: {:3} vz: {:3}",
            self.position.x,
            self.position.y,
            self.position.z,
            self.velocity.x,
            self.velocity.y,
            self.velocity.z
        )
    }

    pub fn interact(&mut self, other: &Body) {
        let d_gravity = |p1: i64, p2: i64| {
            if p1 < p2 {
                return 1;
            }
            if p1 > p2 {
                return -1;
            }
            0
        };
        self.velocity.x += d_gravity(self.position.x, other.position.x);
        self.velocity.y += d_gravity(self.position.y, other.position.y);
        self.velocity.z += d_gravity(self.position.z, other.position.z);
    }

    pub fn update_position(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    pub fn energy(&self) -> i64 {
        self.position.energy() * self.velocity.energy()
    }
}

fn update_bodies(bodies: &mut Vec<Body>) {
    for i in 1..bodies.len() {
        let (left, right) = bodies.split_at_mut(i);
        for b2 in left {
            right[0].interact(&b2);
            b2.interact(&right[0]);
        }
    }

    for b in bodies {
        b.update_position();
    }
}

fn print_bodies(bodies: &Vec<Body>) {
    for b in bodies {
        println!("{}", b.to_string());
    }
    println!("====");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();

    let mut bodies1: Vec<_> = contents.lines().map(|l| Body::new(l)).collect();
    let mut bodies2 = bodies1.clone();

    for _ in 0..1000 {
        update_bodies(&mut bodies1);
    }

    let total_energy = bodies1.iter().fold(0, |acc, b| acc + b.energy());
    println!("Solution Part 1: {:?}", total_energy);

    // I got stuck on the second part, so I looked for some help on the internet.
    // Two important bits of insight that I probably wouldn't have come up with myself:
    //
    // 1. The first reoccuring state will always be identical to the initial state (so the loop of states will always be ABCDABCD, never ABCDCDCDCD.
    // This means that we only need to compare to the initial state, not every state encountered this far.
    // This follows from the fact that the forward transformation function is inversible, so every state can only be reached from exactly one other state.
    //
    // 2. The transformations for x, y and z are independent. This means that each of these have individual cycles. The global cycle length will then be the LCM of the cycles for x, y and z individually.

    // Split bodies up into their x,y and z positions and velocities
    let get_axis = |bodies: &Vec<Body>, f: fn(&Body) -> (i64, i64)| bodies.iter().map(f).collect();
    let get_x = |b: &Body| (b.position.x, b.velocity.x);
    let get_y = |b: &Body| (b.position.y, b.velocity.y);
    let get_z = |b: &Body| (b.position.z, b.velocity.z);

    // Keep initial states per axis to see where if we have gone a full round
    let initial_x = get_axis(&bodies2, get_x);
    let initial_y = get_axis(&bodies2, get_y);
    let initial_z = get_axis(&bodies2, get_z);

    let mut i = 0;
    let mut cycles: (u64, u64, u64) = (0, 0, 0);

    loop {
        let current_x = get_axis(&bodies2, get_x);
        let current_y = get_axis(&bodies2, get_y);
        let current_z = get_axis(&bodies2, get_z);

        // Compare initial state and current state per axis
        let compare = |b1: &Vec<(i64, i64)>, b2: &Vec<(i64, i64)>| {
            b1.iter().zip(b2.iter()).all(|(a, b)| a == b)
        };

        // Check if we've done a full cycle per axis. If so, store cycle length
        if cycles.0 == 0 && compare(&initial_x, &current_x) {
            cycles.0 = i;
        }
        if cycles.1 == 0 && compare(&initial_y, &current_y) {
            cycles.1 = i;
        }
        if cycles.2 == 0 && compare(&initial_z, &current_z) {
            cycles.2 = i;
        }

        // All axes have gone at least one full cycle. Compute LCM from results
        if cycles.0 != 0 && cycles.1 != 0 && cycles.2 != 0 {
            let ans = (cycles.0).lcm(&cycles.1).lcm(&cycles.2);
            println!("Solution Part 2: {:?}", ans);
            break;
        }

        update_bodies(&mut bodies2);
        i += 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let position = "<x=-1, y=0, z=2>";
        let body = Body::new(&position);
        assert_eq!(body.position, Vec3 { x: -1, y: 0, z: 2 });
        assert_eq!(body.velocity, Vec3 { x: 0, y: 0, z: 0 });
    }

    #[test]
    fn test_interact() {
        let mut b1 = Body::new("<x=3,y=0,z=5>");
        let b2 = Body::new("<x=5,y=0,z=3>");
        b1.interact(&b2);
        assert_eq!(b1.position, Vec3 { x: 3, y: 0, z: 5 });
        assert_eq!(b1.velocity, Vec3 { x: 1, y: 0, z: -1 });
    }

    #[test]
    fn test_update() {
        let mut b1 = Body::new("<x=1,y=2,z=3>");
        b1.velocity = Vec3 { x: -2, y: 0, z: 3 };
        b1.update_position();
        assert_eq!(b1.position, Vec3 { x: -1, y: 2, z: 6 });
        assert_eq!(b1.velocity, Vec3 { x: -2, y: 0, z: 3 });
    }

    #[test]
    fn test_energy() {
        let mut b1 = Body::new("<x=2,y=1,z=-3>");
        b1.velocity = Vec3 { x: -3, y: -2, z: 1 };
        assert_eq!(b1.energy(), 36);
    }
}

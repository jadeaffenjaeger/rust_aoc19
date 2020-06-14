use std::env;
use std::fs;

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
struct Body {
    position: Vec3,
    velocity: Vec3,
}

impl Body {
    pub fn new(position: &str) -> Body {
        let position: Vec<_> = position
            .trim_start_matches('<')
            .trim_end_matches('>')
            .split(',')
            .collect();

        let parse_coord = |coord: &str| {
            coord
                .split('=')
                .skip(1)
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap()
        };

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
    for i in 1..bodies.len()  {
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

    let mut bodies: Vec<_> = contents.lines().map(|l| Body::new(l)).collect();

    for _ in 0..1000 {
        update_bodies(&mut bodies);
    }
    
    let total_energy = bodies.iter().fold(0, |acc, b| acc + b.energy());
    println!("Solution Part 1: {:?}", total_energy);
    // println!("Solution Part 2: {:?}", 0);
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

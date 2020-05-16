use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();

    let lines: Vec<&str> = contents.lines().collect();
    let map = read_solar_system(lines);

    let orbits = count_orbits(&map, "COM", 0);

    println!("Solution Part 1: {}", orbits);

    // Get paths from root to each element
    let path_you = find_in_tree(&map, "COM", "YOU");
    let path_san = find_in_tree(&map, "COM", "SAN");

    // Count how many leading elements are identical for both paths
    let common_elements = path_san
        .iter()
        .zip(path_you.iter())
        .filter(|(x, y)| x == y)
        .count();

    let solution2 = path_san.len() + path_you.len() - 2 * common_elements;
    println!("Solution Part 2: {}", solution2);
}

fn read_solar_system(lines: Vec<&str>) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::with_capacity(lines.len());
    for line in lines {
        let substr: Vec<&str> = line.trim().split(')').collect();
        if map.contains_key(substr[0]) {
            map.get_mut(substr[0]).unwrap().push(substr[1]);
        } else {
            map.insert(substr[0], vec![substr[1]]);
        }
    }
    map
}

fn find_in_tree<'a>(map: &'a HashMap<&str, Vec<&str>>, cur: &'a str, dst: &str) -> Vec<&'a str> {
    // Reached leaf node -> recursion end
    if !map.contains_key(cur) {
        return vec![];
    }

    let children = map.get(cur).unwrap();

    // Found destination node in children, return ourselves as part of the path
    if children.contains(&dst) {
        return vec![cur];
    // Recursively search through children
    } else {
        let mut out: Vec<&str> = vec![];
        for c in children {
            let mut traversal = find_in_tree(map, c, dst);
            if !traversal.is_empty() {
                out.push(cur);
            }
            out.append(&mut traversal);
        }
        out
    }
}

// Recursively count orbits in tree by calculating the distance from the root
fn count_orbits(map: &HashMap<&str, Vec<&str>>, body: &str, depth: u32) -> u32 {
    if let Some(bodies) = map.get(body) {
        // Tree Node: Result is the cumulated result of all children plus ours
        bodies
            .iter()
            .fold(depth, |acc, &b| acc + count_orbits(map, b, depth + 1))
    } else {
        // Leaf: Result is the distance from the root (aka traversal depth)
        depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L";

    #[test]
    fn test_read() {
        let lines: Vec<&str> = INPUT.lines().collect();
        let map = read_solar_system(lines);
        assert_eq!(map.get("COM"), Some(&vec!["B"]));
        assert_eq!(map.get("B"), Some(&vec!["C", "G"]));
        assert!(!map.contains_key("H"));
    }

    #[test]
    fn test_count() {
        let lines: Vec<&str> = INPUT.lines().collect();
        let map = read_solar_system(lines);
        assert_eq!(count_orbits(&map, "COM", 0), 42);
        assert_eq!(count_orbits(&map, "D", 3), 34);
        assert_eq!(count_orbits(&map, "L", 7), 7);
    }

    #[test]
    fn test_find() {
        let lines: Vec<&str> = INPUT.lines().collect();
        let map = read_solar_system(lines);
        assert_eq!(
            find_in_tree(&map, "COM", "L"),
            vec!["COM", "B", "C", "D", "E", "J", "K"]
        );
    }
}

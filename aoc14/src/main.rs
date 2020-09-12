use std::env;
use std::fs;

use std::collections::HashMap;
use petgraph::graphmap::DiGraphMap;

#[derive(Debug, Clone, Copy)]
struct Reaction {
    qty_in: u32,
    qty_out: u32,
}

// fn read_reaction(&str line) -> (&str, &str, Reaction) {
fn read_reaction<'a>(line: &'a str, graph: &mut DiGraphMap<&'a str, Reaction>) {
    let parts: Vec<&str> = line.split(' ').filter(|x| *x != "=>").collect();

    let edges: Vec<u32> = parts
        .iter()
        .step_by(2)
        .map(|x| x.parse().unwrap())
        .collect();
    let nodes: Vec<_> = parts.iter().skip(1).step_by(2).collect();

    let output = graph.add_node(nodes[nodes.len() - 1]);

    for i in 0..nodes.len() - 1 {
        let input = graph.add_node(nodes[i]);
        let r = Reaction {
            qty_in: edges[i],
            qty_out: edges[edges.len() - 1],
        };
        graph.add_edge(input, output, r);
    }
}

fn produce<'a>(node: &'a str, qty: u32, graph: &DiGraphMap<&'a str, Reaction>, inventory: &mut HashMap<&'a str, u32>) -> u32 {
    if node == "ORE" {
        return qty
    }

    let neighbors: Vec<_> = graph.neighbors_directed(node, petgraph::Incoming).collect();
    let qty_out = graph[(neighbors[0], node)].qty_out;

    let mut total_produced = match inventory.remove(node) {
        Some(n) => n,
        None => 0};
    let mut total_ore = 0;

    while total_produced < qty {
        for n in &neighbors {
            let r = graph[(*n, node)].qty_in;
            total_ore += produce(n, r, graph, inventory);
        }
        total_produced += qty_out
    }
    if total_produced > qty {
        inventory.insert(node, total_produced - qty);
    }
    total_ore
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap().replace(",", "");
    let mut graph = DiGraphMap::<&str, Reaction>::new();
    let mut inventory = HashMap::<&str, u32>::new();

    contents.lines().for_each(|l| read_reaction(l, &mut graph));

    let result1 = produce("FUEL", 1, &graph, &mut inventory);
    println!("Solution Part 1: {:?}", result1);

    Ok(())
}

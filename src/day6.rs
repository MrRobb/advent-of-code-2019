extern crate itertools;
extern crate petgraph;

use itertools::Itertools;
use petgraph::algo::astar;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;
use petgraph::EdgeType;
use petgraph::Undirected;
use std::collections::HashMap;
use std::fs::read_to_string;

fn build_graph<D: EdgeType>(edges: &String) -> GraphMap<&str, (), D> {
	GraphMap::<_, (), D>::from_edges(
		edges
			.lines()
			.map(|edge| edge.split(')').collect_tuple::<(&str, &str)>().unwrap()),
	)
}

fn iter_graph(graph: &GraphMap<&str, (), Directed>, node: &str, cache: &mut HashMap<String, usize>) -> usize {
	match cache.get(node) {
		Some(n) => *n,
		None => {
			let s = graph
				.neighbors(node)
				.map(|child| 1 + iter_graph(graph, child, cache))
				.sum();
			cache.insert(node.to_string(), s);
			s
		},
	}
}

fn calculate_indirect(edges: String) -> usize {
	let graph = build_graph::<Directed>(&edges);
	let mut cache = HashMap::new();
	graph.nodes().map(|node| iter_graph(&graph, node, &mut cache)).sum()
}

fn calculate_path(edges: String) -> usize {
	let graph = build_graph::<Undirected>(&edges);
	let path = astar(&graph, "YOU", |f| f == "SAN", |_| 1, |_| 0).unwrap();
	path.1.len() - 3
}

////////////////////////////////////////
/// MAIN
////////////////////////////////////////

pub fn main() {
	let input = read_to_string("input/day6/input1.txt").unwrap();

	let total = calculate_indirect(input.clone());
	println!("PART 1 -> Total orbits: {}", total);

	let path = calculate_path(input);
	println!("PART 2 -> Orbit changes: {}", path);
}

////////////////////////////////////////
/// TESTS
////////////////////////////////////////

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day6_test1() {
		let edges = vec![
			"COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
		];
		assert_eq!(42, calculate_indirect(edges.join("\n")));
	}

	#[test]
	fn day6_test2() {
		let edges = vec![
			"COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU", "I)SAN",
		];
		assert_eq!(4, calculate_path(edges.join("\n")));
	}
}

extern crate petgraph;

use petgraph::algo::astar;
use petgraph::graphmap::GraphMap;
use petgraph::visit::Dfs;
use petgraph::visit::Walker;
use petgraph::Directed;
use petgraph::EdgeType;
use petgraph::Undirected;
use std::fs::read_to_string;

fn build_graph<D: EdgeType>(edges: Vec<&str>) -> GraphMap<&str, (), D> {
	let mut graph = GraphMap::<_, (), D>::new();
	for edge in edges {
		let from_to: Vec<_> = edge.split(')').collect();
		graph.add_edge(from_to[0], from_to[1], ());
	}
	graph
}

fn calculate_indirect(edges: Vec<&str>) -> usize {
	let graph = build_graph::<Directed>(edges);
	graph
		.nodes()
		.map(|node| Dfs::new(&graph, node).iter(&graph).count() - 1)
		.sum()
}

fn calculate_path(edges: Vec<&str>) -> usize {
	let graph = build_graph::<Undirected>(edges);
	let path = astar(&graph, "YOU", |f| f == "SAN", |_| 1, |_| 0).unwrap();
	path.1.len() - 3
}

////////////////////////////////////////
/// MAIN
////////////////////////////////////////

pub fn main() {
	let input = read_to_string("input/day6/input1.txt").unwrap();
	let edges: Vec<_> = input.lines().collect();

	let total = calculate_indirect(edges.clone());
	println!("PART 1 -> Total orbits: {}", total);

	let path = calculate_path(edges.clone());
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
		assert_eq!(42, calculate_indirect(edges));
	}

	#[test]
	fn day6_test2() {
		let edges = vec![
			"COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU", "I)SAN",
		];
		assert_eq!(4, calculate_path(edges));
	}
}

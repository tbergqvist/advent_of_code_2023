use std::{collections::HashMap, time::Instant};

use itertools::Itertools;

pub fn find_between(nodes: &HashMap<&str, (&str, &str)>, path: &[u8], start: &str, end: &str) -> usize {
	let mut current_node = start;
	for (i, p) in path.iter().cycle().enumerate() {
		current_node = if *p == b'L' {
			nodes[current_node].0 
		} else {
			nodes[current_node].1 
		};

		if current_node == end {
			return i + 1;
		}
	}
	0
}

pub fn a(input: &str) -> usize {
	let mut lines = input.lines();
	let path = lines.next().unwrap().as_bytes();

	let nodes: HashMap<&str, (&str, &str)> = lines
		.skip(1)
		.map(|line| {
			let key = &line[0..3];
			let left_path = &line[7..10];
			let right_path = &line[12..15];
			(key, (left_path, right_path))
		})
		.collect();

	find_between(&nodes, path, "AAA", "ZZZ")
}

pub fn b(input: &str) -> usize {
	let mut lines = input.lines();
	let path = lines.next().unwrap().as_bytes();

	let nodes:HashMap<&str, (&str, &str)> = lines
		.skip(1)
		.map(|line| {
			let key = &line[0..3];
			let left_path = &line[7..10];
			let right_path = &line[12..15];
			(key, (left_path, right_path))
		})
		.collect();
	
	let mut start_nodes = nodes.keys().filter(|node| node.ends_with("A")).map(|n|*n).collect_vec();

	for (i, p) in path.iter().cycle().enumerate() {
		for node in start_nodes.iter_mut() {
			*node = if *p == b'L' {
				nodes[node].0
			} else {
				nodes[node].1
			};
		}

		if start_nodes.iter().all(|n| n.ends_with("Z")) {
			return i + 1;
		}
	}
	0
}

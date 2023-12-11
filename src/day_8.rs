use std::collections::HashMap;

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

fn to_int(st: &str) -> i32 {
	let s:String = st.chars().map(|c| c as u8 - 55).map(|u| u.to_string()).collect();
	s.parse().unwrap()
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
	let path = lines.next().unwrap().replace("L", "\n").replace("R", "(");

	let nodes: HashMap<i32, (i32, i32)> = input.lines()
		.skip(2)
		.map(|line| {
			let key = &line[0..3];
			let left_path = &line[7..10];
			let right_path = &line[12..15];
			(to_int(key), (to_int(left_path), to_int(right_path)))
		})
		.collect();
	
	let mut start_nodes = nodes.keys().filter(|node| node.to_string().ends_with("10")).map(|n|*n).collect_vec();
	let mut cool_nodes: Vec<i32> = vec![0;399999];
	for node in nodes{
		cool_nodes[node.0 as usize + b'\n' as usize] = node.1.0;
		cool_nodes[node.0 as usize + b'(' as usize] = node.1.1;
	}

	for (i, p) in path.bytes().cycle().enumerate() {
		for node in start_nodes.iter_mut() {
			*node = cool_nodes[*node as usize + p as usize];
		}

		if start_nodes.iter().all(|n| *n == 353535 || *n == 332935 || *n == 201935 || *n == 211335 || *n == 333135 || *n == 281235) {
			return i + 1;
		}
	}
	0
}

use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

fn special_char_around<F>(input: &Vec<&str>, x: i32, y: i32, length: i32, is_special: F) -> Vec<(i32, i32)> where F: Fn(char)->bool {
	let mut neighbor_pos: Vec<(i32, i32)> = vec![(x - 1, y), (x + length, y)];
	for i in -1..length + 1 {
		neighbor_pos.push((x + i, y - 1));	
		neighbor_pos.push((x + i, y + 1));	
	}

	let max_x = input[0].len() as i32;
	let max_y = input.len() as i32;
	neighbor_pos.into_iter()
		.filter(|(x, y)| *x >= 0 && *y >= 0 && *x < max_x && *y < max_y)
		.filter(|(x, y)| is_special(input[*y as usize].as_bytes()[*x as usize] as char))
		.collect_vec()
}

pub fn a(input: &str) -> i32 {
	let positions = input.lines().collect_vec();
	let re = Regex::new(r"[0-9]+").unwrap();
	positions.iter()
		.enumerate()
		.flat_map(|(i, row)|re.find_iter(row).map(move |m|(i, m)))
		.filter(|(i, m)| {
			!special_char_around(&positions, m.start() as i32, *i as i32, m.len() as i32, |c| c != '.' && !c.is_digit(10)).is_empty()
		})
		.map(|(_, m)|m.as_str().parse::<i32>().unwrap())
		.sum()
}

pub fn b(input: &str) -> i32 {
	let positions = input.lines().collect_vec();
	let re = Regex::new(r"[0-9]+").unwrap();
	let nums_by_gear_location = positions.iter()
		.enumerate()
		.flat_map(|(i, row)|re.find_iter(row).map(move |m|(i, m)))
		.flat_map(|(i, m)| {
			let num = m.as_str().parse::<i32>().unwrap();
			special_char_around(&positions, m.start() as i32, i as i32, m.len() as i32, |c| c == '*')
				.into_iter()
				.map(move |l|(l, num))
		})
		.fold(HashMap::new(),|mut map, (pos, num)| {
			map.entry(pos).or_insert(vec![]).push(num);
			map
		});

		nums_by_gear_location.into_values()
			.filter(|a| a.len() == 2)
			.map(|a| a.into_iter().reduce(|v, v2| v * v2).unwrap())
			.sum()
}
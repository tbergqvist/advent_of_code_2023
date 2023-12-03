use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

fn is_special(input: &Vec<&str>, x: i32, y: i32) -> bool {
	let line_length = input[0].len() as i32;
	if x >= 0 && y >= 0 && x < line_length && y < input.len() as i32 {
		let bytes = input[y as usize].as_bytes();
		let c = bytes[x as usize] as char;
		return c != '.' && !c.is_digit(10);
	}
	false
}

fn special_char_around(input: &Vec<&str>, x: i32, y: i32, length: i32) -> bool {
	is_special(input, x - 1, y) || 
	is_special(input, x + length, y) || 
	(-1..length + 1 as i32).into_iter().any(|pos| is_special(input, x + pos, y - 1) || is_special(input, x + pos, y + 1))
}

pub fn a(input: &str) -> i32 {
	let positions = input.lines().collect_vec();
	let re = Regex::new(r"[0-9]+").unwrap();
	positions.iter()
		.enumerate()
		.flat_map(|(i, row)|re.find_iter(row).map(move |m|(i, m)))
		.filter(|(i, m)| {
			special_char_around(&positions, m.start() as i32, *i as i32, m.len() as i32)
		})
		.map(|(_, m)|m.as_str().parse::<i32>().unwrap())
		.sum()
}

fn is_special2(input: &Vec<&str>, x: i32, y: i32) -> Option<(i32, i32)> {
	let line_length = input[0].len() as i32;
	if x >= 0 && y >= 0 && x < line_length && y < input.len() as i32 {
		let bytes = input[y as usize].as_bytes();
		let c = bytes[x as usize] as char;
		return if c == '*' {Some((x, y))} else { None };
	}
	None
}

fn special_char_around2(input: &Vec<&str>, x: i32, y: i32, length: i32) -> Vec<(i32, i32)> {
	let mut neighbors: Vec<Option<(i32, i32)>> = vec![];
	neighbors.push(is_special2(input, x - 1, y));
	neighbors.push(is_special2(input, x + length, y));
	neighbors.append(&mut (-1..length + 1 as i32).into_iter().map(|pos| is_special2(input, x + pos, y - 1)).collect_vec());
	neighbors.append(&mut (-1..length + 1 as i32).into_iter().map(|pos| is_special2(input, x + pos, y + 1)).collect_vec());
	neighbors.into_iter()
		.filter_map(|o|o)
		.collect_vec()
}

pub fn b(input: &str) -> i32 {
	let positions = input.lines().collect_vec();
	let re = Regex::new(r"[0-9]+").unwrap();
	let nums_by_gear_location = positions.iter()
		.enumerate()
		.flat_map(|(i, row)|re.find_iter(row).map(move |m|(i, m)))
		.flat_map(|(i, m)| {
			let num = m.as_str().parse::<i32>().unwrap();
			special_char_around2(&positions, m.start() as i32, i as i32, m.len() as i32)
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
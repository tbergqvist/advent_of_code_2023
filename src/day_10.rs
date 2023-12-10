use std::{vec, collections::HashSet};

use itertools::{Itertools, enumerate};

fn get_next_step(steps: (i32, i32), c: u8) -> (i32, i32) {
	match (steps, c) {
			((0, 1), b'-') => (0, 1),
			((0, -1), b'-') => (0, -1),
			((1, 0), b'|') => (1, 0),
			((-1, 0), b'|') => (-1, 0),
			((1, 0), b'L') => (0, 1),
			((0, -1), b'L') => (-1, 0),
			((1, 0), b'J') => (0, -1),
			((0, 1), b'J') => (-1, 0),
			((0, -1), b'F') => (1, 0),
			((-1, 0), b'F') => (0, 1),
			((0, 1), b'7') => (1, 0),
			((-1, 0), b'7') => (0, -1),
			_ => (0, 0)
	}
}

fn find_start(lines: &Vec<Vec<u8>>) -> (i32, i32) {
	for (y, line) in lines.iter().enumerate() {
		for (x, b) in line.iter().enumerate() {
			if *b == b'S' {
				return (y as i32, x as i32);
			}
		}
	}

	(0, 0)
}

fn walk(lines: &Vec<Vec<u8>>, pos: (i32, i32), next_step: (i32, i32)) -> usize {
	let next_pos = (pos.0 + next_step.0, pos.1 + next_step.1);
	let sign = lines[next_pos.0 as usize][next_pos.1 as usize];
	if sign == b'S' {
		return 1;
	}
	let next_next_step = get_next_step(next_step, sign);
	walk(lines, next_pos, next_next_step) + 1
}

pub fn a(input: &str) -> usize {
	let lines = input.lines()
		.map(|line| line.bytes().collect_vec())
		.collect_vec();

		let start_pos = find_start(&lines);
		walk(&lines, start_pos, (1, 0)) / 2
}

pub fn b(input: &str) -> &str {
	let mut lines = input.lines()
	.map(|line| line.bytes().collect_vec())
	.collect_vec();

	
	let mut pos = find_start(&lines);
	let mut next_step = (1, 0);
	let mut all_pos = Vec::new();
	loop {
		let next_pos = (pos.0 + next_step.0, pos.1 + next_step.1);
		let current_sign = lines[next_pos.0 as usize][next_pos.1 as usize];
		next_step = get_next_step(next_step, current_sign);
		pos = next_pos;
		all_pos.push(pos);
		if current_sign == b'S' {
			break;
		}
	}
	for y in 0..140 {
		for x in 0..140 {
			if !all_pos.contains(&(y as i32, x as i32)) {
				lines[y][x] = b'.';
			}
		}
	};
	
	let mut stack = vec![(0 as i32, 0 as i32)];
	while let Some((y, x)) = stack.pop() {
		if y < 0 || x < 0 || y >= 140 || x >= 140 {
			continue;
		}
	
		if lines[y as usize][x as usize] != b'.' || lines[y as usize][x as usize] == b'O' {
			continue;
		}

		lines[y as usize][x as usize] = b'O';

		stack.push((y, x + 1));
		stack.push((y, x - 1));
		stack.push((y + 1, x));
		stack.push((y - 1, x));
		stack.push((y + 1, x + 1));
		stack.push((y - 1, x - 1));
		stack.push((y + 1, x - 1));
		stack.push((y - 1, x + 1));
	}
	
	/*
	for line in &lines {
		for b in line {
			print!("{}", *b as char);
		}
		println!();
	};
	 */

	"Run print and put it in the js code"
	
}

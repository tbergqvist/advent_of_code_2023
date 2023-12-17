use std::collections::HashMap;

use itertools::Itertools;

fn find_positions(level: &[Vec<char>]) -> Vec<(usize, usize)> {
	level.iter()
		.enumerate()
		.flat_map(|(y, row)| {
			row.into_iter()
				.enumerate()
				.filter(|(_, c)| **c == 'O')
				.map(move |(x, _)| (x, y))
		})
		.collect_vec()
}

fn move_north(level: &mut[Vec<char>]) {
	find_positions(level).into_iter().for_each(|(x, y)| {
		let mut new_y = y;
		while new_y > 0 && level[new_y - 1][x] == '.' {
			new_y -= 1;
		}

		level[y][x] = '.';
		level[new_y][x] = 'O';
	});
}

fn move_south(level: &mut[Vec<char>]) {
	find_positions(level).into_iter().rev().for_each(|(x, y)| {
		let mut new_y = y;
		while new_y < (level.len() - 1) && level[new_y + 1][x] == '.' {
			new_y += 1;
		}

		level[y][x] = '.';
		level[new_y][x] = 'O';
	});
}

fn move_west(level: &mut[Vec<char>]) {
	find_positions(level).into_iter().for_each(|(x, y)| {
		let mut new_x = x;
		while new_x > 0 && level[y][new_x - 1] == '.' {
			new_x -= 1;
		}

		level[y][x] = '.';
		level[y][new_x] = 'O';
	});
}

fn move_east(level: &mut[Vec<char>]) {
	find_positions(level).into_iter().rev().for_each(|(x, y)| {
		let mut new_x = x;
		while new_x < (level[0].len() - 1) && level[y][new_x + 1] == '.' {
			new_x += 1;
		}

		level[y][x] = '.';
		level[y][new_x] = 'O';
	});
}

pub fn a(input: &str) -> usize {
	let mut level = input
		.lines()
		.map(|line| line.chars().collect_vec())
		.collect_vec();
	
	move_north(&mut level);

	let length = level.len();
	level.into_iter()
		.enumerate()
		.flat_map(|(y, row)| {
			row.into_iter()
				.filter(|c| *c == 'O')
				.map(move |_| (length - y))
		})
		.sum()
}

pub fn b(input: &str) -> usize {
	let mut level = input
		.lines()
		.map(|line| line.chars().collect_vec())
		.collect_vec();
	
	let mut position_by_level = HashMap::new();

	let mut i = 0;
	while i < 1000000000 {
		move_north(&mut level);
		move_west(&mut level);
		move_south(&mut level);
		move_east(&mut level);

		let level_clone = level.clone();
		if let Some(pos) = position_by_level.get(&level_clone) {
			let diff = i - pos;
			i = 1000000000 - ((1000000000 - i) % diff); 
		}

		position_by_level.insert(level_clone, i);
		i += 1;
	}

	let length = level.len();
	level.into_iter()
		.enumerate()
		.flat_map(|(y, row)| {
			row.into_iter()
				.filter(|c| *c == 'O')
				.map(move |_| (length - y))
		})
		.sum()
}
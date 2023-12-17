use itertools::Itertools;

fn move_north(level: &mut[Vec<char>]) {
	let rocks_pos = level.into_iter()
	.enumerate()
	.flat_map(|(y, row)| {
		row.into_iter()
			.enumerate()
			.filter(|(_, c)| **c == 'O')
			.map(move |(x, _)| (x, y))
	})
	.collect_vec();

	rocks_pos.into_iter().for_each(|(x, y)| {
		let mut new_y = y;
		while new_y > 0 && level[new_y - 1][x] == '.' {
			new_y -= 1;
		}

		level[y][x] = '.';
		level[new_y][x] = 'O';
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
	0
}
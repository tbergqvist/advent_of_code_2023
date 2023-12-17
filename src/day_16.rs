use std::collections::HashSet;

use itertools::Itertools;

pub fn start_stepping(start_pos: (i32, i32), start_velocity: (i32, i32), level: &[Vec<char>], traveled: &mut HashSet<((i32, i32), (i32, i32))>) {
	let mut position = start_pos;
	let mut velocity = start_velocity;

	loop {
		let sign = level[position.1 as usize][position.0 as usize];
		if traveled.contains(&(position, velocity)) {
			return;
		}
		traveled.insert((position, velocity));
		velocity = match (sign, velocity) {
			('/', (x, y)) => (-y, -x),
			('\\', (x, y)) => (y, x),
			('-', (_, y)) if y != 0 => {
				start_stepping(position, (1, 0), level, traveled);
				start_stepping(position, (-1, 0), level, traveled);
				break;
			},
			('|', (x, _)) if x != 0 => {
				start_stepping(position, (0, 1), level, traveled);
				start_stepping(position, (0, -1), level, traveled);
				break;
			},
			_ => velocity
		};

		position = (position.0 + velocity.0, position.1 + velocity.1);

		if position.0 < 0 || position.0 >= level[0].len() as i32 || position.1 < 0 || position.1 >= level.len() as i32 {
			break;
		}
	}
}

pub fn a(input: &str) -> usize {
	let level = input
		.lines()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let mut traveled = HashSet::new();

	start_stepping((0, 0),(1, 0), &level, &mut traveled);
	traveled.into_iter().map(|(pos, _)| pos).unique().count()
}

pub fn b(input: &str) -> usize {
	let level = input
		.lines()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	(0..level.len() as i32).map(|i|((i, 0), (0, 1)))
	.chain((0..level.len() as i32).map(|i|((i, level.len() as i32 - 1), (0, -1))))
	.chain((0..level.len() as i32).map(|i|((0, i), (1, 0))))
	.chain((0..level.len() as i32).map(|i|((level.len() as i32 - 1, i), (-1, 0))))
	.map(|(start_pos, start_velocity)| {
		let mut traveled = HashSet::new();
		start_stepping(start_pos, start_velocity, &level, &mut traveled);
		traveled.into_iter().map(|(pos, _)| pos).unique().count()
	})
	.max()
	.unwrap()
}
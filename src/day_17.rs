use itertools::Itertools;
use pathfinding::prelude::astar;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Step {
	None,
	Up,
	Right,
	Down,
	Left
}

pub fn a(input: &str) -> u32 {
	let level = input
		.lines()
		.map(|line| line.chars().map(|n| n.to_digit(10).unwrap()).collect_vec())
		.collect_vec();

	let start_pos = (0 as i32, 0 as i32);
	let start_val = (start_pos, 1, Step::None);
	let end = (level[0].len() as i32 - 1, level.len() as i32 - 1);
	let size_y = level.len() as i32;
	let size_x = level[0].len() as i32;
	astar(&start_val, |&((x, y), steps_forward, last_step)| {
		let mut paths = Vec::new();

		//right
		if last_step != Step::Left && (last_step != Step::Right || steps_forward <= 2) {
			if last_step == Step::Right {
				paths.push(((x + 1, y), steps_forward + 1, Step::Right));
			} else {
				paths.push(((x + 1, y), 1, Step::Right));
			}
		}
		
		//left
		if last_step != Step::Right && (last_step != Step::Left || steps_forward <= 2) {
			if last_step == Step::Left {
				paths.push(((x - 1, y), steps_forward + 1, Step::Left));
			} else {
				paths.push(((x - 1, y), 1, Step::Left));
			}
		}

		//down
		if last_step != Step::Up && (last_step != Step::Down || steps_forward <= 2) {
			if last_step == Step::Down {
				paths.push(((x, y + 1), steps_forward + 1, Step::Down));
			} else {
				paths.push(((x, y + 1), 1, Step::Down));
			}
		}

	//up
	if last_step != Step::Down && (last_step != Step::Up || steps_forward <= 2) {
		if last_step == Step::Up {
			paths.push(((x, y - 1), steps_forward + 1, Step::Up));
		} else {
			paths.push(((x, y - 1), 1, Step::Up));
		}
	}

		paths.into_iter().filter(|((x, y), _, _)| {
			*x >= 0 && *y >= 0 && *x < size_x && *y < size_y
		}).map(|stuff| {
			let cost = level[stuff.0.1 as usize][stuff.0.0 as usize];
			(stuff, cost)
		})
	}, |&((x, y), _, _)| {
		end.0.abs_diff(x) + end.1.abs_diff(y)
	}, |&(current, _, _)| {
		end == current
	}).unwrap().1
}

pub fn b(input: &str) -> u32 {
	let level = input
		.lines()
		.map(|line| line.trim().chars().map(|n| n.to_digit(10).unwrap()).collect_vec())
		.collect_vec();

	let start_pos = (0 as i32, 0 as i32);
	let start_val = (start_pos, 10, Step::None);
	let end = (level[0].len() as i32 - 1, level.len() as i32 - 1);
	let size_y = level.len() as i32;
	let size_x = level[0].len() as i32;
	astar(&start_val, |&((x, y), steps_forward, last_step)| {
		let mut paths = Vec::new();

		if steps_forward <= 3 {
			if last_step == Step::Right {
				paths.push(((x + 1, y), steps_forward + 1, Step::Right));
			} else if last_step == Step::Left {
				paths.push(((x - 1, y), steps_forward + 1, Step::Left));
			} else if last_step == Step::Down {
				paths.push(((x, y + 1), steps_forward + 1, Step::Down));
			} else if last_step == Step::Up {
				paths.push(((x, y - 1), steps_forward + 1, Step::Up));
			}
		} else {
			//right
			if last_step != Step::Left {
				if last_step == Step::Right && steps_forward <= 9 {
					paths.push(((x + 1, y), steps_forward + 1, Step::Right));
				} else if last_step != Step::Right {
					paths.push(((x + 1, y), 1, Step::Right));
				}
			}

			//left
			if last_step != Step::Right {
				if last_step == Step::Left && steps_forward <= 9 {
					paths.push(((x - 1, y), steps_forward + 1, Step::Left));
				} else if last_step != Step::Left {
					paths.push(((x - 1, y), 1, Step::Left));
				}
			}

			//down
			if last_step != Step::Up {
				if last_step == Step::Down && steps_forward <= 9 {
					paths.push(((x, y + 1), steps_forward + 1, Step::Down));
				} else if last_step != Step::Down {
					paths.push(((x, y + 1), 1, Step::Down));
				}
			}

			//up
			if last_step != Step::Down {
				if last_step == Step::Up && steps_forward <= 9 {
					paths.push(((x, y - 1), steps_forward + 1, Step::Up));
				} else if last_step != Step::Up {
					paths.push(((x, y - 1), 1, Step::Up));
				}
			}
		}

		paths.into_iter().filter(|((x, y), _, _)| {
			*x >= 0 && *y >= 0 && *x < size_x && *y < size_y
		}).map(|stuff| {
			let cost = level[stuff.0.1 as usize][stuff.0.0 as usize];
			(stuff, cost)
		})
	}, |&((x, y), _, _)| {
		end.0.abs_diff(x) + end.1.abs_diff(y)
	}, |&(current, steps, _)| {
		end == current && steps >= 4
	}).unwrap().1
}
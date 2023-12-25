use itertools::Itertools;

pub fn a(input: &str) -> i64 {
	let level: Vec<Vec<char>> = input
		.lines()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	follow(1, 0, &level, 0)
}

fn follow(x: i32, y: i32, level: &Vec<Vec<char>>, steps: i64) -> i64 {
	let surroundings = vec![(-1 as i32, 0 as i32), (1, 0), (0, -1), (0, 1)];
	let mut prev_pos = (x, y);
	let mut current_pos = (x, y);
	for i in steps.. {
		if current_pos.1 >= level.len() as i32 {
			return i - 1;
		}
		let sign = level[current_pos.1 as usize][current_pos.0 as usize];
		match sign {
			'>' => current_pos = (current_pos.0 + 1, current_pos.1),
			'<' => current_pos = (current_pos.0 - 1, current_pos.1),
			'v' => current_pos = (current_pos.0, current_pos.1 + 1),
			'^' => current_pos = (current_pos.0, current_pos.1 - 1),
			'.' => {
				let neighbors = surroundings.iter()
					.filter(|diff| {
						let pos = (current_pos.0 + diff.0, current_pos.1 + diff.1);
						if pos == prev_pos || pos.1 < 0 {
							return false;
						}

						if pos.1 >= level.len() as i32 {
							return true;
						}

						let sign = level[pos.1 as usize][pos.0 as usize];
						match (diff, sign) {
							(_, '#') => false,
							((-1, _), '>') => false,
							((1, _), '<') => false,
							((_, -1), 'v') => false,
							((_, 1), '^') => false,
							_ => true
						}
					})
					.map(|diff| (current_pos.0 + diff.0, current_pos.1 + diff.1))
					.collect_vec();
				
				if neighbors.len() == 1 {
					prev_pos = current_pos;
					current_pos = neighbors[0];
					continue;
				} else {
					return neighbors.into_iter().map(|path| follow(path.0, path.1, level, i + 1)).max().unwrap();
				}
			}
			_ => panic!("waaa"),
		};
	}

	0
}

pub fn b(input: &str) -> i64 {
	let level: Vec<Vec<char>> = input
		.lines()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let visited = vec![false;150 * 151];
	follow2(1, 0, &level, 0, visited)
}

fn follow2(x: i32, y: i32, level: &Vec<Vec<char>>, steps: i64, mut visited: Vec<bool>) -> i64 {
	let surroundings = vec![(-1 as i32, 0 as i32), (1, 0), (0, -1), (0, 1)];
	let mut current_pos = (x, y);
	for i in steps.. {
		visited[current_pos.0 as usize * 150 + current_pos.1 as usize] = true;
		if current_pos.1 >= level.len() as i32 {
			return i - 1;
		}
		let neighbors = surroundings.iter()
			.filter(|diff| {
				let pos = (current_pos.0 + diff.0, current_pos.1 + diff.1);
				if visited[pos.0 as usize * 150 + pos.1 as usize] || pos.1 < 0 {
					return false;
				}

				if pos.1 >= level.len() as i32 {
					return true;
				}
				let sign = level[pos.1 as usize][pos.0 as usize];
				match sign {
					'#' => false,
					_ => true
				}
			}
		)
		.map(|diff| (current_pos.0 + diff.0, current_pos.1 + diff.1))
		.collect_vec();
		
		if neighbors.len() == 1 {
			current_pos = neighbors[0];
			continue;
		} else {
			return neighbors.into_iter().map(|path| follow2(path.0, path.1, level, i + 1, visited.clone())).max().unwrap_or(0);
		}
	}
	0
}
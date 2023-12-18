use itertools::Itertools;

pub fn a(input: &str) -> usize {
	let instructions = input.lines()
		.map(|line| {
			let items: (&str, &str) = line.split(" ").take(2).collect_tuple().unwrap();
			(items.0, items.1.parse::<usize>().unwrap())
		}).collect_vec();

	let mut level = vec![vec!['.';700];300];

	let start_position = (200, 250);
	let mut current_position = start_position;
	for (direction, length) in instructions {
		level[current_position.1][current_position.0] = '#';

		for _ in 0..length {
			level[current_position.1][current_position.0] = '#';
			current_position = match direction {
				"R" => (current_position.0 + 1, current_position.1),
				"L" => (current_position.0 - 1, current_position.1),
				"D" => (current_position.0, current_position.1 + 1),
				"U" => (current_position.0, current_position.1 - 1),
				_ => panic!("weird direction!")
			}
		}
	}

	let mut stack = vec![(200, 265)];
	
	while let Some((x, y)) = stack.pop() {
		
		level[y][x] = 'T';
		stack.extend(vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter().filter(|(x, y)| level[*y][*x] == '.'));
	}

	level.into_iter().flat_map(|l|l).filter(|c| *c == '#' || *c == 'T').count()
}

pub fn b(input: &str) -> i64 {
	let instructions = input.lines()
		.map(|line| {
			let item = line.split(" ").skip(2).next().unwrap();

			let direction = match &item[7..8] {
				"0" => "R",
				"1" => "D",
				"2" => "L",
				"3" => "U",
				_ => panic!("weird direction!")
			};
			(direction, usize::from_str_radix(&item[2..7], 16).unwrap())
		}).collect_vec();
	
	let positions = instructions
	.iter()
	.scan((0 as i64, 0 as i64), |current_position, &(direction, length)| {
		let length = length as i64;
		*current_position = match direction {
			"R" => (current_position.0 + length, current_position.1),
			"L" => (current_position.0 - length, current_position.1),
			"D" => (current_position.0, current_position.1 - length),
			"U" => (current_position.0, current_position.1 + length),
			_ => panic!("weird direction!")
		};
		Some(current_position.clone())
	})
	.collect_vec();

	let area: i64 = positions
		.windows(2)
		.map(|items| {
			let first = items[1];
			let second = items[0];
			(second.0 + first.0) * (second.1 - first.1)
		})
		.sum();

	let border: i64 = instructions.iter().map(|(_, length)| *length as i64).sum();
	(area + border) / 2 + 1
}
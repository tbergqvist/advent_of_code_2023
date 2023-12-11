use itertools::Itertools;

pub fn a(input: &str) -> i32 {
	let mut star_map = input
		.lines()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	for y in (0..star_map.len()).rev() {
		if star_map[y].iter().all(|b| *b == '.') {
			star_map.insert(y, vec!['.'; star_map[0].len()]);
		}
	}

	for x in (0..star_map[0].len()).rev() {
		if (0..star_map.len()).all(|y| star_map[y][x] == '.') {
			star_map.iter_mut().for_each(|v| v.insert(x, '.'));
		}
	}

	let mut planet_positions = Vec::new();
	for y in 0..star_map.len() {
		for x in 0..star_map[y].len() {
			if star_map[y][x] == '#' {
				planet_positions.push((y as i32, x as i32));
			}
		}
	}

	let mut sum = 0;
	for i in 0..planet_positions.len() {
		for j in (i+1)..planet_positions.len() {
			let diff_y = planet_positions[i].0 - planet_positions[j].0;
			let diff_x = planet_positions[i].1 - planet_positions[j].1;
			sum += diff_y.abs() + diff_x.abs();
		}
	}

	sum
}

pub fn b(input: &str) -> i64 {
	let mut star_map = input
		.lines()
		.map(|line| line.chars().map(|c| (c, 1)).collect_vec())
		.collect_vec();
		
	for y in (0..star_map.len()).rev() {
		if star_map[y].iter().all(|b| b.0 == '.') {
			star_map[y].iter_mut().for_each(|(_, cost)| *cost *= 1000000);
		}
	}


	for x in (0..star_map[0].len()).rev() {
		if (0..star_map.len()).all(|y| star_map[y][x].0 == '.') {
			for y in 0..star_map.len() {
				star_map[y][x].1 *= 1000000;
			}
		}
	}

	let mut planet_positions = Vec::new();
	for y in 0..star_map.len() {
		for x in 0..star_map[y].len() {
			if star_map[y][x].0 == '#' {
				planet_positions.push((y, x));
			}
		}
	}

	let mut sum = 0;
	for i in 0..planet_positions.len() {
		for j in (i+1)..planet_positions.len() {
			let diff_y = (planet_positions[i].0 as i32 - planet_positions[j].0 as i32).abs() as usize;
			let diff_x = (planet_positions[i].1 as i32 - planet_positions[j].1 as i32).abs() as usize;
			let min_pos_y = std::cmp::min(planet_positions[i].0, planet_positions[j].0);
			let min_pos_x = std::cmp::min(planet_positions[i].1, planet_positions[j].1);

			sum += (0..diff_y).fold(0, |cost, step| {
				let step_y = min_pos_y + step;
				star_map[step_y][0].1 + cost
			});
			sum +=
			(0..diff_x).fold(0, |cost, step| {
				let step_x = min_pos_x + step;
				star_map[0][step_x].1 + cost
			});
		}
	}
	sum
}

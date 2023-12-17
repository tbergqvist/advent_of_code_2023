use itertools::Itertools;

fn hash(s: &str) -> usize {
	s.bytes().fold(0, |sum, byte| {
		(sum + byte as usize) * 17 % 256
	})
}

pub fn a(input: &str) -> usize {
	input
		.split(",")
		.map(|command| hash(command))
		.sum()
}

pub fn b(input: &str) -> usize {
	let mut boxes = vec![Vec::new();256];
	for command in input.split(",") {
		let command_label: String = command.chars().take_while(|b| *b != '-' && *b != '=').collect();
		let command_nr = hash(&command_label);
		if command.ends_with('-') {
			if let Some((i, _)) = boxes[command_nr].iter().find_position(|(key, _)| *key == command_label) {
				boxes[command_nr].remove(i);
			}
		} else {
			let focal = command.split("=").last().map(|f|f.parse::<usize>().unwrap()).unwrap();
			if let Some(existing) = boxes[command_nr].iter_mut().find(|(key, _)| *key == command_label) {
				existing.1 = focal;
			} else {
				boxes[command_nr].push((command_label.clone(), focal));
			}
		}
	}

	boxes.into_iter()
		.zip(1..)
		.flat_map(|(box_vec, box_nr)| {
			box_vec.into_iter()
				.zip(1..)
				.map(move |((_, focal_point), slot_nr)| {
					box_nr * slot_nr * focal_point
				})
		})
		.sum()
}
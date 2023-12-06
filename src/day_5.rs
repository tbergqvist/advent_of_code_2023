use itertools::Itertools;

struct GroupLine {
	des_start: usize,
	source_start: usize,
	length: usize,
}

pub fn a(input: &str) -> usize {
	let mut iter = input.split("\r\n\r\n");
	let seeds = iter.next();
	let converters = iter.map(|group| {
		group
			.split("\r\n")
			.filter_map(|line| {
				let nums = line.split(" ")
				.filter_map(|item| item.parse().ok())
				.collect_vec();
				if nums.is_empty() {
					return None;
				}
				
				Some(GroupLine {
					des_start: nums[0],
					source_start: nums[1],
					length: nums[2],
				})
			}
			)
			.collect_vec()
	}).collect_vec();
	seeds.unwrap()
		.split(" ")
		.filter_map(|line| line.parse::<usize>().ok())
		.map(|seed| converters.iter().fold(seed, |current_pos, group| {
			let item = group.iter()
				.find(|g| current_pos >= g.source_start && current_pos < g.source_start + g.length)
				.map(|g| current_pos + g.des_start - g.source_start);
			item.unwrap_or(current_pos)
		}))
		.min()
		.unwrap()
}

pub fn b(input: &str) -> usize {
	let mut iter = input.split("\r\n\r\n");
	let seeds_line = iter.next().unwrap();
	let mut converters = iter.map(|group| {
		group
			.split("\r\n")
			.filter_map(|line| {
				let nums = line.split(" ")
				.filter_map(|item| item.parse().ok())
				.collect_vec();

				if nums.is_empty() {
					return None;
				}
				
				Some(GroupLine {
					des_start: nums[0],
					source_start: nums[1],
					length: nums[2],
				})
			}
			)
			.collect_vec()
	}).collect_vec();

	converters.reverse();
	let seeds: Vec<(usize, usize)> = seeds_line[7..]
		.split(" ")
		.filter_map(|line| line.parse::<usize>().ok())
		.chunks(2)
		.into_iter()
		.map(|chunk| chunk.collect_tuple())
		.filter_map(|o|o)
		.collect_vec();

	let min_length = converters.iter().flat_map(|c| c).map(|c|c.length).min().unwrap();
	let mut i = 0;
	loop {
		i += min_length;
		let (pos, min_diff) = converters.iter().fold((i, i), |(current_pos, min), group| {
			group.iter()
				.find(|g| current_pos >= g.des_start && current_pos < g.des_start + g.length)
				.map(|g| (current_pos + g.source_start - g.des_start, min.min(current_pos - g.des_start)))
				.unwrap_or((current_pos, min))
		});

		if seeds.iter().any(|(seed_pos, seed_length)| pos >= *seed_pos && pos <= (seed_pos + seed_length)) {
			return i - min_diff;
		}
	};
}
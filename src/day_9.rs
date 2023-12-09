use itertools::Itertools;

fn find_last(row: &[i32]) -> i32 {
	let new_stuff = row.windows(2).map(|nums| nums[1] - nums[0]).collect_vec();
	if new_stuff.iter().all(|b| *b == new_stuff[0]) {
		new_stuff[0]
	} else {
		find_last(&new_stuff) + new_stuff.last().unwrap()
	}
}

pub fn a(input: &str) -> i32 {
	let rows = input.lines()
		.map(|line| line.split_ascii_whitespace().map(|s|s.parse::<i32>().unwrap()).collect_vec())
		.collect_vec();

	rows.into_iter()
		.map(|r| find_last(&r) + r.last().unwrap())
		.sum()
}

pub fn b(input: &str) -> i32 {
	let rows = input.lines()
	.map(|line| line.split_ascii_whitespace().map(|s|s.parse::<i32>().unwrap()).rev().collect_vec())
	.collect_vec();

rows.into_iter()
	.map(|r| find_last(&r) + r.last().unwrap())
	.sum()
}

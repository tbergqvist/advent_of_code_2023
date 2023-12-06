use itertools::Itertools;

pub fn a(input: &str) -> usize {
	let (times, distances) = input.lines()
		.map(|line| line.split(" ").filter_map(|num|num.parse::<i32>().ok()))
		.take(2)
		.collect_tuple()
		.unwrap();

	times.zip(distances)
		.map(|(time, distance)| {
			(1..time)
				.map(move |speed| (time - speed) * speed)
				.filter(move |new_distance| *new_distance > distance)
				.count()
	})
	.reduce(|b, b2| b * b2)
	.unwrap()
}

pub fn b(input: &str) -> usize {
	let (times, distances) = input.lines()
		.map(|line| line.replace(" ", "").split(":").filter_map(|num|num.parse::<i64>().ok()).collect_vec())
		.take(2)
		.collect_tuple()
		.unwrap();

	times.into_iter().zip(distances).map(|(time, distance)| {
		(1..time)
			.map(move |speed| (time - speed) * speed)
			.filter(move |new_distance| *new_distance > distance)
			.count()
	})
	.reduce(|num, num2| num * num2)
	.unwrap()
}
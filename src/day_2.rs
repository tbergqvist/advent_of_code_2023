use std::collections::HashMap;
use itertools::Itertools;

pub fn a(input: &str) -> i32 {
	input.lines()
		.map(|line| {
			let parsed_line = line.split(":").collect_vec();
			let found = parsed_line[1]
				.split(";")
				.all(|group| {
					let map: HashMap<_, _> = group
						.split(",")
						.map(|die_cast| die_cast.trim().split(" ").take(2).collect_tuple().unwrap())
						.map(|(num_string, color)| (color, num_string.trim().parse::<i32>().unwrap()))
						.collect();
		
					map.get("red").unwrap_or(&0) <= &12 && map.get("green").unwrap_or(&0) <= &13 && map.get("blue").unwrap_or(&0) <= &14
				});

				if found {
					let id: String = parsed_line.first().unwrap().chars().filter(|a| a.is_digit(10)).collect();
					id.parse().unwrap()
				} else {
					0
				}
		})
		.sum()
}

pub fn b(input: &str) -> i32 {
	input.lines()
		.map(|line| {
			let parsed_line = line.split(":").collect_vec();
			let maps = parsed_line[1]
				.split(";")
				.map(|group| {
					group
						.split(",")
						.map(|die_cast| die_cast.trim().split(" ").take(2).collect_tuple().unwrap())
						.map(|(num_string, color)| (color, num_string.trim().parse::<i32>().unwrap()))
						.collect::<HashMap<_, _>>()
				}).collect_vec();
				let greens = maps.iter().map(|m| m.get("green").unwrap_or(&0)).max().unwrap_or(&0);
				let red = maps.iter().map(|m| m.get("red").unwrap_or(&0)).max().unwrap_or(&0);
				let blue = maps.iter().map(|m| m.get("blue").unwrap_or(&0)).max().unwrap_or(&0);
				greens * red * blue
		})
		.sum()
}
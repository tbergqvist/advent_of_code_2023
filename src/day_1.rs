pub fn a(input: &str) -> u32 {
	input.lines()
		.map(|line| 
			{
				let list: Vec<u32> = line.chars()
					.filter_map(|char| char.to_digit(10))
					.collect();

				list.first().unwrap() * 10 + list.last().unwrap()
			})
		.sum()
}

pub fn b(input: &str) -> u32 {
	input.lines()
		.map(|line| 
			{
				let list: Vec<u32> = line
					.replace("one", "o1ne")
					.replace("two", "t2wo")
					.replace("three", "t3hree")
					.replace("four", "f4our")
					.replace("five", "f5ive")
					.replace("six", "s6ix")
					.replace("seven", "s7even")
					.replace("eight", "e8ight")
					.replace("nine", "n9ine")
					.chars()
					.filter_map(|char| char.to_digit(10))
					.collect();

				list.first().unwrap() * 10 + list.last().unwrap()
			})
		.sum()
}
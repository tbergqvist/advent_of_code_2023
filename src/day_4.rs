use itertools::Itertools;

pub fn a(input: &str) -> i32 {
	input.lines()
		.map(|line| {
			let real_row = line.split(":").skip(1).next().unwrap();
			let (winners, others) = real_row.split("|").collect_tuple().unwrap();
			let winning_nos = winners.split(" ").filter(|b| *b != "").map(|b| b.trim().parse::<i32>().unwrap()).collect_vec();
			let other_nos = others.split(" ").filter(|b| *b != "").map(|b| b.trim().parse::<i32>().unwrap()).collect_vec();

			let no_of_winners = other_nos.into_iter().filter(|o| winning_nos.contains(o)).count();
			if no_of_winners == 0  {0} else {1 << no_of_winners - 1}
			
		})
		.sum()
}

pub fn b(input: &str) -> usize {
	let mut winners = input.lines()
		.map(|line| {
			let real_row = line.split(":").skip(1).next().unwrap();
			let (winners, others) = real_row.split("|").collect_tuple().unwrap();
			let winning_nos = winners.split(" ").filter(|b| *b != "").map(|b| b.trim().parse::<i32>().unwrap()).collect_vec();
			let other_nos = others.split(" ").filter(|b| *b != "").map(|b| b.trim().parse::<i32>().unwrap()).collect_vec();

			let no_of_winners = other_nos.into_iter().filter(|o| winning_nos.contains(o)).count();
			no_of_winners
		})
		.map(|n| (n, 1))
		.collect_vec();
	
	for i in 0..winners.len() {
		let (winning_numbers, cards) = winners[i];
		if winning_numbers == 0 {
			continue;
		}
		
		for _ in 0..cards {
			winners[i+1..(i + winning_numbers + 1)].iter_mut().for_each(|(_, cards)| *cards += 1);
		}
		
	}
	
	winners.into_iter().map(|(_, cards)| cards).sum()
}
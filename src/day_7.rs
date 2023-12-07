use std::{cmp::Ordering, collections::HashMap};
use itertools::Itertools;

#[derive(PartialEq, Debug)]
enum Type {
	HighCard,
	TwoOfAKind,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

pub fn a(input: &str) -> usize {
	let mut hands = input
		.lines()
		.map(|l| parse_card_line(l.to_string()))
		.collect_vec();

	hands.sort_by(|(hand1, _), (hand2, _)| compare_hand(hand1, hand2, ';', get_hand_type));
	hands.into_iter()
		.enumerate()
		.fold(0, |sum, (i, (_, count))| {
			 sum + (count as usize) * (i + 1)
		})
}

pub fn b(input: &str) -> usize {
	let mut hands = input
		.lines()
		.map(|l| parse_card_line(l.to_string()))
		.collect_vec();

	hands.sort_by(|(hand1, _), (hand2, _)| compare_hand(hand1, hand2, '0', get_hand_type_joker));
	hands.into_iter()
		.enumerate()
		.fold(0, |sum, (i, (_, count))| {
			sum + (count as usize) * (i + 1)
		})
}

type CardHand = (String, i32);

fn parse_card_line(line: String) -> CardHand {
	let split = line.split_ascii_whitespace().collect_vec();
	(split[0].to_string(), split[1].parse().unwrap())
}

fn replace_letters(hand: &str, joker: char) -> String {
	hand
		.replace("T", ":")
		.replace("J", &joker.to_string())
		.replace("Q", "<")
		.replace("K", "=")
		.replace("A", ">")
}

fn compare_hand<T>(hand1: &str, hand2: &str, joker: char, hand_type: T) -> Ordering where T: Fn(&str)->Type {
	let type1 = hand_type(hand1);
	let type2 = hand_type(hand2);

	match (type1 as i32).cmp(&(type2 as i32)) {
		Ordering::Equal => replace_letters(hand1, joker).cmp(&replace_letters(hand2, joker)),
		Ordering::Greater => Ordering::Greater,
		Ordering::Less => Ordering::Less,
	}
}

fn get_hand_type(hand: &str) -> Type {
	let amount_by_char: HashMap<char, i32> = hand.chars().fold( HashMap::new(),|mut map, card| {
		map.entry(card).and_modify(|count| *count += 1).or_insert(1);
		map
	});
	
	if amount_by_char.iter().any(|(_, count)| *count == 5) {
		return Type::FiveOfAKind;
	} else if amount_by_char.iter().any(|(_, count)| *count == 4) {
		return Type::FourOfAKind;
	} else if amount_by_char.iter().any(|(_, count)| *count == 3) && amount_by_char.iter().any(|(_, count)| *count == 2) {
		return Type::FullHouse;
	} else if amount_by_char.iter().any(|(_, count)| *count == 3) {
		return Type::ThreeOfAKind;
	}
	let pair = amount_by_char.iter().find(|(_, count)| **count == 2).map(|(card, _)| card);
	if pair.is_some() && amount_by_char.iter().any(|(card, count)| *count == 2 && card != pair.unwrap()) {
		Type::TwoPair
	} else if pair.is_some() {
		Type::TwoOfAKind
	} else {
		Type::HighCard
	}
}

fn get_hand_type_joker(hand: &str) -> Type {
	let amount_by_char: HashMap<char, i32> = hand
		.chars()
		.filter(|c| *c != 'J')
		.fold( HashMap::new(),|mut map, card| {
			map.entry(card).and_modify(|count| *count += 1).or_insert(1);
			map
		});
	
	let highest_card_op = amount_by_char.iter().max_by_key(|(_, count)| **count).map(|(card, _)| card);

	if let Some(highest_card) = highest_card_op {
		get_hand_type(&hand.replace("J", &highest_card.to_string()))
	} else {
		get_hand_type(&hand)
	}

}

#[cfg(test)]
mod tests {
	use super::*;
	use std::cmp::Ordering;

	#[test]
	fn parse_line() {
			assert_eq!(parse_card_line("32T3K 765".to_string()), ("32T3K".to_string(), 765));
	}

	#[test]
	fn test_compare_hand() {
		assert_eq!(compare_hand("33332", "2AAAA", ';', get_hand_type), Ordering::Greater);
	}

	#[test]
	fn test_compare_hand2() {
		assert_eq!(compare_hand("Q3333", "TAAAA", ';', get_hand_type), Ordering::Greater);
	}
	
	#[test]
	fn test_compare_hand3() {
		assert_eq!(compare_hand("Q3332", "TAAAA", ';', get_hand_type), Ordering::Less);
	}

	#[test]
	fn test_compare_hand_joker() {
		assert_eq!(compare_hand("KTJJT", "QQQJA", ';', get_hand_type_joker), Ordering::Greater);
	}

	#[test]
	fn test_hand_type() {
		assert_eq!(get_hand_type("33333"), Type::FiveOfAKind);
		assert_eq!(get_hand_type("A3333"), Type::FourOfAKind);
		assert_eq!(get_hand_type("33322"), Type::FullHouse);
		assert_eq!(get_hand_type("33312"), Type::ThreeOfAKind);
		assert_eq!(get_hand_type("33151"), Type::TwoPair);
		assert_eq!(get_hand_type("33125"), Type::TwoOfAKind);
		assert_eq!(get_hand_type("12345"), Type::HighCard);
	}

	#[test]
	fn test_hand_type_joker() {
		assert_eq!(get_hand_type_joker("JJ333"), Type::FiveOfAKind);
		assert_eq!(get_hand_type_joker("A3J33"), Type::FourOfAKind);
		assert_eq!(get_hand_type_joker("3J322"), Type::FullHouse);
		assert_eq!(get_hand_type_joker("33J12"), Type::ThreeOfAKind);
		assert_eq!(get_hand_type_joker("3J125"), Type::TwoOfAKind);
	}

	#[test]
	fn test_a() {
		let test_input = "32T3K 765
		T55J5 684
		KK677 28
		KTJJT 220
		QQQJA 483";
		assert_eq!(a(test_input), 6440);
	}

	#[test]
	fn test_b() {
		let test_input = "32T3K 765
		T55J5 684
		KK677 28
		KTJJT 220
		QQQJA 483";
		assert_eq!(b(test_input), 5905);

	}
}
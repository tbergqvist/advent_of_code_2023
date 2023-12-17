use std::cmp::min;

use itertools::Itertools;

fn check_rows(field: &[Vec<char>], left_x: usize, right_x: usize, smudged: &mut bool) -> bool {
	for y in 0..field.len() {
		if field[y][left_x] != field[y][right_x] {
			if !*smudged {
				*smudged = true;
				continue;
			}
			return false;
		}
	}
	true
}

fn check_cols(field: &[Vec<char>], left_y: usize, right_y: usize, smudged: &mut bool) -> bool {
	for x in 0..field[0].len() {
		if field[left_y][x] != field[right_y][x] {
			if !*smudged {
				*smudged = true;
				continue;
			}
			return false;
		}
	}
	true
}

fn find_vertical(field: &[Vec<char>], smudge: bool) -> Option<usize> {
	for i in 1..field[0].len() {
		let length = min(i, field[0].len() - i);
		let mut broken = false;
		let mut smudged = smudge;
		for j in 0..length {
			let left_x = i - j - 1;
			let right_x = i + j;
			if !check_rows(field, left_x, right_x, &mut smudged) {
				broken = true;
				break;
			}
		}
		if !broken && smudged{
			return Some(i);
		}
	}
	None
}

fn find_horizontal(field: &[Vec<char>], smudge: bool) -> Option<usize> {
	for i in 1..field.len() {
		let length = min(i, field.len() - i);
		let mut broken = false;
		let mut smudged = smudge;
		for j in 0..length {
			let left_y = i - j - 1;
			let right_y = i + j;
			if !check_cols(field, left_y, right_y, &mut smudged) {
				broken = true;
				break;
			}
		}
		
		if !broken && smudged {
			return Some(i);
		}
	}
	None
}

pub fn a(input: &str) -> usize {
	input.split("\r\n\r\n").map(|field| {
		field.lines().map(|f| f.chars().collect_vec()).collect_vec()
	}).map(|field| {
		let horizontal = find_vertical(&field, true);
		let vertical = find_horizontal(&field, true).map(|v| v * 100);
		vertical.or(horizontal).unwrap()
	})
	.sum()
}

pub fn b(input: &str) -> usize {
	input.split("\r\n\r\n").map(|field| {
		field.lines().map(|f| f.chars().collect_vec()).collect_vec()
	}).map(|field| {
		let horizontal = find_vertical(&field, false);
		let vertical = find_horizontal(&field, false).map(|v| v * 100);
		vertical.or(horizontal).unwrap()
	})
	.sum()
}
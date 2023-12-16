use std::collections::HashMap;

use itertools::Itertools;

pub fn a(input: &str) -> usize {
	let gear_thingies = input.lines().map(|line| {
		let split = line.split_ascii_whitespace().collect_vec();
		(split[0].chars().collect_vec(), split[1].split(",").map(|c| c.parse::<usize>().unwrap()).collect_vec())
	})
	.collect_vec();

	gear_thingies.into_iter().map(|(gears, validations)| {
		let mut cache = HashMap::new();
		deep(None, &gears, &validations, &mut cache)
	})
	.sum()
}

pub fn b(input: &str) -> usize {
	let gear_thingies = input.lines().map(|line| {
		let split = line.split_ascii_whitespace().collect_vec();
		(split[0].chars().collect_vec(), split[1].split(",").map(|c| c.parse::<usize>().unwrap()).collect_vec())
	})
	.collect_vec();

	gear_thingies.into_iter().map(|(gears, validations)| {
		let mut clone = Vec::new();
		let mut v_clone = Vec::new();
		for _ in 0..4 {
			clone.extend(gears.clone());
			clone.push('?');
			v_clone.extend(validations.clone())
		}
		clone.extend(gears);
		v_clone.extend(validations);

		(clone, v_clone)
	}).map(|(gears, validations)| {
			let mut cache = HashMap::new();
			deep(None, &gears, &validations, &mut cache)
	})
	.sum()
}



fn deep<'a>(c: Option<char>, gears: &'a [char], validation: &'a [usize], cache: &mut HashMap<(&'a[char], &'a[usize]), usize>) -> usize {
	if validation.len() == 0 {
		return if gears.iter().any(|b| *b == '#') {0} else {1};
	}

	if gears.len() == 0 {
		return 0;
	}

	let sign = c.unwrap_or(gears[0]);
	if sign == '.' {
		return deep(None, &gears[1..], validation, cache);
	} else if sign == '#' {
		if gears.len() == validation[0] && validation.len() == 1 && gears.iter().all(|b| *b == '#' || *b == '?') {
			return 1;
		}

		if gears.len() <= validation[0] {
			return 0;
		}

		for i in 1..validation[0] {
			if gears[i] == '.' {
				return 0;
			}
		}
		
		let last = gears[validation[0]];
		if last == '#' {
			return 0;
		}
		let res = deep(None, &gears[validation[0] + 1..], &validation[1..], cache);
		return res;

	} else if sign == '?' {
		if let Some(cached) = cache.get(&(gears, validation)) {
			return *cached;
		}
		
		let res = deep(Some('#'), &gears, validation, cache) + deep(None, &gears[1..], validation, cache);
		cache.entry((gears, validation)).or_insert(res);
		return res;
	}
	0
}
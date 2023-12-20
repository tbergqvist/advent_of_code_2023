use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
enum Process<'a> {
	If(&'a str, &'a str, i64, &'a str),
	Else(&'a str)
}

pub fn a<'a>(input: &'a str) -> i64 {
	let (workflows, parts) = input.split("\r\n\r\n").collect_tuple().unwrap();

	let workflows: HashMap<&str, Vec<Process>> = workflows
		.lines()
		.map(|line| {
			let end = line.find("{").unwrap();
			let name = &line[0..end];
			let processes = line[(end+1)..].split(",").map(|workflow| {
				workflow.find(":").map(|index| {
					Process::If(&workflow[0..1], &workflow[1..2], workflow[2..index].parse().unwrap(), &workflow[(index + 1)..])
				}).unwrap_or(Process::Else(&workflow[0..(workflow.len() - 1)]))
			})
			.collect_vec();
			(name, processes)
		})
		.collect();

	let parts: Vec<HashMap<&str, i64>> = parts.lines()
		.map(|line| {
			line[1..(line.len()-1)]
				.split(",")
				.map(|bla| {
					let name = &bla[0..1];
					let val = &bla[2..].parse().unwrap();
					(name, *val)
				})
				.collect()
		}).collect_vec();

	parts.into_iter().map(|part| {
		match deep(&workflows, "in", &part) {
			"A" => part.values().sum(),
			"R" => 0,
			_ => panic!("o no")
		}
	})
	.sum()
}


fn deep<'a>(workflows: &'a HashMap<&'a str, Vec<Process>>, next_workflow: &'a str, part: &HashMap<&'a str, i64>) -> &'a str {
	if next_workflow == "A" || next_workflow == "R" {
		return next_workflow;
	}

	let flow = &workflows[next_workflow];
	let next = flow.iter().filter_map(|f| {
		match f {
			Process::If(value_name, operation, value, next_process) => {
				let fi = match *operation {
					">" => part[*value_name] > *value,
					"<" => part[*value_name] < *value,
					_ => panic!("weird operation!")
				};

				if fi { Some(next_process) } else { None }
			},
			Process::Else(next_process) => Some(next_process),
		}
	})
	.next()
	.unwrap();

	deep(workflows, next, part)
}

pub fn b(input: &str) -> i64 {
	let workflows = input.split("\r\n\r\n").take(1).next().unwrap();

	let workflows: HashMap<&str, Vec<Process>> = workflows
		.lines()
		.map(|line| {
			let end = line.find("{").unwrap();
			let name = &line[0..end];
			let processes = line[(end+1)..].split(",").map(|workflow| {
				workflow.find(":").map(|index| {
					Process::If(&workflow[0..1], &workflow[1..2], workflow[2..index].parse().unwrap(), &workflow[(index + 1)..])
				}).unwrap_or(Process::Else(&workflow[0..(workflow.len() - 1)]))
			})
			.collect_vec();
			(name, processes)
		})
		.collect();

		let mut parts = HashMap::new();
		parts.insert("x", (1, 4000));
		parts.insert("m", (1, 4000));
		parts.insert("a", (1, 4000));
		parts.insert("s", (1, 4000));
		deep2(&workflows, "in", parts)
}

fn deep2<'a>(workflows: &'a HashMap<&'a str, Vec<Process>>, next_workflow: &'a str, part: HashMap<&'a str, (i64, i64)>) -> i64 {
	//println!("next_workflow: {}, parts: {:?}", next_workflow, part);
	if next_workflow == "A" {
		return part.values().fold(1, |s, b|  s * (b.1 - b.0 + 1));
	} else if next_workflow == "R" {
		return 0;
	}

	let flow = &workflows[next_workflow];

	let mut sum = 0;
	let mut part = part.clone();
	for process in flow {
		sum += match process {
			Process::If(value_name, operation, value, next_process) => {
				let mut deep_part = part.clone();
				match *operation {
					">" => deep_part.entry(&value_name).and_modify(|val| val.0 = val.0.max(*value + 1)),
					"<" => deep_part.entry(&value_name).and_modify(|val| val.1 = val.1.min(*value - 1)),
					_ => panic!("weird operation")
				};
				match *operation {
					"<" => part.entry(&value_name).and_modify(|val| val.0 = val.0.max(*value)),
					">" => part.entry(&value_name).and_modify(|val| val.1 = val.1.min(*value)),
					_ => panic!("weird operation")
				};
				deep2(workflows, next_process, deep_part)
			},
			Process::Else(next_process) => {
				deep2(workflows, next_process, part.clone())
			}
		}
	}
	sum
}
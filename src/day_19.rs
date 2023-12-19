use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
enum Process<'a> {
	If(&'a str, &'a str, i32, &'a str),
	Else(&'a str)
}

pub fn a<'a>(input: &'a str) -> i32 {
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

	let parts: Vec<HashMap<&str, i32>> = parts.lines()
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


fn deep<'a>(workflows: &'a HashMap<&'a str, Vec<Process>>, next_workflow: &'a str, part: &HashMap<&'a str, i32>) -> &'a str {

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

pub fn b(input: &str) -> usize {
	0
}
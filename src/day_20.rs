use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Module {
	name: String,
	module_type: ModuleType,
	destination_names: Vec<String>
}

#[derive(Debug, Clone)]
enum ModuleType {
	Broadcast,
	FlipFlop(bool),
	Conjunction(HashMap<String, Pulse>)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
	Low,
	High,
}

#[derive(Debug)]
struct Message {
	sender: String,
	receiver: String,
	pulse: Pulse,
}

impl Message {
    fn new(sender: &str, receiver: &str, pulse: Pulse) -> Self { Self { sender: sender.to_string(), receiver: receiver.to_string(), pulse } }
}

pub fn a(input: &str) -> i64 {
	let mut modules: HashMap<String, Module> = input
		.lines()
		.map(|line| {
			let (name, neighbours) = line.split(" -> ").collect_tuple().unwrap();
			let module_type = match &name[0..1] {
				"%" => ModuleType::FlipFlop(false),
				"&" => ModuleType::Conjunction(HashMap::new()),
				_ => ModuleType::Broadcast
			};

			let name_start_idx = match module_type {
				ModuleType::Broadcast => 0,
				_ => 1
			};

			let name = name[name_start_idx..].to_string();
			(name.clone(), Module {
				name,
				destination_names: neighbours.split(", ").map(|n| n.to_string()).collect_vec(),
				module_type
			})
		})
		.collect();

	let yolo = modules.clone();
	for module in modules.values_mut() {
		match &mut module.module_type {
			ModuleType::Conjunction(input_modules) => {
				yolo.values()
					.filter(|neighbor| neighbor.destination_names.contains(&module.name))
					.for_each(|neighbor| {
						input_modules.insert(neighbor.name.clone(), Pulse::Low);
					});
			}
			_ => {}
		}
	}

	let mut sum_low = 0;
	let mut sum_high = 0;
	for _ in 0..1000 {
		let mut queue = VecDeque::new();
		queue.push_back(Message::new("button", "broadcaster", Pulse::Low));

		while let Some(message) = queue.pop_front() {
			match &message.pulse {
				Pulse::High => sum_high += 1,
				Pulse::Low => sum_low += 1,
			}

			let receiver = modules.get_mut(&message.receiver);

			if receiver.is_none() {
				continue;
			}

			let receiver = receiver.unwrap();

			match &mut receiver.module_type {
				ModuleType::Broadcast => {
					for destination in &receiver.destination_names {
						queue.push_back(Message::new(&receiver.name, &destination, message.pulse));
					}
				},
				ModuleType::Conjunction(input_modules) => {
					input_modules.entry(message.sender).and_modify(|pulse| *pulse = message.pulse);
					let pulse_to_send = if input_modules.values().all(|pulse| *pulse == Pulse::High) { Pulse::Low } else { Pulse::High };
					for destination in &receiver.destination_names {
						queue.push_back(Message::new(&receiver.name, &destination, pulse_to_send));
					}
				},
				ModuleType::FlipFlop(current_state) => {
					match message.pulse {
						Pulse::High => {},
						Pulse::Low => {
							*current_state = !*current_state;
							for destination in &receiver.destination_names {
								queue.push_back(Message::new(&receiver.name, &destination, if *current_state {Pulse::High} else {Pulse::Low}));
							}
						},
					}
				},
			}
		}
	}

	sum_high * sum_low
}

pub fn b(input: &str) -> usize {
	let mut modules: HashMap<String, Module> = input
		.lines()
		.map(|line| {
			let (name, neighbours) = line.split(" -> ").collect_tuple().unwrap();
			let module_type = match &name[0..1] {
				"%" => ModuleType::FlipFlop(false),
				"&" => ModuleType::Conjunction(HashMap::new()),
				_ => ModuleType::Broadcast
			};

			let name_start_idx = match module_type {
				ModuleType::Broadcast => 0,
				_ => 1
			};

			let name = name[name_start_idx..].to_string();
			(name.clone(), Module {
				name,
				destination_names: neighbours.split(", ").map(|n| n.to_string()).collect_vec(),
				module_type
			})
		})
		.collect();

	let yolo = modules.clone();
	for module in modules.values_mut() {
		match &mut module.module_type {
			ModuleType::Conjunction(input_modules) => {
				yolo.values()
					.filter(|neighbor| neighbor.destination_names.contains(&module.name))
					.for_each(|neighbor| {
						input_modules.insert(neighbor.name.clone(), Pulse::Low);
					});
			}
			_ => {}
		}
	}

	let mut endpoints: HashMap<&str, Option<usize>> = HashMap::new();
	endpoints.insert("tx", None);
	endpoints.insert("dd", None);
	endpoints.insert("nz", None);
	endpoints.insert("ph", None);

	for i in 1.. {
		let mut queue = VecDeque::new();
		queue.push_back(Message::new("button", "broadcaster", Pulse::Low));

		while let Some(message) = queue.pop_front() {
			if let Some(val) = endpoints.get_mut(&message.sender as &str) {
				if message.pulse == Pulse::High {
					if val.is_none() {
						*val = Some(i);
					}

					if endpoints.values().all(|b| b.is_some()) {
						return endpoints.values().into_iter().map(|b|b.unwrap()).reduce(|sum, val| sum * val).unwrap();
					}
				}
			}

			let receiver = modules.get_mut(&message.receiver);

			if receiver.is_none() {
				continue;
			}

			let receiver = receiver.unwrap();

			match &mut receiver.module_type {
				ModuleType::Broadcast => {
					for destination in &receiver.destination_names {
						queue.push_back(Message::new(&receiver.name, &destination, message.pulse));
					}
				},
				ModuleType::Conjunction(input_modules) => {
					input_modules.entry(message.sender).and_modify(|pulse| *pulse = message.pulse);
					let pulse_to_send = if input_modules.values().all(|pulse| *pulse == Pulse::High) { Pulse::Low } else { Pulse::High };
					for destination in &receiver.destination_names {
						queue.push_back(Message::new(&receiver.name, &destination, pulse_to_send));
					}
				},
				ModuleType::FlipFlop(current_state) => {
					match message.pulse {
						Pulse::High => {},
						Pulse::Low => {
							*current_state = !*current_state;
							for destination in &receiver.destination_names {
								queue.push_back(Message::new(&receiver.name, &destination, if *current_state {Pulse::High} else {Pulse::Low}));
							}
						},
					}
				},
			}
		}
	}
	0
}
fn hash(s: &str) -> usize {
	s.bytes().fold(0, |sum, byte| {
		(sum + byte as usize) * 17 % 256
	})
}

pub fn a(input: &str) -> usize {
	input
		.split(",")
		.map(|command| hash(command))
		.sum()
}

pub fn b(input: &str) -> usize {
	0
}
use std::fs;
use paste::paste;
use advent_of_code_2023::*;


macro_rules! run_days {
	( $( $x:expr), + ) => {
		$(
			{
				paste!{
					let input = fs::read_to_string(
						concat!("./inputs/", stringify!($x), ".txt")
					).unwrap();

					println!(concat!($x, "a:{}"), [<day _ $x>]::a(&input));
					println!(concat!($x, "b:{}"), [<day _ $x>]::b(&input));
				}
			}
		)*
	}
}

fn main() {
	run_days!(1, 2, 3, 4, 5, 6);
}

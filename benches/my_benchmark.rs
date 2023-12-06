use std::fs;
use paste::paste;

use criterion::{criterion_group, criterion_main, Criterion};
use advent_of_code_2023::*;

pub fn criterion_benchmark(c: &mut Criterion) {
		macro_rules! run_days {
			($( $x:expr), + ) => {
				$(
					{
						paste!{
							let input = fs::read_to_string(
								concat!("./inputs/", stringify!($x), ".txt")
							).unwrap();
		
							c.bench_function(concat!("day", stringify!($x), "::a"), |b| b.iter(|| [<day _ $x>]::a(&input)));
							c.bench_function(concat!("day", stringify!($x), "::b"), |b| b.iter(|| [<day _ $x>]::b(&input)));
						}
					}
				)*
			}
		}

		run_days!(1, 2, 3, 4, 5, 6);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
use std::env::args;

use advent_of_code::utils::text_colors::green;

fn get_arguments() {}

fn main() {
	let year: String = match args().nth(1) {
		Some(year) => year,
		None => green("green"),
	};

	let day: String = args().nth(2).expect("DAY");
	dbg!(year, day);
}

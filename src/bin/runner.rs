#[macro_use]
extern crate advent_of_code;

use std::env;

use advent_of_code::utils::challenges::prelude::*;

preload_challenges!(into CHALLENGES);
fn is_year(nr: &u16) -> bool {
	nr >= &2015 && nr <= &2023
}

fn is_day(nr: &u8) -> bool {
	nr >= &1 && nr <= &25
}

fn main() {
	let challenges = &*CHALLENGES;
	let year: u16 = env::args()
		.nth(1)
		.expect("First parameter must be the event year!")
		.parse()
		.expect("Could not parse year to unsigned integer");
	let day: u8 = env::args()
		.nth(2)
		.expect("Second parameter must be the day of the month!")
		.parse()
		.expect("Could not parse year to unsigned integer");

	if !is_year(&year) {
		panic!("Wrong year ({}) supplied", year)
	}

	if !is_day(&day) {
		panic!("Wrong day ({}) supplied", day)
	}

	if let Some(challenge) = challenges.iter().find(|c| c.day == day && c.year == year) {
		challenge.run();
	} else {
		panic!("Could not find year {} day {}", year, day);
	}
}

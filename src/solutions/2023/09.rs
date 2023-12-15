use advent_of_code::utils::challenges::prelude::*;
use regex::Regex;

lazy_static! {
	static ref NUMBERS: Regex = Regex::new(r"(-?\d+)").unwrap();
}

#[derive(Debug)]
struct History {
	values: Vec<i32>,
	calculations: Vec<Vec<i32>>,
}

impl History {
	fn calculate(&mut self, reverse: bool) -> &mut Self {
		if reverse {
			self.values.reverse();
		}
		let mut values = self.values.clone();
		while !self.all_zeros(&values) {
			self.calculations.push(values.clone());
			let mut new_values = Vec::new();

			for window in values.windows(2) {
				if let [current, next] = window {
					new_values.push(next - current);
				}
			}
			values = new_values
		}
		self
	}

	fn prediction(&self) -> i32 {
		self.calculations.iter().map(|list| list.iter().last().unwrap()).sum()
	}

	fn all_zeros(&self, values: &Vec<i32>) -> bool {
		for v in values.iter() {
			if *v != 0 {
				return false;
			}
		}

		true
	}
}

fn parse(input: &PuzzleInput) -> Vec<History> {
	input
		.trim()
		.lines()
		.map(|line| History {
			values: NUMBERS
				.find_iter(line)
				.map(|n| n.as_str().parse::<i32>().unwrap())
				.collect(),
			calculations: Vec::new(),
		})
		.collect()
}

fn part_one(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let mut oasis_report = parse(input);
	Answer(
		oasis_report
			.iter_mut()
			.map(|h| h.calculate(false).prediction())
			.sum::<i32>() as u64,
	)
}

fn part_two(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let mut oasis_report = parse(input);
	Answer(
		oasis_report
			.iter_mut()
			.map(|h| h.calculate(true).prediction())
			.sum::<i32>() as u64,
	)
}

solve!(part_one, part_two);

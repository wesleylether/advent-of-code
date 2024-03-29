use std::collections::VecDeque;

use regex::Regex;

use advent_of_code::utils::challenges::prelude::*;

lazy_static! {
	#[derive(Debug)]
	static ref NUMBERS:Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Debug, Clone)]
struct ScratchCard {
	game: usize,
	winning_numbers: Vec<u32>,
	numbers: Vec<u32>,
}

impl ScratchCard {
	fn new() -> Self {
		Self {
			game: 0,
			winning_numbers: vec![],
			numbers: vec![],
		}
	}

	fn matching_numbers(&self) -> Vec<u32> {
		self.winning_numbers
			.iter()
			.filter(|&&n| self.numbers.contains(&n))
			.cloned()
			.collect()
	}

	fn matching_numbers_count(&self) -> u32 {
		self.matching_numbers().iter().count() as u32
	}

	fn matching_numbers_worth(&self) -> u32 {
		let mut result = 1;
		let count = self.matching_numbers_count();
		if count == 0 {
			return 0;
		}

		for _ in 0..count - 1 {
			result *= 2;
		}
		result
	}
}

fn parse(input: &PuzzleInput) -> Vec<ScratchCard> {
	input
		.trim()
		.lines()
		.enumerate()
		.map(|(game, line)| {
			let mut scratch_card = ScratchCard::new();
			scratch_card.game = game + 1;

			let numbers_of_card: &str = line.split(':').collect::<Vec<&str>>().get(1).unwrap();
			let numbers_split: Vec<&str> = numbers_of_card.split('|').collect::<Vec<&str>>();
			if let Some(winning_numbers) = numbers_split.get(0) {
				scratch_card.winning_numbers = NUMBERS
					.find_iter(winning_numbers)
					.map(|n| n.as_str().parse::<u32>().unwrap())
					.collect();
			}
			if let Some(numbers) = numbers_split.get(1) {
				scratch_card.numbers = NUMBERS
					.find_iter(numbers)
					.map(|n| n.as_str().parse::<u32>().unwrap())
					.collect()
			}
			scratch_card
		})
		.collect()
}

fn part_one(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let scratched_cards = parse(input);
	let result = scratched_cards
		.iter()
		.map(|sc| sc.matching_numbers_worth())
		.sum::<u32>();

	Answer(result as u64)
}

fn part_two(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let scratched_cards = parse(input);
	let mut total = 0;
	let mut copies: VecDeque<ScratchCard> = VecDeque::new();
	copies.extend(scratched_cards.clone());

	while let Some(sc) = copies.pop_front() {
		total += 1;

		let winning_numbers_count = sc.matching_numbers_count() as usize;
		if winning_numbers_count > 0 {
			for to_copy in &scratched_cards[sc.game..(sc.game + winning_numbers_count)] {
				copies.push_back(to_copy.clone())
			}
		}
	}

	Answer(total)
}

solve!(part_one, part_two);

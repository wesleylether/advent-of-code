use regex::Regex;

use advent_of_code::utils::challenges::prelude::*;

lazy_static! {
	#[derive(Debug)]
	static ref CUBES: Regex = Regex::new(r"(?x) (?P<amount>\d+)\ (?P<color>\w+) ").unwrap();
}

#[derive(Debug, Default)]
struct GameRound {
	red: u64,
	green: u64,
	blue: u64,
}

impl GameRound {
	fn new() -> Self {
		Self {
			red: 0,
			green: 0,
			blue: 0,
		}
	}
	fn is_playable(&self, max_red: u64, max_green: u64, max_blue: u64) -> bool {
		self.red <= max_red && self.green <= max_green && self.blue <= max_blue
	}
}

impl From<&str> for GameRound {
	fn from(value: &str) -> Self {
		let mut round = GameRound::new();

		for cap in CUBES.captures_iter(value) {
			let amount = cap["amount"].parse::<u64>().unwrap();
			let color = &cap["color"];

			match color {
				"red" => {
					if round.red < amount {
						round.red = amount;
					}
				}
				"blue" => {
					if round.blue < amount {
						round.blue = amount;
					}
				}
				"green" => {
					if round.green < amount {
						round.green = amount;
					}
				}
				_ => unreachable!(),
			}
		}

		round
	}
}

#[derive(Debug)]
struct Game {
	id: u64,
	rounds: Vec<GameRound>,
}

impl Game {
	fn is_playable(&self, max_red: u64, max_green: u64, max_blue: u64) -> bool {
		self.rounds
			.iter()
			.all(|round| round.is_playable(max_red, max_green, max_blue))
	}

	fn power(&self) -> u64 {
		let mut minimal = GameRound::new();
		for round in &self.rounds {
			minimal.red = minimal.red.max(round.red);
			minimal.green = minimal.green.max(round.green);
			minimal.blue = minimal.blue.max(round.blue);
		}

		minimal.red * minimal.green * minimal.blue
	}
}

fn parse(input: &PuzzleInput) -> Vec<Game> {
	input
		.trim()
		.lines()
		.enumerate()
		.map(|(index, line)| {
			let rounds = line.split(";").map(GameRound::from).collect();
			Game {
				id: (index + 1) as u64,
				rounds,
			}
		})
		.collect()
}

fn part_one(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let games = parse(input);
	let possible = games.iter().filter(|game| game.is_playable(12, 13, 14));
	Answer(possible.map(|game| game.id).sum())
}

fn part_two(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let games = parse(input);
	Answer(games.iter().map(|game| game.power()).sum())
}

solve!(part_one, part_two);

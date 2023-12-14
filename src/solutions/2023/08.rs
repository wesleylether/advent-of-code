use num::integer::lcm;
use std::collections::{BTreeMap, HashMap, VecDeque};

use regex::Regex;

use advent_of_code::utils::challenges::prelude::*;
use advent_of_code::utils::math::Math;

lazy_static! {
	static ref NODES: Regex = Regex::new(r"(?<key>.{3}) = \((?<left>.{3}), (?<right>.{3})\)").unwrap();
}

#[derive(Debug, Clone, Copy)]
enum Direction {
	Left,
	Right,
}

#[derive(Debug)]
struct Game {
	instructions: VecDeque<Direction>,
	nodes: BTreeMap<String, (String, String)>,
	current_nodes: Vec<String>,
}

fn parse(input: &PuzzleInput) -> Game {
	let mut lines = input.trim().lines();
	let instructions = lines
		.next()
		.unwrap()
		.chars()
		.map(|c| match c {
			'L' => Direction::Left,
			'R' => Direction::Right,
			_ => unreachable!(),
		})
		.collect();
	let mut nodes: BTreeMap<String, (String, String)> = BTreeMap::new();
	let mut current_nodes: Vec<String> = Vec::new();
	for node in lines.skip(1) {
		let captures = NODES.captures(node).unwrap();
		nodes.insert(
			captures["key"].into(),
			(captures["left"].into(), captures["right"].into()),
		);

		if captures["key"].ends_with("A") {
			current_nodes.push(captures["key"].to_string())
		}
	}

	Game {
		instructions,
		nodes,
		current_nodes,
	}
}

fn part_one(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let mut game = parse(input);
	let mut index = 0;
	let mut current_node: &str = "AAA";

	while current_node != "ZZZ" {
		index += 1;
		let (left, right) = game.nodes.get(current_node).unwrap();
		let direction = game.instructions.pop_front().unwrap();
		current_node = match direction {
			Direction::Left => left,
			Direction::Right => right,
		};
		game.instructions.push_back(direction);
	}

	Answer(index)
}

fn part_two(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let game = parse(input);
	let mut result: HashMap<String, u32> = HashMap::new();

	for node in game.current_nodes.iter() {
		let mut instructions = game.instructions.clone();
		let mut index = 0;
		let mut current_node = node;

		while !current_node.ends_with("Z") {
			index += 1;
			let (left, right) = game.nodes.get(current_node).unwrap();
			let direction = instructions.pop_front().unwrap();
			current_node = match direction {
				Direction::Left => left,
				Direction::Right => right,
			};
			instructions.push_back(direction);
		}

		result.insert(String::from(node), index);
	}

	Answer(
		result
			.values()
			.map(|n| Math::prime_factors(*n as usize))
			.flat_map(|v| v.into_iter())
			.fold(1, |acc, num| lcm(acc, num as u64)),
	)
}

solve!(part_one, part_two);

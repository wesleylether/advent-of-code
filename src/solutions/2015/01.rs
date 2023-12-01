use advent_of_code::utils::challenges::prelude::*;

fn parse(input: &PuzzleInput) -> Vec<char> {
	input.trim().chars().collect()
}

fn part_one(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let mut floor: i64 = 0;
	for char in parse(&input) {
		if char == '(' {
			floor += 1;
		} else {
			floor -= 1;
		}
	}

	Answer(floor as usize)
}

fn part_two(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let mut floor: i64 = 0;
	let mut character = 0;
	for (i, &char) in parse(&input).iter().enumerate() {
		if char == '(' {
			floor += 1;
		} else {
			floor -= 1;
		}

		if floor == -1 {
			character = i;
			break;
		}
	}

	Answer(character)
}

solve!(part_one, part_two);

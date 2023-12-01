use advent_of_code::utils::challenges::prelude::*;

fn parse(input: &PuzzleInput) -> Vec<&str> {
	input.trim().split("\n").collect()
}

fn find_first_digit(s: &str, reverse: bool) -> Option<i32> {
	let chars: Vec<char> = if reverse {
		s.chars().rev().collect()
	} else {
		s.chars().collect()
	};
	for c in chars {
		if c.is_numeric() {
			return Some(c.to_digit(10).unwrap() as i32);
		}
	}
	None
}

fn get_numbers() -> Vec<(&'static str, i32)> {
	vec![
		("one", 1),
		("two", 2),
		("three", 3),
		("four", 4),
		("five", 5),
		("six", 6),
		("seven", 7),
		("eight", 8),
		("nine", 9),
	]
}

fn find_first_digit_or_written_number(s: &str, reverse: bool) -> Option<i32> {
	if reverse {
		let mut mut_line = s;
		while !mut_line.is_empty() {
			if mut_line.chars().rev().next()?.is_numeric() {
				return Some(mut_line.chars().rev().next()?.to_digit(10).unwrap() as i32);
			}
			for (string_number, number) in get_numbers() {
				if mut_line.ends_with(string_number) {
					return Some(number);
				}
			}
			mut_line = &mut_line[..mut_line.len() - 1];
		}
	} else {
		let mut mut_line = s;
		while !mut_line.is_empty() {
			if mut_line.chars().next()?.is_numeric() {
				return Some(mut_line.chars().next()?.to_digit(10).unwrap() as i32);
			}
			for (string_number, number) in get_numbers() {
				if mut_line.starts_with(string_number) {
					return Some(number);
				}
			}
			mut_line = &mut_line[1..];
		}
	}

	None
}

fn part_one(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let mut total = 0;

	for line in parse(input) {
		let first_digit = find_first_digit(line, false).unwrap();
		let last_digit = find_first_digit(line, true).unwrap();

		total = total + ((first_digit * 10) + last_digit);
	}

	Answer(total as usize)
}

fn part_two(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let mut total = 0;

	for line in parse(input) {
		let first_digit = find_first_digit_or_written_number(line, false).unwrap();
		let last_digit = find_first_digit_or_written_number(line, true).unwrap();

		total = total + ((first_digit * 10) + last_digit);
	}

	Answer(total as usize)
}

solve!(part_one, part_two);

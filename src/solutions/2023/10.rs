use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;

use num::One;

use advent_of_code::utils::challenges::prelude::*;
use advent_of_code::utils::grids::{Grid, GridPoint};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
	North,
	West,
	South,
	East,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
enum Pipe {
	NorthAndSouth,
	EastAndWest,
	NorthAndEast,
	NorthAndWest,
	SouthAndWest,
	SouthAndEast,
	#[default]
	Ground,
	StartPosition,
}

impl fmt::Display for Pipe {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let char = match self {
			Self::NorthAndSouth => '|',
			Self::EastAndWest => '-',
			Self::NorthAndWest => 'J',
			Self::NorthAndEast => 'L',
			Self::SouthAndEast => 'F',
			Self::SouthAndWest => '7',
			Self::Ground => '.',
			Self::StartPosition => 'S',
		};
		write!(f, "{}", char)?;
		Ok(())
	}
}

#[derive(Debug, Default)]
struct Landscape {
	grid: Grid<isize, isize, Pipe>,
}

impl From<&str> for Landscape {
	fn from(schematic: &str) -> Self {
		let mut landscape = Self { ..Self::default() };
		for (y, line) in schematic.lines().enumerate() {
			for (x, char) in line.chars().enumerate() {
				let point = match char {
					'|' => Pipe::NorthAndSouth,
					'-' => Pipe::EastAndWest,
					'L' => Pipe::NorthAndEast,
					'J' => Pipe::NorthAndWest,
					'7' => Pipe::SouthAndWest,
					'F' => Pipe::SouthAndEast,
					'S' => Pipe::StartPosition,
					_ => Pipe::Ground,
				};
				landscape.grid.set((y as isize, x as isize), RefCell::new(point))
			}
		}
		landscape
	}
}

fn get_adjacent_indices((y, x): GridPoint<isize, isize>) -> Vec<(Direction, GridPoint<isize, isize>)> {
	let x1 = isize::one();
	let y1 = isize::one();
	vec![
		(Direction::North, (y - y1, x)),
		(Direction::East, (y, x + x1)),
		(Direction::South, (y + y1, x)),
		(Direction::West, (y, x - x1)),
	]
}

fn direction_to_go(direction: &Direction, pipe: &Pipe, grid_point: &GridPoint<isize, isize>) -> (Option<Direction>, GridPoint<isize, isize>) {
	match direction {
		Direction::North => match pipe {
			Pipe::NorthAndSouth => (Option::from(Direction::North), (grid_point.0 - 1, grid_point.1)),
			Pipe::SouthAndWest => (Option::from(Direction::West), (grid_point.0, grid_point.1 - 1)),
			Pipe::SouthAndEast => (Option::from(Direction::East), (grid_point.0, grid_point.1 + 1)),
			_ => (None, (0, 0)),
		},
		Direction::East => match pipe {
			Pipe::EastAndWest => (Option::from(Direction::East), (grid_point.0, grid_point.1 + 1)),
			Pipe::SouthAndWest => (Option::from(Direction::South), (grid_point.0 + 1, grid_point.1)),
			Pipe::NorthAndWest => (Option::from(Direction::North), (grid_point.0 - 1, grid_point.1)),
			_ => (None, (0, 0)),
		},
		Direction::South => match pipe {
			Pipe::NorthAndSouth => (Option::from(Direction::South), (grid_point.0 + 1, grid_point.1)),
			Pipe::NorthAndWest => (Option::from(Direction::West), (grid_point.0, grid_point.1 - 1)),
			Pipe::NorthAndEast => (Option::from(Direction::East), (grid_point.0, grid_point.1 + 1)),
			_ => (None, (0, 0)),
		},
		Direction::West => match pipe {
			Pipe::EastAndWest => (Option::from(Direction::West), (grid_point.0, grid_point.1 - 1)),
			Pipe::SouthAndEast => (Option::from(Direction::South), (grid_point.0 + 1, grid_point.1)),
			Pipe::NorthAndEast => (Option::from(Direction::North), (grid_point.0 - 1, grid_point.1)),
			_ => (None, (0, 0)),
		},
	}
}

fn parse(input: &PuzzleInput) -> Landscape {
	input.trim().into()
}

fn part_one(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let landscape = parse(input);
	let start_point = &landscape
		.grid
		.iter()
		.find_map(|(point, value)| match value.borrow().deref() {
			Pipe::StartPosition => Some((point, value)),
			_ => None,
		})
		.unwrap();

	let mut next_direction: (Option<Direction>, GridPoint<isize, isize>) = (None, (0, 0));
	for (direction, point) in get_adjacent_indices(*start_point.0) {
		let pipe = landscape.grid.get(point).unwrap().borrow();
		next_direction = direction_to_go(&direction, &pipe, &point);
		if next_direction.0.is_some() {
			break;
		}
	}

	let mut count: u64 = 0;
	let mut next_pipe = landscape.grid.get(next_direction.1).unwrap().borrow();
	while next_pipe.deref() != &Pipe::StartPosition {
		count += 1;
		next_direction = direction_to_go(&next_direction.0.unwrap(), &next_pipe, &next_direction.1);
		next_pipe = landscape.grid.get(next_direction.1).unwrap().borrow();
	}

	Answer(count / 2 + 1)
}

fn part_two(_input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	Unsolved
}

solve!(part_one, part_two);

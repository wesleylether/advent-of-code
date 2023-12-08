use std::fmt;

use advent_of_code::utils::challenges::prelude::*;
use advent_of_code::utils::grids::Grid;

#[derive(Debug, Default)]
enum SchematicElement {
	Number(u32),
	Gear(Vec<u32>),
	Symbol,
	#[default]
	Empty,
}

impl fmt::Display for SchematicElement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let char = match self {
			Self::Number(nr) => char::from_digit(*nr, 10).unwrap(),
			Self::Gear(_) => '*',
			Self::Empty => '.',
			Self::Symbol => '_',
		};
		write!(f, "{}", char)?;
		Ok(())
	}
}

#[derive(Debug, Default)]
struct EngineSchematic {
	grid: Grid<isize, isize, SchematicElement>,
}

impl From<&str> for EngineSchematic {
	fn from(schematic: &str) -> Self {
		let mut engine_schematic = Self { ..Self::default() };
		for (y, line) in schematic.lines().enumerate() {
			for (x, char) in line.chars().enumerate() {
				let point = match char {
					'0'..='9' => SchematicElement::Number(char.to_digit(10).unwrap()),
					'.' => SchematicElement::Empty,
					'*' => SchematicElement::Gear(vec![]),
					_ => SchematicElement::Symbol,
				};
				engine_schematic.grid.set((y as isize, x as isize), point)
			}
		}
		engine_schematic
	}
}

#[derive(Debug)]
struct Result {
	is_part_number: bool,
	number: u32,
	numbers: Vec<u32>,
}

impl Result {
	fn new() -> Self {
		Self {
			is_part_number: false,
			number: 0,
			numbers: vec![],
		}
	}

	fn run_schematics(&mut self, engine_schematic: EngineSchematic) {
		for ((y, x), element) in engine_schematic.grid.iter() {
			if let SchematicElement::Number(n) = element {
				self.grow_number(*n);

				for (_, adjacent) in engine_schematic.grid.adjacent_iter((*y, *x)) {
					if let SchematicElement::Symbol = adjacent {
						self.is_part_number = true;
					}
					if let SchematicElement::Gear(_) = adjacent {
						self.is_part_number = true;
					}
				}

				match engine_schematic.grid.get((*y, *x + 1)) {
					Some(element) => match element {
						SchematicElement::Symbol | SchematicElement::Gear(_) | SchematicElement::Empty => {
							self.add_part_if_part_number()
						}
						_ => (),
					},
					None => self.add_part_if_part_number(),
				}
			}
		}
	}

	fn grow_number(&mut self, number: u32) {
		self.number = self.number * 10 + number
	}

	fn add_part_if_part_number(&mut self) {
		if self.is_part_number {
			self.numbers.push(self.number);
		}

		self.reset();
	}

	fn reset(&mut self) {
		self.number = 0;
		self.is_part_number = false;
	}

	fn total(&self) -> u64 {
		self.numbers.iter().sum::<u32>() as u64
	}
}

fn parse(input: &PuzzleInput) -> EngineSchematic {
	input.trim().into()
}

fn part_one(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let mut result = Result::new();
	result.run_schematics(parse(input));
	Answer(result.total())
}

fn part_two(_input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	Unsolved
}

solve!(part_one, part_two);

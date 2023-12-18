use std::cell::RefCell;
use std::fmt;
use std::ops::{Deref, DerefMut};

use advent_of_code::utils::challenges::prelude::*;
use advent_of_code::utils::grids::{Grid, GridPoint};

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
				engine_schematic.grid.set((y as isize, x as isize), RefCell::new(point))
			}
		}
		engine_schematic
	}
}

#[derive(Debug)]
struct Result {
	is_part_number: bool,
	gear_point: Option<GridPoint<isize, isize>>,
	number: u32,
	numbers: Vec<u32>,
}

impl Result {
	fn new() -> Self {
		Self {
			is_part_number: false,
			gear_point: None,
			number: 0,
			numbers: vec![],
		}
	}

	fn grow_number(&mut self, number: u32) {
		self.number = self.number * 10 + number
	}

	fn add_part_if_part_number(&mut self) {
		if self.is_part_number {
			self.numbers.push(self.number.clone());
		}

		self.reset();
	}

	fn reset(&mut self) {
		self.number = 0;
		self.is_part_number = false;
		self.gear_point = None;
	}

	fn total(&self) -> u64 {
		self.numbers.iter().sum::<u32>() as u64
	}
}

fn run_schematics(schematics: &mut EngineSchematic, result: &mut Result) {
	for ((y, x), element) in schematics.grid.iter() {
		if let SchematicElement::Number(n) = element.borrow().deref() {
			result.grow_number(*n);

			for (point, adjacent) in schematics.grid.adjacent_iter((*y, *x)) {
				if let SchematicElement::Symbol = adjacent.borrow().deref() {
					result.is_part_number = true;
				}
				if let SchematicElement::Gear(_) = adjacent.borrow().deref() {
					result.is_part_number = true;
					result.gear_point = Some(*point);
				}
			}

			let mut add_number = false;

			match schematics.grid.get((*y, *x + 1)) {
				Some(element) => match element.borrow().deref() {
					SchematicElement::Symbol | SchematicElement::Empty => add_number = true,
					SchematicElement::Gear(_) => add_number = true,
					_ => (),
				},
				None => add_number = true,
			}

			if add_number {
				if let Some(point) = result.gear_point {
					match schematics.grid.get(point).unwrap().borrow_mut().deref_mut() {
						SchematicElement::Gear(numbers) => numbers.push(result.number.clone()),
						_ => unreachable!(),
					}
				}
				result.add_part_if_part_number();
			}
		}
	}
}

fn parse(input: &PuzzleInput) -> EngineSchematic {
	input.trim().into()
}

fn part_one(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let mut result = Result::new();
	run_schematics(&mut parse(input), &mut result);

	Answer(result.total())
}

fn part_two(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let mut result = Result::new();
	let mut schematics = parse(input);
	run_schematics(&mut schematics, &mut result);

	let total = schematics
		.grid
		.iter()
		.filter(|(_, node)| match node.borrow().deref() {
			SchematicElement::Gear(numbers) => return if numbers.iter().count() >= 2 { true } else { false },
			_ => false,
		})
		.map(|(_, node)| match node.borrow().deref() {
			SchematicElement::Gear(gears) => gears.iter().product::<u32>(),
			_ => unreachable!(),
		})
		.sum::<u32>();

	Answer(total as u64)
}

solve!(part_one, part_two);

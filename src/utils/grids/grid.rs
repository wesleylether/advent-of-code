use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{AddAssign, Range};

use num::Integer;

type GridPoint<Y, X> = (Y, X);
type GridValue<V> = V;

#[derive(Debug, Default)]
pub struct Grid<X, Y, V> {
	points: BTreeMap<GridPoint<Y, X>, GridValue<V>>,
}

impl<X, Y, V> Grid<X, Y, V>
where
	X: AddAssign + Copy + Debug + Integer,
	Y: AddAssign + Copy + Debug + Integer,
	V: Display,
{
	pub fn column(&self, x: X) -> impl Iterator<Item = Option<&GridValue<V>>> + '_ {
		let mut y = self.min_y();
		let max_y = self.max_y();
		std::iter::from_fn(move || {
			let result = if y <= max_y { Some(self.get((y, x))) } else { None };
			y += Y::one();
			result
		})
	}

	pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = Option<&GridValue<V>>> + '_> + '_ {
		let mut x = self.min_x();
		let max_x = self.max_x();
		std::iter::from_fn(move || {
			let result = if x <= max_x { Some(self.column(x)) } else { None };
			x += X::one();
			result
		})
	}

	pub fn row(&self, y: Y) -> impl Iterator<Item = Option<&GridValue<V>>> + '_ {
		let mut x = self.min_x();
		let max_x = self.max_x();
		std::iter::from_fn(move || {
			let result = if x <= max_x { Some(self.get((y, x))) } else { None };
			x += X::one();
			result
		})
	}

	pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = Option<&GridValue<V>>> + '_> + '_ {
		let mut y = self.min_y();
		let max_y = self.max_y();
		std::iter::from_fn(move || {
			let result = if y <= max_y { Some(self.row(y)) } else { None };
			y += Y::one();
			result
		})
	}

	fn xs(&self) -> impl Iterator<Item = X> + '_ {
		self.points.iter().map(|((_, x), _)| *x)
	}

	fn min_x(&self) -> X {
		self.xs().min().unwrap()
	}

	fn max_x(&self) -> X {
		self.xs().max().unwrap()
	}

	fn ys(&self) -> impl Iterator<Item = Y> + '_ {
		self.points.iter().map(|((y, _), _)| *y)
	}

	fn min_y(&self) -> Y {
		self.ys().min().unwrap()
	}

	fn max_y(&self) -> Y {
		self.ys().max().unwrap()
	}

	pub fn iter(&self) -> impl Iterator<Item = (&GridPoint<Y, X>, &GridValue<V>)> {
		self.points.iter()
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = (&GridPoint<Y, X>, &mut GridValue<V>)> {
		self.points.iter_mut()
	}

	pub fn get_point(&self, point: GridPoint<Y, X>) -> Option<(&GridPoint<Y, X>, &GridValue<V>)> {
		self.points.get_key_value(&point)
	}

	pub fn get(&self, point: GridPoint<Y, X>) -> Option<&GridValue<V>> {
		self.points.get(&point)
	}

	pub fn get_mut(&mut self, point: GridPoint<Y, X>) -> Option<&mut GridValue<V>> {
		self.points.get_mut(&point)
	}

	pub fn set(&mut self, point: GridPoint<Y, X>, value: GridValue<V>) {
		self.points.insert(point, value);
	}

	pub fn adjacent_iter(&self, (y, x): GridPoint<Y, X>) -> impl Iterator<Item = (&GridPoint<Y, X>, &GridValue<V>)> {
		let x1 = X::one();
		let y1 = Y::one();
		let adjacent = [
			(y - y1, x - x1),
			(y - y1, x),
			(y - y1, x + x1),
			(y, x - x1),
			(y, x + x1),
			(y + y1, x - x1),
			(y + y1, x),
			(y + y1, x + x1),
		];
		let mut index = 0;
		std::iter::from_fn(move || {
			while index < adjacent.len() {
				let result = self.get_point(adjacent[index]);
				index += 1;
				if result.is_some() {
					return result;
				}
			}
			None
		})
	}
}

impl<X, Y, V> Display for Grid<X, Y, V>
where
	X: AddAssign + Copy + Debug + Integer + Ord,
	Y: AddAssign + Copy + Debug + Integer + Ord,
	V: Display,
	Range<X>: Iterator<Item = X>,
	Range<Y>: Iterator<Item = Y>,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for y in self.min_y()..self.max_y() + Y::one() {
			for x in self.min_x()..self.max_x() + X::one() {
				if let Some(value) = self.points.get(&(y, x)) {
					write!(f, "{}", value)?;
				} else {
					write!(f, ".")?;
				}
			}
			writeln!(f, "\n")?;
		}
		Ok(())
	}
}

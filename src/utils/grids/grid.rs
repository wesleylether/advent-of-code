use std::array::IntoIter;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{AddAssign, Range};

use num::Integer;

pub type GridPoint<Y, X> = (Y, X);
pub type GridValue<V> = V;

#[derive(Debug, Default)]
pub struct Grid<X, Y, V> {
	points: BTreeMap<GridPoint<Y, X>, RefCell<GridValue<V>>>,
}

impl<X, Y, V> Grid<X, Y, V>
where
	X: AddAssign + Copy + Debug + Integer,
	Y: AddAssign + Copy + Debug + Integer,
	V: Display,
{
	pub fn column(&self, x: X) -> impl Iterator<Item = Option<&RefCell<GridValue<V>>>> + '_ {
		let mut y = self.min_y();
		let max_y = self.max_y();
		std::iter::from_fn(move || {
			let result = if y <= max_y { Some(self.get((y, x))) } else { None };
			y += Y::one();
			result
		})
	}

	pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = Option<&RefCell<GridValue<V>>>> + '_> + '_ {
		let mut x = self.min_x();
		let max_x = self.max_x();
		std::iter::from_fn(move || {
			let result = if x <= max_x { Some(self.column(x)) } else { None };
			x += X::one();
			result
		})
	}

	pub fn row(&self, y: Y) -> impl Iterator<Item = Option<&RefCell<GridValue<V>>>> + '_ {
		let mut x = self.min_x();
		let max_x = self.max_x();
		std::iter::from_fn(move || {
			let result = if x <= max_x { Some(self.get((y, x))) } else { None };
			x += X::one();
			result
		})
	}

	pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = Option<&RefCell<GridValue<V>>>> + '_> + '_ {
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

	pub fn iter(&self) -> impl Iterator<Item = (&GridPoint<Y, X>, &RefCell<GridValue<V>>)> {
		self.points.iter()
	}

	pub fn get_point(&self, point: GridPoint<Y, X>) -> Option<(&GridPoint<Y, X>, &RefCell<GridValue<V>>)> {
		self.points.get_key_value(&point)
	}

	pub fn get(&self, point: GridPoint<Y, X>) -> Option<&RefCell<GridValue<V>>> {
		self.points.get(&point)
	}

	pub fn set(&mut self, point: GridPoint<Y, X>, value: RefCell<GridValue<V>>) {
		self.points.insert(point, value);
	}

	pub fn adjacent_iter_indices(&self, (y, x): GridPoint<Y, X>) -> impl Iterator<Item = GridPoint<Y, X>> + '_ {
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
		IntoIter::filter_map(adjacent.into_iter(), move |point| {
			if self.points.contains_key(&point) {
				Some(point)
			} else {
				None
			}
		})
	}

	pub fn adjacent_iter(
		&self,
		point: GridPoint<Y, X>,
	) -> impl Iterator<Item = (&GridPoint<Y, X>, &RefCell<GridValue<V>>)> + '_ {
		self.adjacent_iter_indices(point)
			.filter_map(move |adj_point| self.get_point(adj_point))
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
					write!(f, "{}", value.borrow())?;
				} else {
					write!(f, ".")?;
				}
			}
			writeln!(f, "\n")?;
		}
		Ok(())
	}
}

use std::str::FromStr;

pub enum StaticGridError {
	ParseFailure,
}

#[derive(Debug)]
pub struct StaticGrid<T> {
	pub grid: Vec<Vec<T>>,
}

impl<T> StaticGrid<T> {
	fn new() -> Self {
		Self { grid: vec![] }
	}
}

impl<T> Default for StaticGrid<T> {
	fn default() -> Self {
		StaticGrid::new()
	}
}

impl<T: FromStr> FromStr for StaticGrid<T> {
	type Err = StaticGridError;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		let grid: Vec<Vec<T>> = str
			.lines()
			.map(|line| {
				line.chars()
					.map(|c| c.to_string().parse::<T>())
					.collect::<Result<Vec<T>, _>>()
					.map_err(|_| StaticGridError::ParseFailure)
			})
			.collect::<Result<Vec<Vec<T>>, _>>()?;
		Ok(StaticGrid { grid })
	}
}

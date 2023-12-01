use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{env, fmt, fs};

use colored::Colorize;
use dotenv::dotenv;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

use super::strings::StringExt;

pub mod macros;
pub mod prelude;

pub const PUZZLE_ROOT: &str = "src/puzzle_inputs";

type Day = u8;
type Year = u16;
pub type PuzzleInput = String;
type PartIdentifier = String;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Solution {
	Answer(usize),
	StringAnswer(String),
	Unsolved,
}

impl fmt::Display for Solution {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Solution::Answer(nr) => nr.to_string(),
				Solution::StringAnswer(str) => format!("'{}'", str),
				Solution::Unsolved => "Unsolved".to_string(),
			}
		)
	}
}

type SolutionFn = fn(&PuzzleInput, &RawPuzzleArgs) -> Solution;

pub struct SolutionPart {
	pub ident: PartIdentifier,
	pub solution_fn: SolutionFn,
}

impl fmt::Debug for SolutionPart {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("SolutionPart")
			.field("ident", &self.ident)
			.field("solution_fn", &format_args!("{:p}", &self.solution_fn))
			.finish()
	}
}

impl SolutionPart {
	pub fn title(&self) -> String {
		self.ident.to_string().titleize()
	}

	pub fn new(ident: &'static str, solution_fn: SolutionFn) -> SolutionPart {
		SolutionPart {
			ident: PartIdentifier::from(ident.to_string()),
			solution_fn,
		}
	}
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PuzzleArg {
	Boolean(bool),
	Number(u64),
	String(String),
}

pub type RawPuzzleArgs = BTreeMap<String, PuzzleArg>;

#[derive(Debug)]
pub struct Challenge {
	pub year: Year,
	pub day: Day,
	pub parts: &'static Vec<SolutionPart>,
}

impl Challenge {
	fn input(&self) -> PuzzleInput {
		let mut path: PathBuf = self.puzzle_path();
		path.push(format!("{:02}.txt", self.day));

		match fs::read_to_string(&path) {
			Ok(puzzle_input) => puzzle_input,
			Err(_) => self.download_puzzle(),
		}
	}

	fn download_puzzle(&self) -> PuzzleInput {
		dotenv().ok();

		let aoc_session = env::var("AOC_SESSION").expect("AOC_SESSION is not configured in the .env file");

		let url = format!("https://adventofcode.com/{}/day/{}/input", self.year, self.day);
		let client = reqwest::blocking::Client::new();
		let mut headers = HeaderMap::new();
		headers.insert(
			"Cookie",
			HeaderValue::from_str(&format!("session={}", aoc_session))
				.expect("Couldn't create the correct Cookie header value"),
		);
		let response = client.get(&url).headers(headers).send();

		match response {
			Ok(response) => {
				if response.status().is_success() {
					let puzzle_input = response.text().expect("Failed to parse retrieved input data");
					self.save_puzzle(&puzzle_input);
					puzzle_input
				} else {
					panic!(
						"Failed to download input data with http response code: {}",
						response.status()
					)
				}
			}
			Err(error) => {
				panic!("Couldn't load input file from url: {}, with error: {}", url, error)
			}
		}
	}

	fn puzzle_path(&self) -> PathBuf {
		let mut path: PathBuf = PathBuf::from(PUZZLE_ROOT);
		path.push(self.year.to_string());
		path
	}

	fn save_puzzle(&self, puzzle_input: &PuzzleInput) {
		let mut path = self.puzzle_path();
		fs::create_dir_all(&path).expect("Could not create directories");
		path.push(format!("{:02}.txt", self.day));
		fs::write(&path, puzzle_input).expect("Could not create file");
	}

	fn execute<'a>(&self, part: &SolutionPart, input: &'a PuzzleInput, args: &RawPuzzleArgs) -> (Solution, Duration) {
		let start = Instant::now();
		let result = (part.solution_fn)(input, args);
		let duration = start.elapsed();
		(result, duration)
	}

	pub fn run(&self) {
		let input = self.input();
		let args = RawPuzzleArgs::new();

		lazy_static! {
			static ref WHITESPACE: Regex = Regex::new(r"\s+").unwrap();
		}

		for part in self.parts {
			let fmt_header = format!("{} · Day {} · {}", self.year, self.day, part.title(),).cyan();
			println!("{}", fmt_header);

			let (result, duration) = self.execute(part, &input, &args);
			self.output(&result, &duration);
			println!();
		}
	}

	fn output(&self, result: &Solution, duration: &Duration) {
		let fmt_label = "Answer".normal();

		let fmt_text = match result {
			Solution::Answer(nr) => nr.to_string().green(),
			Solution::StringAnswer(str) => str.green(),
			Solution::Unsolved => "[not yet solved]".red(),
		};

		let fmt_suffix = if matches!(result, Solution::Answer(_)) {
			let nanos = duration.as_nanos();
			if nanos >= 1000000 {
				format!(" {}ms", num::Integer::div_ceil(&nanos, &1000000)).bright_black()
			} else if nanos >= 1000 {
				format!(" {}μs", num::Integer::div_ceil(&nanos, &1000)).bright_black()
			} else {
				format!(" {}ns", nanos).bright_black()
			}
			.to_string()
		} else {
			"".to_string()
		};

		println!(" => {}: {}{}", fmt_label, fmt_text, fmt_suffix);
	}

	pub fn new(year: Year, day: Day, parts: &'static Vec<SolutionPart>) -> Challenge {
		Challenge { year, day, parts }
	}
}

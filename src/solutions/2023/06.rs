use regex::Regex;

use advent_of_code::utils::challenges::prelude::*;

lazy_static! {
	static ref NUMBERS: Regex = Regex::new(r"(\d+)").unwrap();
}

#[derive(Debug)]
struct RaceRecord {
	time: u64,
	distance: u64,
}

impl RaceRecord {
	fn record_breakers(&self) -> Vec<u64> {
		let mut record_breakers: Vec<u64> = Vec::new();
		for i in 1..self.time {
			let distance = i * (self.time - i);
			if distance > self.distance {
				record_breakers.push(i);
			}
		}
		record_breakers
	}

	fn count_record_breakers(&self) -> usize {
		self.record_breakers().iter().count()
	}
}

fn parse(input: &PuzzleInput) -> Vec<RaceRecord> {
	let mut lines = input.trim().lines();
	let time_line = lines.nth(0).unwrap();
	let distance_line = lines.nth(0).unwrap();

	let times: Vec<u64> = NUMBERS
		.find_iter(time_line)
		.map(|n| n.as_str().parse::<u64>().unwrap())
		.collect();

	let distances: Vec<u64> = NUMBERS
		.find_iter(distance_line)
		.map(|n| n.as_str().parse::<u64>().unwrap())
		.collect();

	times
		.iter()
		.zip(distances.iter())
		.map(|(time, distance)| RaceRecord {
			time: *time,
			distance: *distance,
		})
		.collect::<Vec<RaceRecord>>()
}

fn part_one(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let race_records = parse(input);
	Answer(
		race_records
			.iter()
			.map(|rr| rr.count_record_breakers() as u64)
			.product::<u64>() as u64,
	)
}

fn part_two(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let race_records = parse(input);
	let time: String = race_records.iter().map(|n| n.time.to_string()).collect();
	let distance: String = race_records.iter().map(|n| n.distance.to_string()).collect();
	let rr = RaceRecord {
		time: time.parse::<u64>().unwrap(),
		distance: distance.parse::<u64>().unwrap(),
	};
	Answer(rr.count_record_breakers() as u64)
}

solve!(part_one, part_two);

use regex::Regex;

use advent_of_code::utils::challenges::prelude::*;

lazy_static! {
	#[derive(Debug)]
	static ref NUMBERS: Regex = Regex::new(r"\d+").unwrap();
	#[derive(Debug)]
	static ref RANGE_NUMBERS: Regex = Regex::new(r"(?<destination>\d+) (?<source>\d+) (?<length>\d+)").unwrap();
}

#[derive(Debug)]
struct Range {
	source: u64,
	destination: u64,
	length: u64,
}

#[derive(Debug)]
struct Almanac {
	seeds: Vec<u64>,
	seed_to_soil: Vec<Range>,
	soil_to_fertilizer: Vec<Range>,
	fertilizer_to_water: Vec<Range>,
	water_to_light: Vec<Range>,
	light_to_temperature: Vec<Range>,
	temperature_to_humidity: Vec<Range>,
	humidity_to_location: Vec<Range>,
}

impl Almanac {
	fn new() -> Self {
		Self {
			seeds: Vec::new(),
			seed_to_soil: Vec::new(),
			soil_to_fertilizer: Vec::new(),
			fertilizer_to_water: Vec::new(),
			water_to_light: Vec::new(),
			light_to_temperature: Vec::new(),
			temperature_to_humidity: Vec::new(),
			humidity_to_location: Vec::new(),
		}
	}

	fn get_location_from_seed(&self, seed: &u64) -> (u64, u64) {
		let soil = get_destination(&self.seed_to_soil, seed.into()).unwrap_or(*seed);
		let fertilizer = get_destination(&self.soil_to_fertilizer, &soil).unwrap_or(soil);
		let water = get_destination(&self.fertilizer_to_water, &fertilizer).unwrap_or(fertilizer);
		let light = get_destination(&self.water_to_light, &water).unwrap_or(water);
		let temperature = get_destination(&self.light_to_temperature, &light).unwrap_or(light);
		let humidity = get_destination(&self.temperature_to_humidity, &temperature).unwrap_or(temperature);
		let location = get_destination(&self.humidity_to_location, &humidity).unwrap_or(humidity);

		(*seed, location)
	}

	fn seed_locations(&self) -> Vec<(u64, u64)> {
		self.seeds
			.iter()
			.map(|seed| self.get_location_from_seed(seed))
			.collect()
	}

	fn seed_range_locations(&self) -> Vec<(u64, u64)> {
		let mut locations = Vec::new();
		let seed_ranges: Vec<(u64, u64)> = self.seeds.chunks(2).map(|chunks| (chunks[0], chunks[1])).collect();
		for (seed, range) in seed_ranges.iter() {
			for s in *seed..*seed + *range {
				locations.push(self.get_location_from_seed(&s));
			}
		}
		locations
	}
}

fn get_destination(ranges: &Vec<Range>, number: &u64) -> Option<u64> {
	for range in ranges.iter() {
		if number >= &range.source && number <= &(&range.source + &range.length) {
			return Some(&range.destination + &(number - &range.source));
		}
	}
	None
}

fn set_range_map(map: &mut Vec<Range>, str: &str) {
	for range in str.split(":\n").nth(1).unwrap().split("\n") {
		let range_numbers = RANGE_NUMBERS.captures(range).unwrap();
		let source = &range_numbers["source"].parse::<u64>().unwrap();
		let destination = &range_numbers["destination"].parse::<u64>().unwrap();
		let length = &range_numbers["length"].parse::<u64>().unwrap();

		map.push(Range {
			source: *source,
			destination: *destination,
			length: *length,
		})
	}
}

fn parse(input: &PuzzleInput) -> Almanac {
	let mut almanac = Almanac::new();
	for x in input.trim().split("\n\n") {
		match x {
			x if x.starts_with("seeds") => {
				almanac.seeds = NUMBERS
					.find_iter(x)
					.map(|n| n.as_str().parse::<u64>().unwrap())
					.collect();
			}
			x if x.starts_with("seed-") => set_range_map(&mut almanac.seed_to_soil, x),
			x if x.starts_with("soil-") => set_range_map(&mut almanac.soil_to_fertilizer, x),
			x if x.starts_with("fertilizer-") => set_range_map(&mut almanac.fertilizer_to_water, x),
			x if x.starts_with("water-") => set_range_map(&mut almanac.water_to_light, x),
			x if x.starts_with("light-") => set_range_map(&mut almanac.light_to_temperature, x),
			x if x.starts_with("temperature-") => set_range_map(&mut almanac.temperature_to_humidity, x),
			x if x.starts_with("humidity-") => set_range_map(&mut almanac.humidity_to_location, x),
			_ => unreachable!(),
		}
	}
	almanac
}

fn part_one(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let almanac = parse(input);
	let locations = almanac.seed_locations();
	let min_location: &u64 = locations.iter().map(|(_, location)| location).min().unwrap();

	Answer(*min_location)
}

fn part_two(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let almanac = parse(input);
	let locations = almanac.seed_range_locations();
	let min_location: &u64 = locations.iter().map(|(_, location)| location).min().unwrap();

	Answer(*min_location)
}

solve!(part_one, part_two);

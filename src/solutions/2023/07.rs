use std::cmp::Ordering;
use std::collections::BTreeMap;

use regex::Regex;

use advent_of_code::utils::challenges::prelude::*;

lazy_static! {
	static ref HAND_WITH_BID: Regex = Regex::new(r"(?<hand>.{5}) (?<bid>\d+)").unwrap();
}

#[derive(Debug, Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq)]
enum HandRank {
	FiveOfAKind,
	FourOfAKind,
	FullHouse,
	ThreeOfAKind,
	TwoPair,
	OnePair,
	HighCard,
	#[default]
	Undefined,
}

#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
enum CardRank {
	A,
	K,
	Q,
	J,
	T,
	Num(u8),
	#[default]
	Undefined,
}

fn char_to_card_rank(c: char) -> CardRank {
	match c {
		'A' => CardRank::A,
		'K' => CardRank::K,
		'Q' => CardRank::Q,
		'J' => CardRank::J,
		'T' => CardRank::T,
		'9' => CardRank::Num(9),
		'8' => CardRank::Num(8),
		'7' => CardRank::Num(7),
		'6' => CardRank::Num(6),
		'5' => CardRank::Num(5),
		'4' => CardRank::Num(4),
		'3' => CardRank::Num(3),
		'2' => CardRank::Num(2),
		_ => CardRank::Undefined,
	}
}

#[derive(Debug, Clone)]
struct Hand {
	hand: String,
	hand_rank: HandRank,
	cards: BTreeMap<char, u32>,
	bid: u64,
}

impl Hand {
	fn calculate_rank(&mut self) {
		match self.cards.values().max().unwrap() {
			5 => self.hand_rank = HandRank::FiveOfAKind,
			4 => self.hand_rank = HandRank::FourOfAKind,
			3 => {
				if self.cards.iter().count() == 2 {
					self.hand_rank = HandRank::FullHouse;
				} else {
					self.hand_rank = HandRank::ThreeOfAKind;
				}
			}
			2 => {
				if self.cards.iter().count() == 3 {
					self.hand_rank = HandRank::TwoPair;
				} else {
					self.hand_rank = HandRank::OnePair;
				}
			}
			_ => self.hand_rank = HandRank::HighCard,
		}
	}

	fn calculate_joker_rank(&mut self) {
		match self.cards.get(&'J') {
			Some(4) | Some(5) => self.hand_rank = HandRank::FiveOfAKind,
			Some(3) => match self.cards.iter().count() {
				3 => self.hand_rank = HandRank::FourOfAKind,
				2 => self.hand_rank = HandRank::FiveOfAKind,
				_ => unreachable!(),
			},
			Some(2) => match self.cards.iter().count() {
				4 => self.hand_rank = HandRank::ThreeOfAKind,
				3 => self.hand_rank = HandRank::FourOfAKind,
				2 => self.hand_rank = HandRank::FiveOfAKind,
				_ => unreachable!(),
			},
			Some(1) => match self.cards.iter().count() {
				5 => self.hand_rank = HandRank::OnePair,
				4 => self.hand_rank = HandRank::ThreeOfAKind,
				3 => match self.cards.values().max().unwrap() {
					3 => self.hand_rank = HandRank::FourOfAKind,
					2 => self.hand_rank = HandRank::FullHouse,
					_ => unreachable!(),
				},
				2 => self.hand_rank = HandRank::FiveOfAKind,
				_ => unreachable!(),
			},
			None => self.calculate_rank(),
			_ => unreachable!(),
		}
	}
}

#[derive(Debug)]
struct Game {
	hands: Vec<Hand>,
}

impl Game {
	fn calculate_hands(&mut self) {
		self.hands.iter_mut().map(|h| h.calculate_rank()).collect()
	}

	fn calculate_joker_hands(&mut self) {
		self.hands.iter_mut().map(|h| h.calculate_joker_rank()).collect()
	}

	fn sort_hands(&mut self, joker: bool) {
		self.hands.sort_by(|a, b| {
			if a.hand_rank != b.hand_rank {
				return a.hand_rank.cmp(&b.hand_rank);
			}

			let mut a_cards = a.hand.chars();
			let mut b_cards = b.hand.chars();

			let mut a_card_rank = a_cards.next().map(char_to_card_rank);
			let mut b_card_rank = b_cards.next().map(char_to_card_rank);

			while let (Some(a), Some(b)) = (a_card_rank, b_card_rank) {
				match a.cmp(&b) {
					Ordering::Equal => {
						a_card_rank = a_cards.next().map(char_to_card_rank);
						b_card_rank = b_cards.next().map(char_to_card_rank);
						continue;
					}
					ordering => {
						if joker {
							if a == CardRank::J {
								return Ordering::Greater;
							} else if b == CardRank::J {
								return Ordering::Less;
							}
						}
						if let (CardRank::Num(ax), CardRank::Num(bx)) = (&a, &b) {
							return bx.cmp(&ax);
						}
						return ordering;
					}
				}
			}

			Ordering::Equal
		})
	}
}

fn parse(input: &PuzzleInput) -> Game {
	let mut hands = Vec::new();
	for game in input.trim().lines() {
		let captures = HAND_WITH_BID.captures(game).unwrap();
		let mut cards = BTreeMap::new();

		for card in captures["hand"].chars() {
			if let Some(amount) = cards.get(&card) {
				cards.insert(card, *amount + 1);
			} else {
				cards.insert(card, 1);
			}
		}

		let hand = Hand {
			hand: String::from(&captures["hand"]),
			hand_rank: HandRank::Undefined,
			cards,
			bid: captures["bid"].parse::<u64>().unwrap(),
		};

		hands.push(hand)
	}
	Game { hands }
}

fn part_one(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let mut game = parse(input);
	game.calculate_hands();
	game.sort_hands(false);
	game.hands.reverse();

	let total: u64 = game
		.hands
		.iter()
		.enumerate()
		.fold(0, |acc, (index, value)| acc + (index as u64 + 1) * value.bid);

	Answer(total)
}

fn part_two(input: &PuzzleInput, _args: &RawPuzzleArgs) -> Solution {
	let mut game = parse(input);
	game.calculate_joker_hands();
	game.sort_hands(true);
	game.hands.reverse();

	let total: u64 = game
		.hands
		.iter()
		.enumerate()
		.fold(0, |acc, (index, value)| acc + (index as u64 + 1) * value.bid);

	Answer(total)
}

solve!(part_one, part_two);

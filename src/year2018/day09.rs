type GeneratorOut = Game;

use nom::types::CompleteStr;
use std::{collections::VecDeque, convert::AsRef};

pub struct Game {
	player_count: u32,
	last_marble: u32,
}

impl AsRef<Game> for Game {
	fn as_ref(&self) -> &Game {
		self
	}
}

named!(number <CompleteStr, u32>, map!(take_while1!(|c| char::is_ascii_digit(&c)), |s| s.parse::<u32>().unwrap()));

named!(parse_line <CompleteStr, Game>, do_parse!(
	player_count: number >>
	tag!(" players; last marble is worth ") >>
	last_marble: number  >>
	tag!(" points") >>
	(Game {player_count, last_marble})
));

#[aoc_generator(day9)]
pub fn generator(input: &str) -> GeneratorOut {
	parse_line(CompleteStr(input))
		.expect("input file should be valid")
		.1
}

#[aoc(day9, part1)]
pub fn part_1(input: &GeneratorOut) -> u64 {
	fn rotate<T>(deque: &mut VecDeque<T>) {
		if let Some(elt) = deque.pop_front() {
			deque.push_back(elt);
		}
	}
	fn rotate_back<T>(deque: &mut VecDeque<T>) {
		if let Some(elt) = deque.pop_back() {
			deque.push_front(elt);
		}
	}

	let mut ring =
		VecDeque::with_capacity((input.last_marble - input.last_marble / 23 + 1) as usize);
	ring.push_back(0u32);

	let score_count = if input.player_count % 23 == 0 {
		input.player_count as usize / 23
	} else {
		input.player_count as usize
	};
	let mut scores = std::iter::repeat(0u64)
		.take(score_count)
		.collect::<Vec<_>>();
	let mut score_index = 0usize;

	for n in 1..=input.last_marble {
		if n % 23 != 0 {
			rotate(&mut ring);
			ring.push_back(n);
		} else {
			for _ in 0..7 {
				rotate_back(&mut ring);
			}
			let bonus = ring.pop_back().unwrap();
			rotate(&mut ring);
			scores[score_index] += u64::from(n + bonus);
			score_index = (score_index + 1) % score_count;
		}
	}
	scores.into_iter().max().unwrap()
}

#[aoc(day9, part2)]
pub fn part_2(input: &GeneratorOut) -> u64 {
	part_1(&Game {
		player_count: input.player_count,
		last_marble: input.last_marble * 100,
	})
}

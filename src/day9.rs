type GeneratorOut = Game;

use std::{
	convert::AsRef,
	collections::LinkedList
};
use nom::types::CompleteStr;

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
	tag!(" points\n") >>
	(Game {player_count, last_marble})
));

#[aoc_generator(day9)]
pub fn generator(input: &str) -> GeneratorOut {
	parse_line(CompleteStr(input)).unwrap().1
}

struct Ring {
	n: u32,
	head: LinkedList<u32>,
	tail: LinkedList<u32>,
}

impl Ring {
	fn new() -> Self {
		let mut tail = LinkedList::new();
		tail.push_back(0);
		Ring {
			n: 0,
			head: LinkedList::new(),
			tail
		}
	}
}

impl Iterator for Ring {
	type Item = u32;
	fn next(&mut self) -> Option<Self::Item> {
		fn next_removal(index: usize, length: usize) -> usize {
			(length + index - 7) % length
		}
		self.n += 1;
		while self.n % 23 != 0 {
			if self.tail.len() == 0 {
				std::mem::swap(&mut self.tail, &mut self.head);
			}
			self.head.push_back(self.tail.pop_front().unwrap());
			self.head.push_back(self.n);
			self.n += 1;
		}
		if self.head.len() >= 7 {
			let index = next_removal(self.head.len(), self.head.len() + self.tail.len());
			let mut tmp = self.head.split_off(index+1);
			let bonus = self.head.pop_back().unwrap();
			self.head.push_back(tmp.pop_front().unwrap());
			tmp.append(&mut self.tail);
			self.tail = tmp;
			Some(bonus)
		} else {
			let index = next_removal(self.head.len(), self.head.len() + self.tail.len());
			self.head.append(&mut self.tail);
			self.tail = self.head.split_off(index);
			let bonus = self.head.pop_back().unwrap();
			self.head.push_back(self.tail.pop_front().unwrap());
			Some(bonus)
		}
	}
}

#[aoc(day9, part1)]
pub fn part_1(input: &GeneratorOut) -> u32 {
	fn next_insertion(index: usize, length: usize) -> usize {
		((index + 1) % length) + 1
	}
	fn next_removal(index: usize, length: usize) -> usize {
		(length + index - 7) % length
	}
	let mut ring = vec![0];
	let mut index = 0;
	let score_count = if input.player_count % 23 == 0 {
		input.player_count as usize / 23
	} else {
		input.player_count as usize
	};
	let mut scores = std::iter::repeat(0u32).take(score_count).collect::<Vec<_>>();
	let mut score_index = 0usize;
	for n in 1u32..=input.last_marble {
		if n % 23 == 0 {
			index = next_removal(index, ring.len());
			let bonus = ring.remove(index);
			scores[score_index] += n + bonus;
			score_index = (score_index + 1) % scores.len();
		} else {
			index = next_insertion(index, ring.len());
			ring.insert(index, n);
		}
	}
	println!("{:?}", ring);
	scores.into_iter().max().unwrap()
}

#[aoc(day9, part2)]
pub fn part_2(input: &GeneratorOut) -> u32 {
	part_1(&Game { player_count: input.player_count, last_marble: input.last_marble * 100 })
}

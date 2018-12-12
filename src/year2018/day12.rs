type GeneratorOut = Input;
type PartIn = GeneratorOut;

use itertools::multipeek;
use nom::types::CompleteStr;
use std::convert::AsRef;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
	Dead = 0,
	Alive = 1,
}

#[derive(Debug, Clone)]
struct Rule {
	pattern: [State; 5],
	result: State,
}

#[derive(Debug, Clone)]
pub struct Input {
	initial_state: Vec<State>,
	rules: Vec<Rule>,
}

impl AsRef<Input> for Input {
	fn as_ref(&self) -> &Input {
		self
	}
}

named!(parse_state <CompleteStr, State>, alt!(
	value!(State::Dead, tag!("."))
	| value!(State::Alive, tag!("#"))
));

named!(parse_rule <CompleteStr, Rule>, do_parse!(
	pattern: count_fixed!(State, parse_state, 5) >>
	tag!(" => ") >>
	result: parse_state >>
	(Rule {pattern, result})
));

named!(parse_input <CompleteStr, Input>, do_parse!(
	tag!("initial state: ") >>
	initial_state: many0!(parse_state) >>
	take_while!(|c: char| c.is_ascii_whitespace()) >>
	rules: many0!(terminated!(parse_rule, take_while!(|c: char| c.is_ascii_whitespace()))) >>
	(Input {initial_state, rules})
));

#[aoc_generator(day12)]
pub fn generator(input: &str) -> GeneratorOut {
	parse_input(CompleteStr(input))
		.expect("input file should be valid")
		.1
}

fn rule_lut(rules: &[Rule]) -> Vec<State> {
	let mut lut = std::iter::repeat(State::Dead)
		.take(1 << 5)
		.collect::<Vec<_>>();
	for rule in rules {
		let mut index = 0usize;
		for cell in rule.pattern.iter().rev() {
			index = (index << 1) | *cell as usize;
		}
		lut[index] = rule.result;
	}
	lut
}

fn index_list(states: &[State]) -> Vec<i64> {
	let mut list = Vec::new();
	for (idx, state) in states.iter().enumerate() {
		if *state == State::Alive {
			list.push(idx as i64);
		}
	}
	list
}

fn simulate_generation(lut: &[State], last: &[i64], current: &mut Vec<i64>) {
	use self::State::*;
	let mut last_iter = multipeek(last.iter());
	let mut x = 0i64;
	let mut neighborhood = 0u8;
	loop {
		if neighborhood == 0 {
			let next = last_iter.peek();
			if next.is_none() {
				break;
			} else {
				x = **next.unwrap() - 2;
			}
			last_iter.reset_peek();
		}

		if last_iter.peek() == Some(&&(x + 2)) || last_iter.peek() == Some(&&(x + 2)) {
			neighborhood |= 1u8 << 4;
		}
		last_iter.reset_peek();

		if lut[neighborhood as usize] == Alive {
			current.push(x);
		}

		x += 1;
		neighborhood = neighborhood.overflowing_shr(1).0;

		let advance_iter = {
			let next = last_iter.peek();
			next.is_some() && **next.unwrap() <= x
		};
		if advance_iter {
			last_iter.next();
		}
		last_iter.reset_peek();
	}
}

#[aoc(day12, part1)]
pub fn part_1(input: &PartIn) -> i64 {
	let lut = rule_lut(&input.rules);
	let mut last = index_list(&input.initial_state);
	let mut current = Vec::new();
	for _ in 0..20 {
		simulate_generation(&lut, &last, &mut current);
		std::mem::swap(&mut last, &mut current);
		current.clear();
	}
	last.iter().sum::<i64>()
}

fn is_same_shifted(last: &[i64], current: &[i64]) -> bool {
	if last.len() != current.len() {
		return false;
	}
	last.iter().zip(current.iter()).all(|(a, b)| *a + 1 == *b)
}

#[aoc(day12, part2)]
pub fn part_2(input: &PartIn) -> i64 {
	let lut = rule_lut(&input.rules);
	let mut last = index_list(&input.initial_state);
	let mut current = Vec::new();
	let mut generation = 0i64;
	loop {
		simulate_generation(&lut, &last, &mut current);
		if is_same_shifted(&last, &current) {
			break;
		}
		generation += 1;
		std::mem::swap(&mut last, &mut current);
		current.clear();
	}
	last.iter().sum::<i64>() + last.len() as i64 * (50_000_000_000i64 - generation)
}

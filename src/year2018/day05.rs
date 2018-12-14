use rayon::prelude::*;

fn opposite_units(a: u8, b: u8) -> bool {
	if a.is_ascii_lowercase() {
		a.to_ascii_uppercase() == b
	} else {
		a.to_ascii_lowercase() == b
	}
}

fn collapse_length<'a, I>(polymer: I) -> usize
where
	I: Iterator<Item = &'a u8>,
{
	let mut stack = Vec::new();
	for unit in polymer {
		if stack
			.last()
			.map_or(false, |last| opposite_units(*last, *unit))
		{
			stack.pop();
		} else {
			stack.push(*unit);
		}
	}
	stack.len()
}

#[aoc(day5, part1)]
pub fn part_1(input: &[u8]) -> usize {
	collapse_length(input.iter())
}

#[aoc(day5, part2)]
pub fn part_2(input: &[u8]) -> usize {
	#[allow(clippy::range_plus_one)]
	(b'A'..(b'Z' + 1))
		.into_par_iter()
		.map(|deletion| {
			collapse_length(input.iter().filter(|u| !u.eq_ignore_ascii_case(&deletion)))
		})
		.min()
		.unwrap()
}

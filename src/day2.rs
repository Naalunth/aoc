type GeneratorOut = Vec<String>;

use std::collections::{HashMap};

#[aoc_generator(day2)]
pub fn gen(input: &str) -> GeneratorOut {
	input.lines().map(|l| l.to_owned()).collect::<Vec<_>>()
}

#[aoc(day2, part1)]
pub fn p1(input: &GeneratorOut) -> u32 {
	let (twos, threes) = input.iter()
		.map(|id| {
			let mut two = false;
			let mut three = false;
			for &n in id.chars()
				.fold(HashMap::new(), |mut map, c| {
					*map.entry(c).or_insert_with(|| 0u32) += 1;
					map
				})
				.values()
			{
				two = two || n == 2;
				three = three || n == 3;
				if two && three {break;}
			}
			(if two {1} else {0}, if three {1} else {0})
		})
		.fold((0u32, 0u32), |(c2, c3), (n2, n3)| {
			(c2 + n2, c3 + n3)
		});
	twos * threes
}

#[aoc(day2, part2)]
pub fn p2(input: &GeneratorOut) -> String {
	// sorting is a kinda ok-ish heuristic, as the pair will have a
	// good chance of landing next to each other
	let mut sorted = input.to_vec();
	sorted.sort();
	let (a, b) = (0..sorted.len())
		.flat_map(|i|
			sorted[i..].iter()
				.zip(&sorted)
		)
		.find(|(a, b)| differ_by_one(a, b))
		.unwrap();
	common_chars(a, b)
}

fn differ_by_one(a: &str, b: &str) -> bool {
	let mut iter = a.chars()
		.zip(b.chars())
		.filter(|(ac, bc)| ac != bc);
	iter.next().is_some() && iter.next().is_none()
}

fn common_chars(a: &str, b: &str) -> String {
	a.chars()
		.zip(b.chars())
		.filter_map(|(ac, bc)| if ac == bc {
			Some(ac)
		} else {
			None
		})
		.collect::<String>()
}

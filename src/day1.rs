type GeneratorOut = Vec<i64>;
type PartIn = [i64];

#[aoc_generator(day1)]
pub fn gen(input: &str) -> GeneratorOut {
	input
		.lines()
		.map(|l| l.parse::<i64>().unwrap())
		.collect::<Vec<_>>()
}

#[aoc(day1, part1)]
pub fn p1(input: &PartIn) -> i64 {
	input.iter().sum::<i64>()
}

#[aoc(day1, part2)]
pub fn p2(input: &PartIn) -> i64 {
	use std::collections::{HashMap, HashSet};

	let mut freqs = std::iter::once(0)
		.chain(input.iter().scan(0, |state, &i| {
			*state += i;
			Some(*state)
		}))
		.collect::<Vec<_>>();

	if let Some(repetition) = freqs
		.iter()
		.scan(HashSet::new(), |state, &current_frequency| {
			Some(if state.insert(current_frequency) {
				None
			} else {
				Some(current_frequency)
			})
		})
		.filter_map(|x| x)
		.nth(0)
	{
		return repetition;
	}

	let freq_shift = freqs.pop().unwrap();
	if freq_shift == 0 {
		return 0;
	}

	let mut groups = HashMap::new();
	for (i, &freq) in freqs.iter().enumerate() {
		groups
			.entry(freq % freq_shift)
			.or_insert_with(Vec::<(usize, i64)>::new)
			.push((i, freq));
	}

	let mut min_diff: Option<i64> = None;
	let mut min_freq = 0i64;
	let mut min_index = 0usize;

	for group in groups.values_mut() {
		group.sort_unstable_by_key(|e| e.1);
		for i in 1..group.len() {
			let diff = group[i].1 - group[i - 1].1;
			let index = if freq_shift > 0 {
				group[i - 1].0
			} else {
				group[i].0
			};
			let freq = if freq_shift > 0 {
				group[i].1
			} else {
				group[i - 1].1
			};
			if min_diff.is_none()
				|| diff < min_diff.unwrap()
				|| (diff == min_diff.unwrap() && index < min_index)
			{
				min_diff = Some(diff);
				min_freq = freq;
				min_index = index;
			}
		}
	}

	min_freq
}

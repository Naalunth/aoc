type GeneratorOut = Vec<u32>;

use smallvec::SmallVec;

#[aoc_generator(day8)]
pub fn generator(input: &str) -> GeneratorOut {
	input.split_terminator(|c: char| c.is_ascii_whitespace())
		.map(|s| s.parse::<u32>().unwrap())
		.collect()
}

#[aoc(day8, part1)]
pub fn part_1(input: &GeneratorOut) -> u32 {
	fn walk_node<'a, I: Iterator<Item = &'a u32>>(iter: &mut I) -> u32 {
		let child_count = *iter.next().unwrap();
		let meta_count = *iter.next().unwrap();
		(0..child_count)
			.map(|_| walk_node(iter))
			.sum::<u32>() +
		(0..meta_count)
			.map(|_| *iter.next().unwrap())
			.sum::<u32>()
	};
	walk_node(&mut input.iter())
}

#[aoc(day8, part2)]
pub fn part_2(input: &GeneratorOut) -> u32 {
	fn walk_node<'a, I: Iterator<Item = &'a u32>>(iter: &mut I) -> u32 {
		let child_count = *iter.next().unwrap();
		let meta_count = *iter.next().unwrap();
		if child_count != 0 {
			let children = (0..child_count)
				.map(|_| walk_node(iter))
				.collect::<SmallVec<[u32; 128]>>();
			(0..meta_count)
				.map(|_| *iter.next().unwrap())
				.filter_map(|meta| if meta == 0 {None} else {Some(meta-1)})
				.filter_map(|meta| {
					children.get(meta as usize)
				})
				.sum::<u32>()
		} else {
			(0..meta_count)
				.map(|_| *iter.next().unwrap())
				.sum::<u32>()
		}
	};
	walk_node(&mut input.iter())
}

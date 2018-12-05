fn opposite_units(a: u8, b: u8) -> bool {
	if a.is_ascii_lowercase() {
		a.to_ascii_uppercase() == b
	} else {
		a.to_ascii_lowercase() == b
	}
}

fn collapse_length(polymer: &[u8]) -> usize {
	let mut stack = Vec::new();
	for unit in polymer {
		if stack.last().map_or(false, |last| opposite_units(*last, *unit)) {
			stack.pop();
		} else {
			stack.push(*unit);
		}
	}
	stack.len()
}

#[aoc(day5, part1)]
pub fn part_1(input: &[u8]) -> usize {
	let copy = input.iter().filter(|c| u8::is_ascii_alphabetic(c)).cloned().collect::<Vec<_>>();
	collapse_length(&copy)
}

#[aoc(day5, part2)]
pub fn part_2(input: &[u8]) -> usize {
	(b'A'..=b'Z').map(|deletion| {
		let copy = input.iter()
			.filter(|c| c.is_ascii_alphabetic() && !c.eq_ignore_ascii_case(&deletion))
			.cloned()
			.collect::<Vec<_>>();
		collapse_length(&copy)
	}).min().unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	proptest! {
		#[test]
		fn should_be_opposites(s in "[a-z]") {
			let c = s.as_bytes()[0];
			prop_assert!(opposite_units(c, c.to_ascii_uppercase()));
		}
	}
}

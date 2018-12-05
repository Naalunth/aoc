fn opposite_units(a: u8, b: u8) -> bool {
	if a.is_ascii_lowercase() {
		a.to_ascii_uppercase() == b
	} else {
		a.to_ascii_lowercase() == b
	}
}

fn collapsed_length(polymer: &[u8]) -> usize {
	let mut copy = polymer.to_vec();
	let mut i = 1;
	let mut removed_one = false;
	loop {
		while i < copy.len() {
			if opposite_units(copy[i-1], copy[i]) {
				copy.remove(i);
				copy.remove(i-1);
				removed_one = true;
			} else {
				i += 1;
			}
		}
		if !removed_one {
			break;
		} else {
			i = 1;
			removed_one = false;
		}
	}
	copy.len()
}

#[aoc(day5, part1)]
pub fn part_1(input: &[u8]) -> usize {
	let copy = input.iter().filter(|c| u8::is_ascii_alphabetic(c)).cloned().collect::<Vec<_>>();
	collapsed_length(&copy)
}

#[aoc(day5, part2)]
pub fn part_2(input: &[u8]) -> usize {
	(b'A'..=b'Z').map(|deletion| {
		let copy = input.iter()
			.filter(|c| c.is_ascii_alphabetic() && !c.eq_ignore_ascii_case(&deletion))
			.cloned()
			.collect::<Vec<_>>();
		collapsed_length(&copy)
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

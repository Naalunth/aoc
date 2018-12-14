struct ChocolateChart {
	list: Vec<u8>,
	next: usize,
	indices: [usize; 2],
}

impl ChocolateChart {
	fn new() -> Self {
		ChocolateChart {
			list: vec![3, 7],
			next: 0,
			indices: [0, 1],
		}
	}

	fn with_capacity(cap: usize) -> Self {
		let mut list = Vec::with_capacity(cap);
		list.push(3);
		list.push(7);
		ChocolateChart {
			list,
			next: 0,
			indices: [0, 1],
		}
	}
}

impl Iterator for ChocolateChart {
	type Item = u8;
	fn next(&mut self) -> Option<u8> {
		if self.next >= self.list.len() {
			let new_number = self.list[self.indices[0]] + self.list[self.indices[1]];
			if new_number / 10 != 0 {
				self.list.push(new_number / 10);
			}
			self.list.push(new_number % 10);
			for idx in self.indices.iter_mut() {
				*idx = (*idx + 1 + self.list[*idx] as usize) % self.list.len();
			}
		}
		let result = self.list[self.next];
		self.next += 1;
		Some(result)
	}
}

#[aoc(day14, part1)]
pub fn part_1(input: &str) -> String {
	let input = input.parse::<usize>().unwrap();
	ChocolateChart::with_capacity(input + 10)
		.skip(input)
		.take(10)
		.map(|n| n.to_string())
		.collect::<String>()
}

fn generate_partial_match_table(w: &[u8]) -> Vec<isize> {
	let mut t = Vec::with_capacity(w.len());
	let mut cnd = 0;
	t.push(-1isize);
	for idx in 1..w.len() {
		if w[idx] == w[cnd as usize] {
			t.push(t[cnd as usize]);
		} else {
			t.push(cnd);
			cnd = t[cnd as usize];
			while cnd >= 0 && w[idx] != w[cnd as usize] {
				cnd = t[cnd as usize];
			}
		}
		cnd += 1;
	}
	t
}

#[aoc(day14, part2)]
pub fn part_2(input: &str) -> usize {
	let input = input
		.chars()
		.map(|c| c.to_digit(10).unwrap() as u8)
		.collect::<Vec<_>>();
	let pmt = generate_partial_match_table(&input);
	let mut iter = ChocolateChart::new().enumerate();
	let mut current = iter.next().unwrap();
	let mut search_idx = 0isize;
	loop {
		if input[search_idx as usize] == current.1 {
			search_idx += 1;
			current = iter.next().unwrap();
			if search_idx as usize == input.len() {
				return current.0 - input.len();
			}
		} else {
			search_idx = pmt[search_idx as usize];
			if search_idx < 0 {
				search_idx += 1;
				current = iter.next().unwrap();
			}
		}
	}
}

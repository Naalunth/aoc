use smallvec::SmallVec;

#[derive(Debug, Copy, Clone)]
enum ChocolateChartIndex {
	InitialListIndex(usize),
	BigListIndex(usize),
}

#[derive(Debug)]
struct InitialListElement {
	index: ChocolateChartIndex,
	value: u8,
}

#[derive(Debug)]
struct ChocolateChart {
	pub list: Vec<u8>,
	next: usize,
	last_list_index: usize,
	length: usize,
	indices: [ChocolateChartIndex; 2],
	bonus_number: Option<u8>,
}

impl ChocolateChart {
	fn new() -> Self {
		ChocolateChart {
			list: vec![7],
			next: 0,
			last_list_index: 23,
			length: 24,
			indices: [
				ChocolateChartIndex::InitialListIndex(4),
				ChocolateChartIndex::InitialListIndex(12),
			],
			bonus_number: None,
		}
	}
}

impl Iterator for ChocolateChart {
	type Item = u8;
	fn next(&mut self) -> Option<u8> {
		use self::ChocolateChartIndex::*;
		use self::InitialListElement as Ile;
		const INITIAL_LIST: [InitialListElement; 16] = [
			Ile { index: InitialListIndex(4), value: 3 },  //  0
			Ile { index: InitialListIndex(9), value: 7 },  //  1
			Ile { index: InitialListIndex(4), value: 1 },  //  2
			Ile { index: InitialListIndex(4), value: 0 },  //  3
			Ile { index: InitialListIndex(6), value: 1 },  //  4
			Ile { index: InitialListIndex(6), value: 0 },  //  5
			Ile { index: InitialListIndex(8), value: 1 },  //  6
			Ile { index: InitialListIndex(10), value: 2 }, //  7
			Ile { index: InitialListIndex(12), value: 4 }, //  8
			Ile { index: InitialListIndex(13), value: 5 }, //  9
			Ile { index: InitialListIndex(11), value: 1 }, // 10
			Ile { index: InitialListIndex(14), value: 8 }, // 11
			Ile { index: BigListIndex(0), value: 9 },      // 12
			Ile { index: InitialListIndex(15), value: 6 }, // 13
			Ile { index: BigListIndex(0), value: 1 },      // 14
			Ile { index: BigListIndex(0), value: 0 },      // 15
		];
		const FIRST_ELEMENTS: [u8; 24] = [3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9, 2, 5, 1, 0, 7];
		if self.next < 24 {
			let result = FIRST_ELEMENTS[self.next];
			self.next += 1;
			return Some(result);
		}
		if self.bonus_number.is_some() {
			let result = self.bonus_number.unwrap();
			self.bonus_number = None;
			self.next += 1;
			return Some(result);
		}

		//println!("{:?}", self);
		let new_number = self.indices.iter()
			.map(|idx| match idx {
				InitialListIndex(i) => INITIAL_LIST[*i].value,
				BigListIndex(i) => self.list[*i],
			})
			.sum::<u8>();

		let mut new_digits = SmallVec::<[u8; 2]>::new();

		if new_number / 10 != 0 {
			new_digits.push(new_number / 10);
		}
		new_digits.push(new_number % 10);

		for digit in new_digits.iter() {
			if self.last_list_index + *self.list.last().unwrap() as usize + 1 == self.length {
				self.list.push(*digit);
				self.last_list_index = self.length;
			}
			self.length += 1;
		}

		if new_digits.len() == 2 {
			self.bonus_number = Some(new_digits[1]);
		}
		let result = new_digits[0];

		for idx in self.indices.iter_mut() {
			*idx = match idx {
				InitialListIndex(i) => INITIAL_LIST[*i].index,
				BigListIndex(i) => {
					if *i + 1 >= self.list.len() {
						InitialListIndex((self.last_list_index + self.list[*i] as usize + 1) % self.length)
					} else {
						BigListIndex(*i + 1)
					}
				},
			};
		}

		self.next += 1;
		Some(result)
	}
}

#[aoc(day14, part1)]
pub fn part_1(input: &str) -> String {
	let input = input.parse::<usize>().unwrap();
	ChocolateChart::new()
		//.inspect(|num| println!("output: {}", num))
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

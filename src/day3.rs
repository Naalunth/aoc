type GeneratorOut = Vec<Claim>;

use nom::types::CompleteStr;

#[derive(Debug)]
pub struct Claim {
	pub id: u32,
	pub left: u32,
	pub top: u32,
	pub width: u32,
	pub height: u32
}

named!(number <CompleteStr, u32>, map!(take_while1!(|c| char::is_ascii_digit(&c)), |s| s.parse::<u32>().unwrap()));

named!(parse_claim <CompleteStr, Claim>, do_parse!(
	tag!("#") >>
	id: number >>
	tag!(" @ ") >>
	left: number >>
	tag!(",") >>
	top: number >>
	tag!(": ") >>
	width: number >>
	tag!("x") >>
	height: number >>
	(Claim {id, left, top, width, height})
));

#[aoc_generator(day3)]
pub fn generator(input: &str) -> GeneratorOut {
	input.lines().map(|l| parse_claim(CompleteStr(l)).unwrap().1).collect::<Vec<_>>()
}

#[derive(Debug)]
struct QuadTree<T> {
	root: Node<T>,
	levels: u32
}

#[derive(Debug)]
enum Node<T> {
	Data(T),
	Subdivision(Box<[Node<T>; 4]>)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Bounds {
	pub left: u32,
	pub top: u32,
	pub width: u32,
	pub height: u32
}

trait AreaBounds {
	fn bounds(&self) -> Bounds;

	fn contains(&self, other: &AreaBounds) -> bool {
		let a = self.bounds();
		let b = other.bounds();
		a.left <= b.left
			&& a.top <= b.top
			&& a.right() >= b.right()
			&& a.bottom() >= b.bottom()
	}

	fn intersects(&self, other: &AreaBounds) -> bool {
		let a = self.bounds();
		let b = other.bounds();
		a.left <= b.right() && a.right() >= b.left
			&& a.top <= b.bottom() && a.bottom() >= b.top
	}
}

impl<T> QuadTree<T> {
	/// Constructs a quadtree of size (2^`subdivision_levels`)^2.
	fn new(subdivision_levels: u32, initial_value: T) -> Self {
		QuadTree{
			root: Node::Data(initial_value),
			levels: subdivision_levels
		}
	}

	fn map_and_merge<F, B: AreaBounds>(&mut self, bounds: &B, op: F) where
		T: PartialEq + Clone,
		F: Fn(&T) -> T {
		use self::Node::*;
		let size = 1 << self.levels;
		let target = bounds.bounds();

		fn walk_node<T, F>(node: &mut Node<T>, own_bounds: &Bounds, target: &Bounds, op: &F) where
			T: PartialEq + Clone,
			F: Fn(&T) -> T {
			if own_bounds.intersects(target) {
				if !target.contains(own_bounds) {
					node.split();
				}

				let mut merge = None;

				match node {
					Data(ref mut data) => *data = op(data),
					Subdivision(ref mut nodes) => {
						for (n, q) in nodes.iter_mut().zip(&own_bounds.quadrants()) {
							walk_node(n, q, target, op);
						}
						let mut it = nodes.iter()
							.map(|n| if let Data(ref data) = n {Some(data)} else {None});
						let first = it.next().unwrap();
						if first.is_some() && it.all(|n| n.is_some() && *n.unwrap() == *first.unwrap()) {
							merge = Some(first.unwrap().clone());
						}
					}
				}

				if merge.is_some() {
					*node = Data(merge.unwrap());
				}
			}
		}

		let root_bounds = Bounds {
			left: 0,
			top: 0,
			width: size,
			height: size
		};

		walk_node(&mut self.root, &root_bounds, &target, &op);
	}

	fn area_if<F>(&self, op: F) -> u32 where
		F: Fn(&T) -> bool {
		use self::Node::*;
		let size = 1 << self.levels;

		fn walk_node<T, F>(node: &Node<T>, own_bounds: &Bounds, op: &F) -> u32 where
			F: Fn(&T) -> bool {
			match node {
				Data(data) => if op(data) {own_bounds.width * own_bounds.height} else {0},
				Subdivision(nodes) => {
					nodes.iter()
						.zip(&own_bounds.quadrants())
						.map(|(n, q)| walk_node(n, q, op))
						.sum()
				}
			}
		}

		let root_bounds = Bounds {
			left: 0,
			top: 0,
			width: size,
			height: size
		};

		walk_node(&self.root, &root_bounds, &op)
	}

	fn find_if<F, B: AreaBounds>(&self, bounds: &B, op: F) -> Option<&T> where
		F: Fn(&T) -> bool {
		use self::Node::*;
		let size = 1 << self.levels;
		let target = bounds.bounds();

		fn walk_node<'a, T, F>(node: &'a Node<T>, own_bounds: &Bounds, target: &Bounds, op: &F) -> Option<&'a T> where
			F: Fn(&T) -> bool {
			if own_bounds.intersects(target) {
				match node {
					Data(data) => if op(data) {Some(data)} else {None},
					Subdivision(nodes) => {
						nodes.iter()
							.zip(&own_bounds.quadrants())
							.map(|(n, q)| walk_node(n, q, target, op))
							.filter_map(|x| x)
							.nth(0)
					}
				}
			} else {
				None
			}
		}

		let root_bounds = Bounds {
			left: 0,
			top: 0,
			width: size,
			height: size
		};

		walk_node(&self.root, &root_bounds, &target, &op)
	}
}

impl<T> Node<T> {
	fn split(&mut self) where T: Clone {
		use self::Node::*;
		let data_opt = if let Data(data) = self {
			Some(data.clone())
		} else {
			None
		};
		if let Some(data) = data_opt {
			let a = Box::new([
				Data(data.clone()),
				Data(data.clone()),
				Data(data.clone()),
				Data(data)
			]);
			*self = Subdivision(a);
		}
	}
}

impl Bounds {
	fn right(&self) -> u32 {
		self.left + self.width - 1
	}

	fn bottom(&self) -> u32 {
		self.top + self.height - 1
	}

	fn quadrants(&self) -> [Bounds; 4] {
		let half_width = self.width / 2;
		let half_height = self.width / 2;
		let nw = Bounds {
			left: self.left,
			top: self.top,
			width: half_width,
			height: half_height
		};
		let ne = Bounds {
			left: self.left + half_width,
			top: self.top,
			width: half_width,
			height: half_height
		};
		let sw = Bounds {
			left: self.left,
			top: self.top + half_height,
			width: half_width,
			height: half_height
		};
		let se = Bounds {
			left: self.left + half_width,
			top: self.top + half_height,
			width: half_width,
			height: half_height
		};
		[nw, ne, sw, se]
	}
}

impl AreaBounds for Bounds {
	fn bounds(&self) -> Bounds {
		self.clone()
	}
}

impl AreaBounds for Claim {
	fn bounds(&self) -> Bounds {
		Bounds {
			left: self.left,
			top: self.top,
			width: self.width,
			height: self.height
		}
	}
}

#[aoc(day3, part1)]
pub fn part_1(input: &GeneratorOut) -> u32 {
	let mut qt = QuadTree::new(10, 0u8);
	for claim in input {
		qt.map_and_merge(claim, |count| u8::min(count + 1, 2));
	}
	qt.area_if(|count| *count == 2)
}

#[aoc(day3, part2)]
pub fn part_2(input: &GeneratorOut) -> u32 {
	let mut qt = QuadTree::new(10, 0u8);
	for claim in input {
		qt.map_and_merge(claim, |count| u8::min(count + 1, 2));
	}
	input.iter()
		.filter(|&claim| qt.find_if(claim, |count| *count == 2).is_none())
		.nth(0)
		.unwrap().id
}

#[cfg(test)]
mod tests {
	use super::*;
	use proptest::prelude::*;

	fn bounds_strategy(left: impl Strategy<Value = u32>, top: impl Strategy<Value = u32>, width: impl Strategy<Value = u32>, height: impl Strategy<Value = u32>) -> impl Strategy<Value = Bounds> {
		(left, top, width, height)
			.prop_map(|(left, top, width, height)| Bounds {left, top, width, height})
	}

	fn intersecting_bounds() -> impl Strategy<Value = (Bounds, Bounds)> {
		bounds_strategy(0u32..16, 0u32..16, 1u32..16, 1u32..16)
			.prop_flat_map(|a| {
				let b = bounds_strategy(
					a.left..=a.right(),
					a.top..=a.bottom(),
					1u32..16,
					1u32..16
				).boxed()
				.prop_union(
					(a.top..=a.bottom())
						.prop_flat_map(|bottom| (Just(bottom), 1u32..=bottom+1))
						.prop_flat_map(move |(bottom, height)| bounds_strategy(
							a.left..=a.right(),
							Just(bottom + 1 - height),
							1u32..16,
							Just(height)
						)).boxed()
				);
				(Just(a), b.clone()).boxed()
					.prop_union((b, Just(a)).boxed())
			})
	}

	proptest! {
		#[test]
		fn should_intersect((a, b) in intersecting_bounds()) {
			prop_assert!(a.intersects(&b));
		}
	}

	#[test]
	fn bounds_contain() {
		let a = Bounds {
			left: 1,
			top: 1,
			width: 3,
			height: 3
		};
		let b = Bounds {
			left: 1,
			top: 1,
			width: 2,
			height: 2
		};
		assert!(a.contains(&a));
		assert!(a.contains(&b));
		assert!(!b.contains(&a));
	}

	#[test]
	fn bounds_quadrants() {
		let a = Bounds {
			left: 4,
			top: 4,
			width: 4,
			height: 4
		};
		assert_eq!(a.quadrants(), [
			Bounds {
				left: 4,
				top: 4,
				width: 2,
				height: 2
			},
			Bounds {
				left: 6,
				top: 4,
				width: 2,
				height: 2
			},
			Bounds {
				left: 4,
				top: 6,
				width: 2,
				height: 2
			},
			Bounds {
				left: 6,
				top: 6,
				width: 2,
				height: 2
			}
		]);
	}
}

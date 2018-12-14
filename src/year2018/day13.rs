type GeneratorOut = Input;
type PartIn = GeneratorOut;

use nalgebra::{Point2, Vector2};
use ndarray::Array2;
use std::convert::AsRef;

#[derive(Debug, Copy, Clone, PartialEq)]
enum MapElement {
	Empty,
	Horizontal,
	Vertical,
	MainDiagonal,
	AntiDiagonal,
	Crossing,
}

impl Default for MapElement {
	fn default() -> Self {
		MapElement::Empty
	}
}

#[derive(Debug, Copy, Clone)]
enum TurningDirection {
	Left,
	Straight,
	Right,
}

#[derive(Debug, Copy, Clone)]
struct Cart {
	pub position: Point2<i16>,
	pub direction: Vector2<i16>,
	pub next_turn: TurningDirection,
}

#[derive(Debug, Clone)]
pub struct Input {
	map: Array2<MapElement>,
	carts: Vec<Cart>,
}

impl AsRef<Input> for Input {
	fn as_ref(&self) -> &Input {
		self
	}
}

#[aoc_generator(day13)]
pub fn generator(input: &[u8]) -> GeneratorOut {
	use self::MapElement::*;
	let mut map = Vec::new();
	let mut carts = Vec::new();
	for (y, line) in input
		.split(|c| *c == b'\n')
		.filter(|l| !l.is_empty())
		.enumerate()
	{
		for (x, element) in line.iter().enumerate() {
			map.push(match element {
				b' ' => MapElement::Empty,
				b'-' | b'<' | b'>' => Horizontal,
				b'|' | b'^' | b'v' => Vertical,
				b'\\' => MainDiagonal,
				b'/' => AntiDiagonal,
				b'+' => Crossing,
				_ => unreachable!(),
			});

			if let Some(direction) = match element {
				b'<' => Some(Vector2::new(-1, 0)),
				b'>' => Some(Vector2::new(1, 0)),
				b'^' => Some(Vector2::new(0, -1)),
				b'v' => Some(Vector2::new(0, 1)),
				_ => None,
			} {
				carts.push(Cart {
					position: Point2::new(x as i16, y as i16),
					direction,
					next_turn: TurningDirection::Left,
				});
			}
		}
	}
	let y_size = input
		.split(|c| *c == b'\n')
		.filter(|l| !l.is_empty())
		.count();
	let x_size = input
		.split(|c| *c == b'\n')
		.find(|l| !l.is_empty())
		.unwrap()
		.iter()
		.count();
	let map = Array2::from_shape_vec((x_size, y_size), map)
		.unwrap()
		.t()
		.to_owned();
	Input { map, carts }
}

impl Cart {
	fn change_direction(&mut self, track: MapElement) {
		use self::{MapElement::*, TurningDirection::*};
		self.direction = match track {
			Empty => unreachable!(),
			Horizontal | Vertical => self.direction,
			MainDiagonal => Vector2::new(self.direction.y, self.direction.x),
			AntiDiagonal => Vector2::new(-self.direction.y, -self.direction.x),
			Crossing => {
				let (dir, next) = match self.next_turn {
					Left => (Vector2::new(self.direction.y, -self.direction.x), Straight),
					Straight => (self.direction, Right),
					Right => (Vector2::new(-self.direction.y, self.direction.x), Left),
				};
				self.next_turn = next;
				dir
			},
		}
	}
}

#[aoc(day13, part1)]
pub fn part_1(input: &PartIn) -> String {
	let map = &input.map;
	let mut carts = input.carts.clone();
	'collision_search: loop {
		carts.sort_by_key(|cart| (cart.position.y, cart.position.x));
		for idx in 0..carts.len() {
			let new_position = carts[idx].position + carts[idx].direction;
			if carts.iter().any(|&cart| cart.position == new_position) {
				break 'collision_search format!("{},{}", new_position.x, new_position.y);
			}
			let cart = &mut carts[idx];
			cart.position = new_position;
			cart.change_direction(map[(cart.position.x as usize, cart.position.y as usize)]);
		}
	}
}

#[aoc(day13, part2)]
pub fn part_2(input: &PartIn) -> String {
	let map = &input.map;
	let mut carts = input.carts.clone();
	let mut removed = Vec::new();
	loop {
		carts.sort_by_key(|cart| (cart.position.y, cart.position.x));
		let mut idx = 0;
		while idx < carts.len() {
			let new_position = carts[idx].position + carts[idx].direction;
			let possible_crash = carts
				.iter()
				.enumerate()
				.find(|&(idx, cart)| !removed.contains(&idx) && cart.position == new_position)
				.map(|(idx, _)| idx);
			if let Some(other) = possible_crash {
				removed.push(idx);
				removed.push(other);
				continue;
			}
			let cart = &mut carts[idx];
			cart.position = new_position;
			cart.change_direction(map[(cart.position.x as usize, cart.position.y as usize)]);
			idx += 1;
		}
		removed.sort_by(|a, b| b.cmp(a));
		for idx in removed.iter() {
			carts.remove(*idx);
		}
		removed.clear();
		if carts.len() == 1 {
			let cart = carts[0];
			break format!("{},{}", cart.position.x, cart.position.y);
		}
	}
}

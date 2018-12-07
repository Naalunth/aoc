type GeneratorOut = Vec<Point>;

use std::ops::{Add, Sub};
use std::collections::{HashSet, HashMap};
use std::iter::repeat;

use nom::types::CompleteStr;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
	x: i64,
	y: i64
}

named!(signed <CompleteStr, i64>, map!(take_while1!(|c| char::is_ascii_digit(&c)), |s| s.parse::<i64>().unwrap()));

named!(parse_entry <CompleteStr, Point>, do_parse!(
	x: signed >>
	tag!(", ") >>
	y: signed  >>
	(Point {x, y})
));

#[aoc_generator(day6)]
pub fn generator(input: &str) -> GeneratorOut {
	input.lines().map(|l| parse_entry(CompleteStr(l)).unwrap().1).collect::<Vec<_>>()
}

impl Point {
	fn new(x: i64, y: i64) -> Self {
		Point{ x, y }
	}

	fn l1_distance(&self, other: &Point) -> i64 {
		i64::abs(self.x - other.x) + i64::abs(self.y - other.y)
	}
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


#[aoc(day6, part1)]
pub fn part_1(input: &GeneratorOut) -> u32 {
	let x_min = input.iter().map(|p| p.x).min().unwrap() - 1;
	let x_max = input.iter().map(|p| p.x).max().unwrap() + 1;
	let y_min = input.iter().map(|p| p.y).min().unwrap() - 1;
	let y_max = input.iter().map(|p| p.y).max().unwrap() + 1;
	let x_size = (x_max - x_min) as usize;
	let y_size = (y_max - y_min) as usize;
	let mut cells = repeat(repeat(None).take(x_size).collect::<Vec<_>>())
		.take(y_size).collect::<Vec<_>>();
	let offset = Point::new(x_min as i64, y_min as i64);

	cells.par_iter_mut().enumerate().for_each(|(y, row)| {
		row.par_iter_mut().enumerate().for_each(|(x, cell)|{
			let cell_point = Point::new(x as i64, y as i64) + offset;
			let mut current_min_point: Option<(Option<usize>, i64)> = None;
			for (id, point) in input.iter().enumerate() {
				let distance = point.l1_distance(&cell_point);
				if current_min_point.is_none() ||
					current_min_point.unwrap().1 > distance
				{
					current_min_point = Some((Some(id), distance));
				} else if current_min_point.unwrap().1 == distance {
					current_min_point = Some((None, distance));
				}
				*cell = current_min_point.unwrap().0;
			}
		});
	});

	let mut excluded_points = HashSet::new();

	// first row
	for cell in cells[0].iter() {
		for id in cell {
			excluded_points.insert(*id);
		}
	}

	// last row
	for cell in cells[y_size - 1].iter() {
		for id in cell {
			excluded_points.insert(*id);
		}
	}

	// first column
	for row in cells[1..y_size-1].iter() {
		for id in row[0] {
			excluded_points.insert(id);
		}
	}

	// last column
	for row in cells[1..y_size-1].iter() {
		for id in row[x_size - 1] {
			excluded_points.insert(id);
		}
	}

	let counts = cells.par_iter()
		.flat_map(|row| row.par_iter())
		.filter_map(|cell| *cell)
		.filter(|id| !excluded_points.contains(id))
		.fold(|| HashMap::new(), |mut map, id| {
			*map.entry(id).or_insert(0u32) += 1;
			map
		})
		.reduce(|| HashMap::new(), |mut a, b| {
			for (k, v) in b.iter() {
				*a.entry(*k).or_insert(0u32) += v;
			}
			a
		});

	*counts.values()
		.max()
		.unwrap()
}

#[aoc(day6, part2)]
pub fn part_2(input: &GeneratorOut) -> usize {
	let x_min = input.iter().map(|p| p.x).min().unwrap() - 1;
	let x_max = input.iter().map(|p| p.x).max().unwrap() + 1;
	let y_min = input.iter().map(|p| p.y).min().unwrap() - 1;
	let y_max = input.iter().map(|p| p.y).max().unwrap() + 1;
	let x_size = (x_max - x_min) as usize;
	let y_size = (y_max - y_min) as usize;
	const MAX_DISTANCE: u64 = 10000;
	let offset = Point::new(x_min as i64, y_min as i64);

	(0..y_size).into_par_iter().flat_map(|y| rayon::iter::repeat(y).zip((0..x_size).into_par_iter()))
		.map(|(y, x)| {
			let cell_point = Point::new(x as i64, y as i64) + offset;
			input.iter().map(|point| point.l1_distance(&cell_point)).sum::<i64>()
		})
		.filter(|sum| (*sum as u64) < MAX_DISTANCE)
		.count()
}

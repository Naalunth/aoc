#![allow(clippy::trivially_copy_pass_by_ref)]
type GeneratorOut = Rc<u32>;
type PartIn = u32;

use ndarray::{Array, Array2, Axis, Zip};
use rayon::prelude::*;
use std::rc::Rc;

#[aoc_generator(day11)]
pub fn generator(input: &str) -> GeneratorOut {
	Rc::new(
		input
			.lines()
			.map(|l| l.parse::<u32>().expect("input file should be valid"))
			.next()
			.unwrap(),
	)
}

fn hundreds_digit(n: u32) -> u32 {
	(n / 100) % 10
}

fn accumulate(array: &mut Array2<i32>) {
	array
		.axis_iter_mut(Axis(0))
		.into_par_iter()
		.for_each(|row| {
			let mut sum = 0;
			for item in row {
				sum += *item;
				*item = sum;
			}
		});
	array
		.axis_iter_mut(Axis(1))
		.into_par_iter()
		.for_each(|column| {
			let mut sum = 0;
			for item in column {
				sum += *item;
				*item = sum;
			}
		});
}

fn build_power_levels(serial_number: u32) -> Array2<i32> {
	let mut levels = Array::from_shape_fn((301, 301), |(x, y)| {
		if x == 0 || y == 0 {
			0
		} else {
			let rack_id = x as u32 + 10;
			hundreds_digit((rack_id * (y as u32) + serial_number) * rack_id) as i32 - 5
		}
	});
	accumulate(&mut levels);
	levels
}

fn largest_power(power_levels: &Array2<i32>, window_size: usize) -> ((usize, usize), i32) {
	let sum_array_size = 301 - window_size;
	let mut power_sums = Array::zeros((sum_array_size, sum_array_size));
	Zip::from(&mut power_sums)
		.and(power_levels.windows((window_size + 1, window_size + 1)))
		.par_apply(|sum, area| {
			*sum =
				area[(window_size, window_size)] - area[(0, window_size)] - area[(window_size, 0)]
					+ area[(0, 0)];
		});
	power_sums
		.indexed_iter()
		.max_by_key(|&(_, sum)| sum)
		.map(|((x, y), s)| ((x + 1, y + 1), *s))
		.unwrap()
}

#[aoc(day11, part1)]
pub fn part_1(serial_number: &PartIn) -> String {
	let power_levels = build_power_levels(*serial_number);
	let ((x, y), _) = largest_power(&power_levels, 3);
	format!("{},{}", x, y)
}

#[aoc(day11, part2)]
pub fn part_2(serial_number: &PartIn) -> String {
	let power_levels = build_power_levels(*serial_number);
	(1usize..301)
		.into_par_iter()
		.map(|window_size| (window_size, largest_power(&power_levels, window_size)))
		.max_by_key(|&(_, (_, area_sum))| area_sum)
		.map(|(window_size, ((x, y), _))| format!("{},{},{}", x, y, window_size))
		.unwrap()
}

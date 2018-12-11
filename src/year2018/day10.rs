type GeneratorOut = Vec<Light>;
type PartIn = [Light];

use nalgebra::{Point2, Vector2};
use nom::types::CompleteStr;

#[derive(Copy, Clone)]
pub struct Light {
	initial_position: Point2<i64>,
	velocity: Vector2<i64>,
}

named!(number <CompleteStr, u64>, map!(take_while1!(|c: char| c.is_ascii_digit()), |s| s.parse::<u64>().unwrap()));

named!(signed <CompleteStr, i64>, alt!(
	map!(preceded!(tag!("-"), number), |n: u64| -(n as i64))
	| map!(number, |n: u64| n as i64)
));

named!(parse_vec2 <CompleteStr, Vector2<i64>>, do_parse!(
	tag!("<") >>
	take_while!(|c: char| c.is_ascii_whitespace()) >>
	x: signed >>
	tag!(",") >>
	take_while!(|c: char| c.is_ascii_whitespace()) >>
	y: signed >>
	tag!(">") >>
	(Vector2::new(x, y))
));

named!(parse_light <CompleteStr, Light>, do_parse!(
	tag!("position=") >>
	position: parse_vec2 >>
	tag!(" velocity=") >>
	velocity: parse_vec2 >>
	(Light { initial_position: position.into(), velocity })
));

#[aoc_generator(day10)]
pub fn generator(input: &str) -> GeneratorOut {
	input
		.lines()
		.map(|l| {
			parse_light(CompleteStr(l))
				.expect("input file should be valid")
				.1
		})
		.collect::<Vec<_>>()
}

fn extent(points: &[Point2<i64>]) -> (i64, i64, i64, i64) {
	let x_max = points.iter().map(|p| p.x).max().unwrap();
	let x_min = points.iter().map(|p| p.x).min().unwrap();
	let y_max = points.iter().map(|p| p.y).max().unwrap();
	let y_min = points.iter().map(|p| p.y).min().unwrap();
	(x_max, x_min, y_max, y_min)
}

fn size(points: &[Point2<i64>]) -> (u64, u64) {
	let (x_max, x_min, y_max, y_min) = extent(points);
	((x_max + 1 - x_min) as u64, (y_max + 1 - y_min) as u64)
}

fn area(points: &[Point2<i64>]) -> u64 {
	let (x, y) = size(points);
	x * y
}

fn format_grid(points: &[Point2<i64>]) -> String {
	let (_x_max, x_min, _y_max, y_min) = extent(points);
	let (x_size, y_size) = size(points);
	let mut strings = std::iter::repeat_with(|| {
		std::iter::repeat(b'.')
			.take(x_size as usize)
			.collect::<Vec<_>>()
	})
	.take(y_size as usize)
	.collect::<Vec<_>>();
	let offset = Vector2::new(x_min, y_min);
	for point in points {
		let array_pos = point - offset;
		strings[array_pos.y as usize][array_pos.x as usize] = b'#';
	}
	String::from_utf8(strings.join(&b'\n')).unwrap()
}

#[aoc(day10, part1)]
pub fn part_1(input: &PartIn) -> String {
	let mut current_positions = input.iter().map(|l| l.initial_position).collect::<Vec<_>>();
	let mut min_area = area(&current_positions);
	let mut min_positions = current_positions.clone();
	loop {
		for (current_position, light) in current_positions.iter_mut().zip(input.iter()) {
			*current_position += light.velocity;
		}
		let area = area(&current_positions);
		if area > min_area {
			break;
		}
		min_area = area;
		min_positions = current_positions.clone();
	}
	format!("\n{}", format_grid(&min_positions))
}

#[aoc(day10, part2)]
pub fn part_2(input: &PartIn) -> u64 {
	let mut current_positions = input.iter().map(|l| l.initial_position).collect::<Vec<_>>();
	let mut min_area = area(&current_positions);
	let mut seconds = 0;
	loop {
		for (current_position, light) in current_positions.iter_mut().zip(input.iter()) {
			*current_position += light.velocity;
		}
		let area = area(&current_positions);
		if area > min_area {
			break;
		}
		min_area = area;
		seconds += 1;
	}
	seconds
}

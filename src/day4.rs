type GeneratorOut = Vec<Entry>;

use nom::types::CompleteStr;
use chrono::prelude::*;
use std::collections::HashMap;
use std::iter::repeat;

#[derive(Debug, Copy, Clone)]
pub struct Entry {
	timestamp: DateTime<Utc>,
	message: Message
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Message {
	GuardChange(u32),
	Wake,
	Sleep,
}

named!(number <CompleteStr, u32>, map!(take_while1!(|c| char::is_ascii_digit(&c)), |s| s.parse::<u32>().unwrap()));
named!(signed <CompleteStr, i32>, map!(take_while1!(|c| char::is_ascii_digit(&c)), |s| s.parse::<i32>().unwrap()));

named!(parse_entry <CompleteStr, Entry>, do_parse!(
	tag!("[") >>
	year: signed >>
	tag!("-") >>
	month: number >>
	tag!("-") >>
	day: number >>
	tag!(" ") >>
	hour: number >>
	tag!(":") >>
	minute: number >>
	tag!("] ") >>
	message: alt!(
		value!(Message::Wake, tag!("wakes up")) |
		value!(Message::Sleep, tag!("falls asleep")) |
		do_parse!(
			tag!("Guard #") >>
			id: number >>
			tag!(" begins shift") >>
			(Message::GuardChange(id))
		)
	) >>
	(Entry {
		timestamp: Utc.ymd(year, month, day).and_hms(hour, minute, 0),
		message: message
	})
));

#[aoc_generator(day4)]
pub fn generator(input: &str) -> GeneratorOut {
	input.lines().map(|l| parse_entry(CompleteStr(l)).unwrap().1).collect::<Vec<_>>()
}

fn calculate_minute_tables(input: &Vec<Entry>) -> HashMap<u32, [u32; 60]> {
	use self::Message::*;
	let sorted = {
		let mut copy = input.to_vec();
		copy.sort_by_key(|entry| entry.timestamp);
		copy
	};

	#[derive(PartialEq)]
	enum GuardState {
		Awake, Asleep
	}

	let mut guards = HashMap::<u32, [u32; 60]>::new();
	let mut current_guard: Option<u32> = None;
	let mut last_minute = 0;
	let mut last_state = GuardState::Awake;

	for entry in sorted {
		match entry.message {
			GuardChange(_) | Wake => {
				if current_guard.is_some() && last_state == GuardState::Asleep {
					let current_minute = if entry.message == Wake {
						entry.timestamp.minute() as usize - 1
					} else {
						59
					};
					for m in &mut guards.entry(current_guard.unwrap())
						.or_insert_with(|| [0; 60])[last_minute..current_minute]
					{
						*m += 1;
					}
				}
				last_state = GuardState::Awake;
			},
			Sleep => {
				last_state = GuardState::Asleep;
				last_minute = entry.timestamp.minute() as usize;
			}
		};
		if let GuardChange(guard) = entry.message {
			current_guard = Some(guard);
		}
	}

	guards
}


#[aoc(day4, part1)]
pub fn part_1(input: &GeneratorOut) -> u32 {
	calculate_minute_tables(input).iter()
		.max_by_key(|&(_, arr)| arr.iter().sum::<u32>())
		.and_then(|(id, arr)| arr.iter()
			.enumerate()
			.max_by_key(|&(_, times)| times)
			.map(|(day, _)| id * day as u32)
		)
		.unwrap()
}

#[aoc(day4, part2)]
pub fn part_2(input: &GeneratorOut) -> u32 {
	calculate_minute_tables(input).iter()
		.flat_map(|(id, arr)| {
			repeat(id)
			.zip(arr.iter())
			.enumerate()
		})
		.max_by_key(|&(_, (_, times))| times)
		.map(|(day, (id, _))| id * day as u32)
		.unwrap()
}

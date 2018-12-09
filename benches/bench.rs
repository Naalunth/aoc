#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
	{
		use std::{io::Read, rc::Rc};
		let mut file = std::fs::File::open("input/2018/day8.txt").unwrap();
		let mut input = Rc::new(String::new());
		file.read_to_string(Rc::get_mut(&mut input).unwrap())
			.unwrap();
		c.bench_function("year2018/day08/generator", move |b| {
			b.iter(|| aoc_naalunth::year2018::day08::generator(&input))
		});
	}
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

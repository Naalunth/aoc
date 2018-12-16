type GeneratorOut = Input;
type PartIn = GeneratorOut;

use indexmap::{
	map::Entry::{Occupied, Vacant},
	IndexMap,
	IndexSet,
};
use nalgebra::Point2;
use ndarray::Array2;
use smallvec::SmallVec;
use specs::{
	join::Join,
	storage::{DenseVecStorage, NullStorage},
	world::{Builder, World},
	Component,
	DispatcherBuilder,
	Entities,
	Read,
	ReadStorage,
	System,
	Write,
	WriteStorage,
};
use std::{
	cmp::Ordering,
	collections::{BinaryHeap, VecDeque},
	convert::AsRef,
};

#[derive(Debug, Copy, Clone, PartialEq)]
enum MapElement {
	Empty,
	Wall,
}

impl Default for MapElement {
	fn default() -> Self {
		MapElement::Empty
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum EntityClass {
	Elf,
	Goblin,
}

#[derive(Debug, Copy, Clone)]
struct Entity {
	pub class: EntityClass,
	pub position: Point2<u8>,
}

#[derive(Debug, Clone)]
pub struct Input {
	map: Array2<MapElement>,
	entities: Vec<Entity>,
}

impl AsRef<Input> for Input {
	fn as_ref(&self) -> &Input {
		self
	}
}

#[aoc_generator(day15)]
pub fn generator(input: &[u8]) -> GeneratorOut {
	use self::MapElement::*;
	let mut map = Vec::new();
	let mut entities = Vec::new();
	for (y, line) in input
		.split(|c| *c == b'\n')
		.filter(|l| !l.is_empty())
		.enumerate()
	{
		for (x, element) in line.iter().enumerate() {
			map.push(match element {
				b'.' | b'E' | b'G' => MapElement::Empty,
				b'#' => Wall,
				_ => unreachable!(),
			});

			if let Some(class) = match element {
				b'E' => Some(EntityClass::Elf),
				b'G' => Some(EntityClass::Goblin),
				_ => None,
			} {
				entities.push(Entity {
					class,
					position: Point2::new(x as u8, y as u8),
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
	Input { map, entities }
}

fn absdiff<T>(x: T, y: T) -> T
where
	T: std::ops::Sub<Output = T> + PartialOrd,
{
	if x < y {
		y - x
	} else {
		x - y
	}
}

fn format_map(map: &Array2<MapElement>, units: Vec<(Point2<u8>, char)>) -> String {
	let mut result = String::new();
	for y in 0..map.shape()[1] {
		for x in 0..map.shape()[0] {
			result.push(
				if let Some(letter) = units
					.iter()
					.find(|&(pos, _)| *pos == Point2::new(x as u8, y as u8))
					.map(|(_, l)| l)
				{
					*letter
				} else {
					match map[(x, y)] {
						MapElement::Empty => '.',
						MapElement::Wall => '#',
					}
				},
			)
		}
		result.push('\n');
	}
	result
}

#[derive(Component, Debug)]
struct Position(Point2<u8>);

#[derive(Component, Debug)]
struct Health(u8);

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
struct Elf;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
struct Goblin;

#[derive(Debug, Default)]
struct Map(Array2<MapElement>);

#[derive(Debug, Default)]
struct CombatStatus(bool);

#[derive(Debug, Default)]
struct Part2Info {
	elf_died: bool,
	elf_attack_damage: u8,
}

#[derive(Debug, Default)]
struct Round(u32);

struct SimulationSystem {
	map_with_units: Option<Array2<MapElement>>,
}
impl Default for SimulationSystem {
	fn default() -> Self {
		SimulationSystem {
			map_with_units: None,
		}
	}
}
impl<'a> System<'a> for SimulationSystem {
	type SystemData = (
		Read<'a, Map>,
		Write<'a, Round>,
		Write<'a, CombatStatus>,
		Option<Write<'a, Part2Info>>,
		Entities<'a>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Health>,
		ReadStorage<'a, Elf>,
		ReadStorage<'a, Goblin>,
	);

	fn run(&mut self, data: Self::SystemData) {
		use specs::Join;
		let (
			map,
			mut round,
			mut combat_status,
			mut part_2_info,
			entities,
			mut position_storage,
			mut health_storage,
			elf_storage,
			goblin_storage,
		) = data;
		let map = &map.0;

		let mut map_with_units = self.map_with_units.get_or_insert_with(|| {
			let mut map_with_units = map.clone();

			for Position(pos) in position_storage.join() {
				map_with_units[(pos.x as usize, pos.y as usize)] = MapElement::Wall;
			}
			map_with_units
		});

		let mut sorted = (
			&entities,
			&mut position_storage,
			&mut health_storage,
			elf_storage.maybe(),
			goblin_storage.maybe(),
		)
			.join()
			.map(|(entity, _, _, elf, goblin)| {
				(
					entity,
					if elf.is_some() {
						EntityClass::Elf
					} else if goblin.is_some() {
						EntityClass::Goblin
					} else {
						unreachable!()
					},
				)
			})
			.collect::<SmallVec<[(specs::Entity, EntityClass); 32]>>();

		sorted.sort_unstable_by_key(|&(ent, _)| {
			let Position(pos) = position_storage.get(ent).unwrap();
			(pos.y, pos.x)
		});

		fn neighbours(point: &Point2<u8>, map: &Array2<MapElement>) -> SmallVec<[Point2<u8>; 4]> {
			let mut candidates = SmallVec::new();
			if point.x > 0 {
				candidates.push(Point2::new(point.x - 1, point.y))
			}
			if point.x < map.shape()[0] as u8 - 1 {
				candidates.push(Point2::new(point.x + 1, point.y))
			}
			if point.y > 0 {
				candidates.push(Point2::new(point.x, point.y - 1))
			}
			if point.y < map.shape()[1] as u8 - 1 {
				candidates.push(Point2::new(point.x, point.y + 1))
			}
			candidates.retain(|candidate: &mut Point2<u8>| {
				map[(candidate.x as usize, candidate.y as usize)] == MapElement::Empty
			});
			candidates
		}

		let mut removed = Vec::<specs::Entity>::new();

		for (entity, class) in sorted.iter() {
			if removed.contains(&entity) {
				continue;
			}

			let Position(position) = position_storage.get(*entity).unwrap();
			map_with_units[(position.x as usize, position.y as usize)] = MapElement::Empty;
			// movement phase
			let targets: SmallVec<[Point2<u8>; 32]> = if *class == EntityClass::Elf {
				(&entities, &position_storage, &goblin_storage)
					.join()
					.filter(|(e, ..)| !removed.contains(&e))
					.map(|(_, &Position(pos), _)| pos)
					.collect()
			} else {
				(&entities, &position_storage, &elf_storage)
					.join()
					.filter(|(e, ..)| !removed.contains(&e))
					.map(|(_, &Position(pos), _)| pos)
					.collect()
			};

			if targets.is_empty() {
				for entity in removed {
					entities.delete(entity).unwrap();
				}
				combat_status.0 = false;
				return;
			}

			let target_neighbours: SmallVec<[Point2<u8>; 32]> = targets
				.into_iter()
				.flat_map(|target| neighbours(&target, &map_with_units).into_iter())
				.collect();

			if !target_neighbours.is_empty() && !target_neighbours.contains(position) {
				let chosen = {
					let mut queue = VecDeque::new();
					queue.push_back((0usize, 0u8));
					let mut visited: IndexSet<Point2<u8>> = IndexSet::new();
					visited.insert(*position);
					let mut closest_nodes = SmallVec::<[Point2<u8>; 4]>::new();
					let mut min_cost = None;
					while let Some((index, cost)) = queue.pop_front() {
						if let Some(lowest_cost) = min_cost {
							if cost > lowest_cost {
								break;
							}
						}
						let successors = {
							let node = visited.get_index(index).unwrap();
							if target_neighbours.contains(node) {
								closest_nodes.push(*node);
								min_cost = Some(cost);
							}
							neighbours(node, &map_with_units)
						};
						for successor in successors {
							let (index, newly_inserted) = visited.insert_full(successor);
							if newly_inserted {
								queue.push_back((index, cost + 1));
							}
						}
					}
					closest_nodes.into_iter().min_by_key(|pos| (pos.y, pos.x))
				};

				if let Some(chosen) = chosen {
					let own_neighbours = neighbours(position, &map_with_units);
					let new_position = if own_neighbours.len() == 1 {
						own_neighbours[0]
					} else {
						struct AstarElement {
							estimated_cost: u8,
							cost: u8,
							index: usize,
						}

						impl PartialEq for AstarElement {
							fn eq(&self, other: &AstarElement) -> bool {
								self.estimated_cost.eq(&other.estimated_cost)
									&& self.cost.eq(&other.cost)
							}
						}

						impl Eq for AstarElement {}

						impl PartialOrd for AstarElement {
							fn partial_cmp(&self, other: &AstarElement) -> Option<Ordering> {
								Some(self.cmp(other))
							}
						}

						impl Ord for AstarElement {
							fn cmp(&self, other: &AstarElement) -> Ordering {
								other
									.estimated_cost
									.cmp(&self.estimated_cost)
									.then_with(|| self.cost.cmp(&other.cost))
							}
						}

						let heuristic = |node: Point2<u8>| -> u8 {
							let l1 = absdiff(node.x, position.x) + absdiff(node.y, position.y);
							l1.saturating_sub(1)
						};

						let mut to_see = BinaryHeap::new();
						let mut min_cost = None;
						let mut closest_nodes = SmallVec::<[Point2<u8>; 4]>::new();
						to_see.push(AstarElement {
							estimated_cost: heuristic(chosen),
							cost: 0,
							index: 0,
						});
						let mut costs: IndexMap<Point2<u8>, u8> = IndexMap::new();
						costs.insert(chosen, 0);
						while let Some(AstarElement {
							cost,
							index,
							estimated_cost,
						}) = to_see.pop()
						{
							if let Some(min_cost) = min_cost {
								if estimated_cost > min_cost {
									break;
								}
							}
							let successors = {
								let (node, c) = costs.get_index(index).unwrap();
								if own_neighbours.contains(node) {
									min_cost = Some(cost);
									closest_nodes.push(*node);
								}
								if cost > *c {
									continue;
								}
								neighbours(node, &map_with_units)
							};
							for successor in successors {
								let new_cost = cost + 1;
								let h;
								let idx;
								match costs.entry(successor) {
									Vacant(e) => {
										h = heuristic(*e.key());
										idx = e.index();
										e.insert(new_cost);
									},
									Occupied(mut e) => {
										if *e.get() > new_cost {
											h = heuristic(*e.key());
											idx = e.index();
											*e.get_mut() = new_cost;
										} else {
											continue;
										}
									},
								}

								to_see.push(AstarElement {
									estimated_cost: new_cost + h,
									cost: new_cost,
									index: idx,
								});
							}
						}
						closest_nodes
							.into_iter()
							.min_by_key(|pos| (pos.y, pos.x))
							.unwrap()
					};
					*position_storage.get_mut(*entity).unwrap() = Position(new_position);
				}
			}

			// attack phase
			let Position(position) = position_storage.get(*entity).unwrap();
			let own_neighbour_squares = neighbours(position, &map);

			let target: Option<(specs::Entity, Point2<u8>, &mut Health)> =
				if *class == EntityClass::Elf {
					(
						&entities,
						&position_storage,
						&mut health_storage,
						&goblin_storage,
					)
						.join()
						.filter(|(e, ..)| !removed.contains(e))
						.filter(|(_, &Position(pos), ..)| own_neighbour_squares.contains(&pos))
						.min_by_key(|&(_, &Position(pos), &mut Health(health), _)| {
							(health, pos.y, pos.x)
						})
						.map(|(e, &Position(pos), health, _)| (e, pos, health))
				} else {
					(
						&entities,
						&position_storage,
						&mut health_storage,
						&elf_storage,
					)
						.join()
						.filter(|(e, ..)| !removed.contains(e))
						.filter(|(_, &Position(pos), ..)| own_neighbour_squares.contains(&pos))
						.min_by_key(|&(_, &Position(pos), &mut Health(health), _)| {
							(health, pos.y, pos.x)
						})
						.map(|(e, &Position(pos), health, _)| (e, pos, health))
				};

			const ATTACK_DAMAGE: u8 = 3;

			if let Some((enemy, pos, health)) = target {
				let damage = if part_2_info.is_some() && elf_storage.get(*entity).is_some() {
					part_2_info.as_ref().unwrap().elf_attack_damage
				} else {
					ATTACK_DAMAGE
				};
				if health.0 <= damage {
					if part_2_info.is_some() && elf_storage.get(enemy).is_some() {
						part_2_info.as_mut().unwrap().elf_died = true;
					}
					removed.push(enemy);
					map_with_units[(pos.x as usize, pos.y as usize)] = MapElement::Empty;
				} else {
					health.0 -= damage;
				}
			}

			map_with_units[(position.x as usize, position.y as usize)] = MapElement::Wall;
		}
		for entity in removed {
			entities.delete(entity).unwrap();
		}
		round.0 += 1;
	}
}

#[aoc(day15, part1)]
pub fn part_1(input: &PartIn) -> u32 {
	let mut world = World::new();
	world.add_resource(Map(input.map.clone()));
	world.add_resource(CombatStatus(true));
	world.add_resource(Round(0));

	let mut dispatcher = DispatcherBuilder::new()
		.with(SimulationSystem::default(), "simulation", &[])
		.build();
	dispatcher.setup(&mut world.res);

	for input in input.entities.iter() {
		let mut entity_builder = world
			.create_entity()
			.with(Position(input.position))
			.with(Health(200));
		entity_builder = match input.class {
			EntityClass::Elf => entity_builder.with(Elf),
			EntityClass::Goblin => entity_builder.with(Goblin),
		};
		entity_builder.build();
	}

	loop {
		dispatcher.dispatch_seq(&mut world.res);
		world.maintain();
		let total_health = world
			.read_storage::<Health>()
			.join()
			.map(|&Health(health)| health as u32)
			.sum::<u32>();
		let rounds = world.read_resource::<Round>().0;
		std::thread::sleep(std::time::Duration::from_millis(50));
		if !world.read_resource::<CombatStatus>().0 {
			break;
		}
	}

	let rounds = world.read_resource::<Round>().0;
	let total_health = world
		.read_storage::<Health>()
		.join()
		.map(|&Health(health)| health as u32)
		.sum::<u32>();
	rounds * total_health
}

#[aoc(day15, part2)]
pub fn part_2(input: &PartIn) -> u32 {
	let mut world = World::new();
	world.add_resource(Map(input.map.clone()));
	world.add_resource(CombatStatus(true));
	world.add_resource(Round(0));
	world.add_resource(Part2Info {
		elf_died: false,
		elf_attack_damage: 4,
	});

	let mut dispatcher = DispatcherBuilder::new()
		.with(SimulationSystem::default(), "simulation", &[])
		.build();
	dispatcher.setup(&mut world.res);

	'outer: loop {
		for input in input.entities.iter() {
			let mut entity_builder = world
				.create_entity()
				.with(Position(input.position))
				.with(Health(200));
			entity_builder = match input.class {
				EntityClass::Elf => entity_builder.with(Elf),
				EntityClass::Goblin => entity_builder.with(Goblin),
			};
			entity_builder.build();
		}

		loop {
			dispatcher.dispatch_seq(&mut world.res);
			world.maintain();
			let elf_died = world.read_resource::<Part2Info>().elf_died;
			let elf_damage = world.read_resource::<Part2Info>().elf_attack_damage;
			let total_health = world
				.read_storage::<Health>()
				.join()
				.map(|&Health(health)| health as u32)
				.sum::<u32>();
			let rounds = world.read_resource::<Round>().0;
			if elf_died {
				*world.write_resource::<CombatStatus>() = CombatStatus(true);
				*world.write_resource::<Round>() = Round(0);
				world.write_resource::<Part2Info>().elf_attack_damage += 1;
				world.write_resource::<Part2Info>().elf_died = false;
				world.delete_all();
				world.maintain();
				dispatcher = DispatcherBuilder::new()
					.with(SimulationSystem::default(), "simulation", &[])
					.build();
				continue 'outer;
			}
			std::thread::sleep(std::time::Duration::from_millis(10));
			if !world.read_resource::<CombatStatus>().0 {
				break 'outer;
			}
		}
	}

	let rounds = world.read_resource::<Round>().0;
	let total_health = world
		.read_storage::<Health>()
		.join()
		.map(|&Health(health)| health as u32)
		.sum::<u32>();
	rounds * total_health
}

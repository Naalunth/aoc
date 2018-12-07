type GeneratorOut = Vec<Edge>;

use std::collections::{HashSet, HashMap, BinaryHeap};
use std::cmp::Ordering;
use nom::types::CompleteStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Edge {
	from: u8,
	to: u8
}

named!(letter <CompleteStr, u8>, map!(take_while_m_n!(1, 1, |c: char| c.is_ascii_alphabetic()), |s| s.bytes().next().unwrap()));

named!(parse_line <CompleteStr, Edge>, do_parse!(
	tag!("Step ") >>
	from: letter >>
	tag!(" must be finished before step ") >>
	to: letter  >>
	tag!(" can begin.") >>
	(Edge {from, to})
));

#[aoc_generator(day7)]
pub fn generator(input: &str) -> GeneratorOut {
	input.lines().map(|l| parse_line(CompleteStr(l)).unwrap().1).collect::<Vec<_>>()
}

#[derive(Debug)]
struct NodeInfo {
	pub incoming: HashSet<u8>,
	pub outgoing: Vec<u8>,
}

impl NodeInfo {
	fn new() -> Self {
		NodeInfo {
			incoming: HashSet::new(),
			outgoing: Vec::new(),
		}
	}
}

#[derive(Debug)]
struct HeapElement {
	pub node: u8
}

impl Eq for HeapElement {}

impl PartialEq for HeapElement {
	fn eq(&self, other: &HeapElement) -> bool {
		self.node == other.node
	}
}

impl Ord for HeapElement {
	fn cmp(&self, other: &HeapElement) -> Ordering {
		self.node.cmp(&other.node).reverse()
	}
}

impl PartialOrd for HeapElement {
	fn partial_cmp(&self, other: &HeapElement) -> Option<Ordering> {
		Some(self.cmp(&other))
	}
}

fn generate_topo_sort_components(graph: &[Edge]) -> (HashMap<u8, NodeInfo>, BinaryHeap<HeapElement>) {
	let mut nodes: HashMap<u8, NodeInfo> = HashMap::with_capacity(graph.len());

	let dependency_less_nodes: BinaryHeap<HeapElement> = {
		let mut top_level_nodes: HashSet<u8> = HashSet::new();

		for edge in graph {
			if !nodes.contains_key(&edge.from) {
				top_level_nodes.insert(edge.from);
			}
			top_level_nodes.remove(&edge.to);
			nodes.entry(edge.from)
				.or_insert_with(|| NodeInfo::new())
				.outgoing.push(edge.to);
			nodes.entry(edge.to)
				.or_insert_with(|| NodeInfo::new())
				.incoming.insert(edge.from);
		}

		top_level_nodes.into_iter().map(|node| HeapElement{node}).collect()
	};

	(nodes, dependency_less_nodes)
}

#[aoc(day7, part1)]
pub fn part_1(input: &GeneratorOut) -> String {
	let (mut nodes, mut dependency_less_nodes) = generate_topo_sort_components(&input);

	let mut output_buffer: Vec<u8> = Vec::with_capacity(input.len());

	while let Some(HeapElement { node: current_node }) = dependency_less_nodes.pop() {
		output_buffer.push(current_node);

		for outgoing in nodes.remove(&current_node).unwrap()
			.outgoing.into_iter()
		{
			let mut out_ref = nodes.get_mut(&outgoing).unwrap();
			out_ref.incoming.remove(&current_node);
			if out_ref.incoming.is_empty() {
				dependency_less_nodes.push(HeapElement { node: outgoing });
			}
		}
	}

	unsafe { String::from_utf8_unchecked(output_buffer) }
}

#[aoc(day7, part2)]
pub fn part_2(input: &GeneratorOut) -> u32 {
	let (mut nodes, mut dependency_less_nodes) = generate_topo_sort_components(&input);

	fn cost(node: &u8) -> u32 {
		61 + *node as u32 - b'A' as u32
	}

	let mut time = 0u32;

	#[derive(Copy, Clone)]
	struct Worker {
		time: u32,
		job: Option<u8>,
	}
	let mut workers = [Worker{time: 0, job: None}; 5];

	while !nodes.is_empty() {
		workers.sort_unstable_by_key(|&w| w.time);
		let mut new_job_found = false;
		for worker in workers.iter() {
			if new_job_found && worker.time > time { break; }
			time = u32::max(time, worker.time);
			if let Some(job) = worker.job {
				for outgoing in nodes.remove(&job).unwrap()
					.outgoing.into_iter()
				{
					let mut out_ref = nodes.get_mut(&outgoing).unwrap();
					out_ref.incoming.remove(&job);
					if out_ref.incoming.is_empty() {
						new_job_found = true;
						dependency_less_nodes.push(HeapElement { node: outgoing });
					}
				}
			}
		}
		for worker in workers.iter_mut() {
			if worker.time > time { break; }
			if let Some(HeapElement { node: current_node }) = dependency_less_nodes.pop() {
				worker.job = Some(current_node);
				worker.time = time + cost(&current_node);
				continue;
			} else {
				worker.job = None;
			}
		}
	}

	time
}

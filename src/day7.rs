type GeneratorOut = Vec<Edge>;

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use nom::types::CompleteStr;
use smallvec::SmallVec;
use partition::partition;

type Node = u8;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Edge {
	from: Node,
	to: Node
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
	/// Count of incoming edges not yet removed
	pub incoming_count: u32,
	/// Nodes, that this node has edges going to
	pub outgoing: SmallVec<[Node; 6]>,
}

impl NodeInfo {
	fn new() -> Self {
		NodeInfo {
			incoming_count: 0,
			outgoing: SmallVec::new(),
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
struct HeapElement {
	pub node: Node
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

fn generate_topo_sort_components(graph: &[Edge]) -> (HashMap<Node, NodeInfo>, BinaryHeap<HeapElement>) {
	let mut nodes: HashMap<Node, NodeInfo> = HashMap::with_capacity(200);

	for edge in graph {
		nodes.entry(edge.from)
			.or_insert_with(|| NodeInfo::new())
			.outgoing.push(edge.to);
		nodes.entry(edge.to)
			.or_insert_with(|| NodeInfo::new())
			.incoming_count += 1;
	}

	let dependency_less_nodes = nodes.iter()
		.filter(|&(_, info)| info.incoming_count == 0)
		.map(|(&node, _)| HeapElement {node})
		.collect::<Vec<_>>()
		.into();

	(nodes, dependency_less_nodes)
}

#[aoc(day7, part1)]
pub fn part_1(input: &GeneratorOut) -> String {
	let (mut nodes, mut dependency_less_nodes) = generate_topo_sort_components(&input);

	let mut output_buffer: Vec<Node> = Vec::with_capacity(nodes.len());

	while let Some(HeapElement { node: current_node }) = dependency_less_nodes.pop() {
		output_buffer.push(current_node);

		for outgoing in nodes.remove(&current_node).unwrap()
			.outgoing.into_iter()
		{
			let mut out_ref = nodes.get_mut(&outgoing).unwrap();
			out_ref.incoming_count -= 1;
			if out_ref.incoming_count == 0 {
				dependency_less_nodes.push(HeapElement { node: outgoing });
			}
		}
	}

	unsafe { String::from_utf8_unchecked(output_buffer) }
}

#[aoc(day7, part2)]
pub fn part_2(input: &GeneratorOut) -> u32 {
	let (mut nodes, mut dependency_less_nodes) = generate_topo_sort_components(&input);

	fn cost(node: &Node) -> u32 {
		const OFFSET: u8 = u8::max_value() - (b'A' as u8) + 61 + 1;
		(*node).wrapping_add(OFFSET) as u32
	}

	let mut time = 0u32;

	#[derive(Copy, Clone)]
	struct Worker {
		time: u32,
		job: Option<Node>,
	}
	let mut workers = [Worker{time: 0, job: None}; 5];

	while !nodes.is_empty() {
		let mut new_job_found = false;
		let inactive_worker_count = {
			let (inactive, active) = partition(&mut workers, |&w| w.job.is_none());
			let mut inactive_worker_count = inactive.len();
			active.sort_unstable_by_key(|&w| w.time);
			for worker in active.iter() {
				if new_job_found && worker.time > time {
					break;
				}
				time = worker.time;
				let job = worker.job.unwrap();
				for outgoing in nodes.remove(&job).unwrap()
					.outgoing.into_iter()
				{
					let mut out_ref = nodes.get_mut(&outgoing).unwrap();
					out_ref.incoming_count -= 1;
					if out_ref.incoming_count == 0 {
						new_job_found = true;
						dependency_less_nodes.push(HeapElement { node: outgoing });
					}
				}
				inactive_worker_count += 1;
			}
			inactive_worker_count
		};

		for worker in workers[..inactive_worker_count].iter_mut().rev() {
			if let Some(HeapElement { node: current_node }) = dependency_less_nodes.pop() {
				worker.job = Some(current_node);
				worker.time = time + cost(&current_node);
			} else {
				worker.job = None;
			}
		}
	}

	time
}

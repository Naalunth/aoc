type GeneratorOut = Node;

use nom::types::CompleteStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
	children: Vec<Node>,
	meta: Vec<u32>,
}

impl std::convert::AsRef<Node> for Node {
	fn as_ref(&self) -> &Self {
		self
	}
}

named!(number <CompleteStr, u32>, map!(take_while1!(|c| char::is_ascii_digit(&c)), |s| s.parse::<u32>().unwrap()));

named!(parse_input <CompleteStr, Vec<u32>>, do_parse!(
	list: separated_list!(tag!(" "), number) >>
	tag!("\n") >>
	(list)
));

fn parse_node<'a, I>(input: &mut I) -> Node where
	I: Iterator<Item = &'a u32>
{
	let node_count = *input.next().unwrap();
	let meta_count = *input.next().unwrap();
	let nodes = (0..node_count).map(|_| parse_node(input)).collect::<Vec<_>>();
	let meta = (0..meta_count).map(|_| *input.next().unwrap()).collect::<Vec<_>>();
	Node {
		children: nodes,
		meta
	}
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> GeneratorOut {
	let list = parse_input(CompleteStr(input)).unwrap().1;
	parse_node(&mut list.iter())
}

#[aoc(day8, part1)]
pub fn part_1(input: &GeneratorOut) -> u32 {
	fn walk_node(node: &Node) -> u32 {
		node.children.iter()
			.map(|n| walk_node(n))
			.chain(node.meta.iter().cloned())
			.sum()
	}
	walk_node(&input)
}

#[aoc(day8, part2)]
pub fn part_2(input: &GeneratorOut) -> u32 {
	fn walk_node(node: &Node) -> u32 {
		if node.children.is_empty() {
			node.meta.iter().cloned().sum()
		} else {
			node.meta.iter()
				.filter_map(|meta| if *meta == 0 {None} else {Some(*meta-1)})
				.filter_map(|meta| {
					node.children.get(meta as usize).map(|c| walk_node(c))
				})
				.sum()
		}
	}
	walk_node(&input)
}

extern crate clap;
extern crate lettre;
extern crate lettre_email;
extern crate native_tls;
extern crate petgraph;
extern crate rand;


use clap::{App, Arg};
use lettre::EmailTransport;
use lettre::smtp::SmtpTransport;
use lettre::smtp::authentication::Credentials;
use lettre_email::EmailBuilder;
use petgraph::{Directed, Graph};
use petgraph::graph::{EdgeIndex, NodeIndex};
use rand::{random, thread_rng, Rng};

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn make_lines(
	input_filename: &str, credentials_filename: &str
) -> (Vec<String>, Vec<String>) {
	let input_file = match File::open(input_filename) {
		Ok(f) => f,
		Err(e) => {
			panic!(
				"opening file '{}' failed with error '{}'",
				input_filename,
				e
			);
		}
	};

	let credentials_file = match File::open(credentials_filename) {
		Ok(f) => f,
		Err(e) => {
			panic!(
				"opening file '{}' failed with error '{}'",
				credentials_filename,
				e
			);
		}
	};

	let input_buffer = BufReader::new(input_file);
	let input_lines = input_buffer
		.lines()
		.map(|line| match line {
			Ok(l) => l.trim().to_string(),
			Err(e) => {
				panic!(
					"reading file '{}' from buffer failed with error '{}'",
					input_filename,
					e
				);
			}
		})
		.filter(|ref line| match line.len() {
			0 => false,
			1 => true,
			_ => !line.starts_with("//"),
		})
		.collect();

	let credentials_buffer = BufReader::new(credentials_file);
	let credentials_lines = credentials_buffer
		.lines()
		.map(|line| match line {
			Ok(l) => l.trim().to_string(),
			Err(e) => {
				panic!(
					"reading file '{}' from buffer failed with error '{}'",
					credentials_filename,
					e
				);
			}
		})
		.filter(|ref line| match line.len() {
			0 => false,
			1 => true,
			_ => !line.starts_with("//"),
		})
		.collect();

	(input_lines, credentials_lines)
}

fn parse_input(
	input_filename: &str, input_lines: &mut Vec<String>
) -> Graph<String, i32, Directed> {
	if input_lines.len() < 1 {
		panic!("too few lines in input file '{}'", input_filename);
	}

	let num_nodes: usize = match input_lines[0].parse() {
		Ok(n) => n,
		Err(e) => {
			panic!(
				"could not parse '{}' as a number of nodes with error '{}'",
				input_lines[0],
				e
			);
		}
	};

	let num_edges = input_lines.len() - num_nodes - 1;

	let mut santa_graph = Graph::with_capacity(num_nodes, num_edges);
	let mut node_map = BTreeMap::new();

	thread_rng().shuffle(&mut input_lines[1..num_nodes + 1]);
	thread_rng().shuffle(&mut input_lines[num_nodes + 1..]);

	for line in input_lines[1..num_nodes + 1].iter() {
		let line_parts: Vec<_> = line.split_whitespace().collect();

		if line_parts.len() != 2 {
			panic!(
				"could not parse '{}' as a node in file '{}'",
				line,
				input_filename
			);
		}

		let index = santa_graph.add_node(line_parts[1].to_string());
		node_map.insert(line_parts[0], (line_parts[1], index));
	}

	for line in input_lines[num_nodes + 1..].iter() {
		let line_parts: Vec<_> = line.split_whitespace().collect();

		if line_parts.len() != 2 {
			panic!(
				"could not parse '{}' as an edge in file '{}'",
				line,
				input_filename
			);
		}

		let origin_node_index = match node_map.entry(line_parts[0]) {
			Entry::Occupied(e) => e.get().1,
			Entry::Vacant(_) => {
				panic!(
					"node '{}' not defined in file '{}'",
					line_parts[0],
					input_filename
				);
			}
		};

		let destination_node_index = match node_map.entry(line_parts[1]) {
			Entry::Occupied(e) => e.get().1,
			Entry::Vacant(_) => {
				panic!(
					"node '{}' not defined in file '{}'",
					line_parts[1],
					input_filename
				);
			}
		};

		santa_graph.add_edge(origin_node_index, destination_node_index, 1);
	}

	santa_graph
}

fn parse_credentials(
	credentials_filename: &str, credentials_lines: &Vec<String>
) -> (SmtpTransport, String) {
	if credentials_lines.len() < 4 {
		panic!(
			"too few lines in credentials file '{}'",
			credentials_filename
		);
	} else if credentials_lines.len() > 4 {
		panic!(
			"too many lines in credentials file '{}'",
			credentials_filename
		);
	}

	let address = credentials_lines[0].as_str();
	let sender = credentials_lines[1].clone();
	let username = credentials_lines[2].clone();
	let password = credentials_lines[3].clone();

	let builder = match SmtpTransport::simple_builder(address.to_string()) {
		Ok(b) => b,
		Err(e) => {
			panic!("creating SMTP transport failed with error '{}'", e);
		}
	};

	let built = builder
		.credentials(Credentials::new(username, password))
		.build();

	(built, sender)
}

fn find_hamilton_cycle(
	santa_graph: &Graph<String, i32, Directed>
) -> Option<Vec<EdgeIndex>> {
	let mut current_node = NodeIndex::new(0);
	let mut edges = Vec::with_capacity(santa_graph.node_count());
	let mut visited = vec![false; santa_graph.node_count()];
	let mut tainted =
		vec![vec![false; santa_graph.node_count()]; santa_graph.node_count()];
	visited[0] = true;

	while edges.len() != santa_graph.node_count() {
		if edges.len() == santa_graph.node_count() - 1 {
			visited[0] = false;
		} else {
			visited[0] = true;
		}

		let mut broke = false;

		for neighbor in santa_graph.neighbors(current_node) {
			if !visited[neighbor.index()]
				&& !tainted[current_node.index()][neighbor.index()]
			{
				edges.push(
					santa_graph.find_edge(current_node, neighbor).unwrap(),
				);
				visited[neighbor.index()] = true;
				current_node = neighbor;
				broke = true;

				tainted[current_node.index()] =
					vec![false; santa_graph.node_count()];

				break;
			}
		}

		if !broke {
			if edges.is_empty() {
				return None;
			}

			let (last_node, this_node) =
				santa_graph.edge_endpoints(*edges.last().unwrap()).unwrap();

			edges.pop();

			tainted[last_node.index()][this_node.index()] = true;
			visited[this_node.index()] = false;
			current_node = last_node;
		}
	}

	Some(edges)
}

fn email_cycle(
	sender: &str, transport: &mut SmtpTransport,
	graph: &Graph<String, i32, Directed>, cycle: &Vec<EdgeIndex>,
) {
	for edge in cycle.iter() {
		let (source_index, destination_index) =
			graph.edge_endpoints(*edge).unwrap();

		let source_address = graph.node_weight(source_index).unwrap();
		let destination_address =
			graph.node_weight(destination_index).unwrap();

		let email = EmailBuilder::new()
			.to(source_address.as_str())
			.from(sender)
			.subject("Secret Santa Assignment")
			.text(format!(
				"Your assignment for secret santa this year is '{}.'",
				destination_address
			))
			.build()
			.unwrap();

		match transport.send(&email) {
			Ok(r) => println!(
				"sent email to '{}' with response '{}'",
				source_address,
				r.code
			),
			Err(e) => panic!(
				"unable to send email to '{}' with error '{:?}'",
				source_address,
				e
			),
		}
	}
}

fn main() {
	let matches = App::new("secret-santa")
		.version("0.1.0")
		.author("Gregory Meyer <gregjm@umich.edu>")
		.about("Utility to email a group of Secret Santa participants")
		.arg(
			Arg::with_name("INPUT")
				.required(true)
				.index(1)
				.help("file with each node and edge of a secret santa graph"),
		)
		.arg(
			Arg::with_name("CREDENTIALS")
				.required(true)
				.index(2)
				.help("file with SMTP credentials"),
		)
		.arg(
			Arg::with_name("FORCE")
				.short("f")
				.long("force")
				.help("adds edges if there is not a hamiltonian cycle"),
		)
		.get_matches();

	let input_filename = matches.value_of("INPUT").unwrap();
	let credentials_filename = matches.value_of("CREDENTIALS").unwrap();

	let (mut input_lines, credentials_lines) =
		make_lines(input_filename, credentials_filename);

	let mut santa_graph = parse_input(input_filename, &mut input_lines);
	let (mut transport, sender) =
		parse_credentials(credentials_filename, &credentials_lines);

	let mut edges_wrapped = find_hamilton_cycle(&santa_graph);

	let force = matches.is_present("FORCE");

	while edges_wrapped.is_none() {
		if !force {
			panic!(
				"unable to find a hamilton cycle. rerun with -f (--force) to \
				 randomly add edges"
			);
		}

		let to_add_source: usize =
			random::<usize>() % santa_graph.node_count();
		let to_add_destination: usize =
			random::<usize>() % santa_graph.node_count();

		if to_add_source == to_add_destination
			|| santa_graph
				.find_edge(
					NodeIndex::new(to_add_source),
					NodeIndex::new(to_add_destination),
				)
				.is_some()
		{
			continue;
		}

		santa_graph.update_edge(
			NodeIndex::new(to_add_source),
			NodeIndex::new(to_add_destination),
			1,
		);

		println!(
			"added a new edge from node {} to node {}",
			to_add_source,
			to_add_destination
		);

		edges_wrapped = find_hamilton_cycle(&santa_graph);
	}

	let edges = edges_wrapped.unwrap();

	for edge in edges.iter() {
		let (source_node, destination_node) =
			santa_graph.edge_endpoints(*edge).unwrap();

		println!(
			"edge from node {} to node {}",
			source_node.index(),
			destination_node.index()
		);
	}

	email_cycle(sender.as_str(), &mut transport, &santa_graph, &edges);
}

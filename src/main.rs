extern crate clap;


use clap::{App, Arg};

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process::exit;

fn make_lines(
	input_filename: &str, credentials_filename: &str
) -> (Vec<String>, Vec<String>) {
	let input_file = match File::open(input_filename) {
		Ok(f) => f,
		Err(e) => {
			eprintln!(
				"error: opening file '{}' failed with error '{}'",
				input_filename,
				e
			);

			exit(1);
		}
	};

	let credentials_file = match File::open(credentials_filename) {
		Ok(f) => f,
		Err(e) => {
			eprintln!(
				"error: opening file '{}' failed with error '{}'",
				credentials_filename,
				e
			);

			exit(1);
		}
	};

	let input_buffer = BufReader::new(input_file);
	let input_lines = input_buffer
		.lines()
		.map(|line| match line {
			Ok(l) => l,
			Err(e) => {
				eprintln!(
					"error: reading file '{}' from buffer failed with error \
					 '{}'",
					input_filename,
					e
				);

				exit(1);
			}
		})
		.collect();

	let credentials_buffer = BufReader::new(credentials_file);
	let credentials_lines = credentials_buffer
		.lines()
		.map(|line| match line {
			Ok(l) => l,
			Err(e) => {
				eprintln!(
					"error: reading file '{}' from buffer failed with error \
					 '{}'",
					credentials_filename,
					e
				);

				exit(1);
			}
		})
		.collect();

	(input_lines, credentials_lines)
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
		.get_matches();


	let input_filename = matches.value_of("INPUT").unwrap();
	let credentials_filename = matches.value_of("CREDENTIALS").unwrap();

	let (input_lines, credentials_lines) =
		make_lines(input_filename, credentials_filename);
}

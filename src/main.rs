/*
 * Copyright (c) 2018 Rubyist
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * 
 *     http://www.apache.org/licenses/LICENSE-2.0
 * 
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

extern crate googology;

use std::env;
use std::io;

use googology::conway_wechsler::{full_name, power_of_ten};

fn usage() {
	println!("Usage: zillion [OPTIONS] [num]");
	println!("Produces a Conway-Wechsler name for a given number");
	println!("Running without arguments causes num to be read from stdin");
	println!("");
	println!("Options:");
	println!("    -h, --help     Print this message");
	println!("    -p, --power    Compute for 10^num instead");
}

fn handle_arguments(argv : Vec<String>) {
	match argv[0].as_str() {
		"-h" | "--help" => usage(),
		"-p" | "--power" => 
			if argv.len() > 1 {
				println!("{}", power_of_ten(&argv[1], true).unwrap());
			}
			else { panic!("No argument provided for --power"); },
		num  => println!("{}", full_name(num, true).unwrap())
	}
}

fn handle_stdin() {
	let input = io::stdin();
	let mut buf = String::new();

	while let Ok(_) = input.read_line(&mut buf) {
		{
			let stripped = buf.trim();
			if stripped.is_empty() {
				break;
			}
			println!("{}", full_name(stripped, true).unwrap());
		}
		buf.clear();
	}
}

fn main() {
	let argv : Vec<String> = env::args().skip(1).collect();
	match argv.len() {
		0 => handle_stdin(),
		_ => handle_arguments(argv),
	}
}


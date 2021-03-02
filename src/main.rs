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

#[macro_use]
extern crate clap;
extern crate googology;

use std::io;
use googology::ParseError;
use googology::conway_wechsler::Scale;

fn handle_knuth(digits: &str, pow: bool, _: Scale) -> Result<String, ParseError> {
	use googology::knuth_yllion::{full_name, power_of_ten};
	if pow { power_of_ten(digits) }
	else { full_name(digits) }
}

fn handle_conway(digits: &str, pow: bool, s: Scale) -> Result<String, ParseError> {
	use googology::conway_wechsler::{full_name, power_of_ten};
	if pow { power_of_ten(digits, s) }
	else { full_name(digits, s) }
}

fn handle_stdin(
	power: bool,
	scale: Scale,
	namefn: fn(&str, bool, Scale) -> Result<String, ParseError>
) -> Result<(), ParseError> {
	let input = io::stdin();
	let mut buf = String::new();

	while let Ok(_) = input.read_line(&mut buf) {
		{
			let stripped = buf.trim();
			if stripped.is_empty() { break; }

			let output = namefn(stripped, power, scale)?;
			println!("{}", output);
		}
		buf.clear();
	}

	Ok(())
}

fn main() -> Result<(), ParseError> {
	// Parse command line arguments
	let matches = clap_app!(zillion =>
		(version: "0.5.1")
		(about: "Produces a natural language name for a given number")
		(@arg POWER: -p --power "Compute a name for a power of ten")
		(@arg NAMEFN: -n --namefn +takes_value possible_value[conway knuth]
			"Scheme for transforming numbers into names (default = conway)")
		(@arg SCALE: -s --scale +takes_value possible_value[short british peletier]
			"Scale used when Conway-Wechsler naming scheme is used (default = short)")
		(@arg INPUT: "Input number (Uses STDIN if not present)")
	).get_matches();

	let power  = matches.is_present("POWER");
	let namefn = matches.value_of("NAMEFN").map(|s| match s {
		"conway" => handle_conway,
		"knuth"  => handle_knuth,
		_        => unreachable!(),
	}).unwrap_or(handle_conway);
	
	let scale  = matches.value_of("SCALE").map(|s| match s {
		"short"    => Scale::Short,
		"british"  => Scale::LongBritish,
		"peletier" => Scale::LongPeletier,
		_          => unreachable!(),
	}).unwrap_or(Scale::Short);
	
	if let Some(input) = matches.value_of("INPUT") {
		let output = namefn(input, power, scale)?;
		println!("{}", output);
		Ok(())
	}
	else {
		handle_stdin(power, scale, namefn)
	}
}

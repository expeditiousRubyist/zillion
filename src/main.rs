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

extern crate num_traits;
extern crate num_bigint;

use std::env;
use std::io;
use std::str::FromStr;

use num_traits::cast::ToPrimitive;
use num_traits::identities::Zero;
use num_traits::identities::One;
use num_bigint::BigUint;

static NAMES_UPTO_TWENTY: [&'static str; 20] = [
	"", "one", "two", "three", "four", "five", "six", "seven", "eight",
	"nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
	"sixteen", "seventeen", "eighteen", "nineteen"
];

static TENS_NAMES: [&'static str; 10] = [
	"", "", "twenty", "thirty", "fourty", "fifty", "sixty", "seventy",
	"eighty", "ninety"
];

static ZILLION_UNIT_PREFIXES: [&'static str; 10] = [
	"", "un", "duo", "tre", "quattuor", "quinqua", "se", "septe", "octo",
	"nove"
];

static ZILLION_TENS_PREFIXES: [&'static str; 10] = [
	"", "deci", "viginti", "triginta", "quadraginta", "quinquaginta",
	"sexaginta", "septuaginta", "octoginta", "nonaginta"
];

static ZILLION_HUNDREDS_PREFIXES: [&'static str; 10] = [
	"", "centi", "ducenti", "trecenti", "quadringenti", "quingenti",
	"sescenti", "septingenti", "octingenti", "nongenti"
];

static ZILLIONS_UNDER_TEN: [&'static str; 10] = [
	"nilli", "milli", "billi", "trilli", "quadrilli", "quintilli",
	"sextilli", "septilli", "octilli", "nonilli"
];

// Produces a name for some number in the range [0, 999].
// The name for zero is the empty string.
// Values above 999 will panic
fn three_digit_name(num: usize) -> String {
	assert!(num < 1000, "Input to three_digit_name is more than 3 digits!");

	let hs = num / 100;      // Hundreds place
	let ts = num % 100 / 10; // Tens place
	let us = num % 10;       // Units place

	// Hundred name (if applicable)
	let mut name = String::from(NAMES_UPTO_TWENTY[hs]);
	if !name.is_empty() { name.push_str(" hundred"); }

	// Rest of name
	if ts > 1 {
		if !name.is_empty() { name.push_str(" "); }
		name.push_str(TENS_NAMES[ts]);
		if us > 0 { 
			name.push_str(" ");
			name.push_str(NAMES_UPTO_TWENTY[us]);
		}
	}
	else {
		let aux = ts*10 + us;
		if aux > 0 {
			if !name.is_empty() { name.push_str(" "); }
			name.push_str(NAMES_UPTO_TWENTY[aux]);
		}
	}

	name
}

// Create a name for an arbitrary power of 1000.
// Value for zero is the empty string.
// Value for one is "thousand".
// Value for anything greater will involve repeated application of the
// zillion_prefix function, to create a number ending in "illion".
fn zillion_number(num: usize) -> String {
	if num == 0 { return String::from(""); }
	if num == 1 { return String::from("thousand"); }
	
	let mut name  = String::from("");
	let mut power = num - 1;

	// Prefixes technically added in reverse order here.
	// e.g. in millinillion, first add "nilli", then "milli", then "on".
	while power > 0 {
		let prefix = zillion_prefix(power % 1000);
		name.insert_str(0, prefix.as_str());
		power /= 1000;
	}

	name.push_str("on");
	name
}

// Create a name for a single 3 digit zillion number, ending in -illi.
// Value for zero is "nilli", for use in chained zillion numbers.
// Values above 999 will panic.
fn zillion_prefix(num: usize) -> String {
	assert!(num < 1000, "Input to zillion_prefix is more than 3 digits!");

	if num < 10 { return String::from(ZILLIONS_UNDER_TEN[num]); }

	let hs = num / 100;      // Hundreds place
	let ts = num % 100 / 10; // Tens place
	let us = num % 10;       // Units place

	let mut name = String::from(ZILLION_UNIT_PREFIXES[us]);
	if ts > 0 {
		// Special unit place endings
		match (us, ts) {
			(3, 2..=5) | (3, 8) => name.push('s'), // tres
			(6, 2..=5)          => name.push('s'), // ses
			(6, 8)              => name.push('x'), // sex
			(7, 1) | (7, 3..=7) => name.push('n'), // septen
			(7, 2) | (7, 8)     => name.push('m'), // septem
			(9, 1) | (9, 3..=7) => name.push('n'), // noven
			(9, 2) | (9, 8)     => name.push('m'), // novem
			_ => (),
		}

		name.push_str(ZILLION_TENS_PREFIXES[ts]);
		name.push_str(ZILLION_HUNDREDS_PREFIXES[hs]);
	}
	else {
		// Special unit place endings
		match (us, hs) {
			(3, 1) | (3, 3..=5) | (3, 8) => name.push('s'), // tres
			(6, 1) | (6, 8) => name.push('x'), // sex
			(6, 3..=5)      => name.push('s'), // ses
			(7, 1..=7)      => name.push('n'), // septen
			(7, 8)          => name.push('m'), // septem
			(9, 1..=7)      => name.push('n'), // noven
			(9, 8)          => name.push('m'), // novem
			_ => (),
		}

		name.push_str(ZILLION_HUNDREDS_PREFIXES[hs]);
	}

	name.pop();
	name.push_str("illi");
	name
}

// Input: a sequence of digits (0-9) of arbitrary length
// Output: a proper name using the conway-wechsler system
fn conway_wechsler(digits: &str) -> String {
	// Input sanity check
	assert!(digits.chars().all(|c| {
		c.is_digit(10)
	}), "Unrecognized input character in conway_wechsler (not 0-9).");

	// Skip leading zeroes. If all characters are 0, return "zero"
	let tmp = digits.find(|c| c != '0');
	let mut output = match tmp {
		Some(_) => String::from(""),
		None => String::from("zero"),
	};

	if !output.is_empty() { return output; }

	// Determine how many digits to process, and how many digits are in the
	// first zillion (e.g. 2 in the case of 12 tredecillion).
	let mut i = tmp.unwrap();
	let mut remaining = digits.len() - i;
	let first = remaining % 3;

	if first > 0 {
		let num = digits.get(i..i+first)
		                .unwrap()
		                .parse::<usize>()
		                .unwrap();
		let leading = three_digit_name(num);
		let zillion = zillion_number(remaining / 3);

		output.push_str(leading.as_str());
		if !zillion.is_empty() {
			output.push(' ');
			output.push_str(zillion.as_str());
		}

		remaining -= first;
		i += first;
	}

	while remaining > 0 {
		let num = digits.get(i..i+3)
		                .unwrap()
		                .parse::<usize>()
		                .unwrap();
		let leading = three_digit_name(num);
		let zillion = zillion_number(remaining / 3 - 1);

		if !leading.is_empty() {
			if !output.is_empty() { output.push(' '); }
			output.push_str(leading.as_str());

			if !zillion.is_empty() {
				output.push(' ');
				output.push_str(zillion.as_str());
			}
		}

		i += 3;
		remaining -= 3;
	}

	output
}

// Input: A string representation of some number n
// Output: A Conway-Wechsler name for 10^n
fn power_of_ten(num: &str) -> String {
	let mut power = BigUint::from_str(num).unwrap();

	// Get the leading word (e.g. "ten" in "ten million")
	let m = (&power % 3u32).to_u32().unwrap();
	let s = match m {
		0 => "One",
		1 => "Ten",
		2 => "One hundred",
		_ => unreachable!(),
	};
	let mut output = String::from(s);

	// Convert into power of one thousand
	// We may return early for edge cases.
	power /= 3u32;
	if power.is_zero() { return output; }
	if power.is_one() {
		output.push_str(" thousand");
		return output;
	}

	// Compute zillion number.
	power -= 1u32;
	output.push_str(" ");
	let loc = output.len(); // Location to insert prefixes at

	// Add prefixes in reverse order because we are stupid and inefficient.
	while !power.is_zero() {
		let m = (&power % 1000u32).to_usize().unwrap();
		let prefix = zillion_prefix(m);
		output.insert_str(loc, prefix.as_str());
		power /= 1000u32;
	}

	output.push_str("on");
	output
}

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
				println!("{}", power_of_ten(&argv[1]));
			}
			else { panic!("No argument provided for --power"); },
		num  => println!("{}", conway_wechsler(num))
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
			println!("{}", conway_wechsler(stripped));
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


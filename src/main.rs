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
			(3, 2...5) | (3, 8) => name.push('s'), // tres
			(6, 2...5)          => name.push('s'), // ses
			(6, 8)              => name.push('x'), // sex
			(7, 1) | (7, 3...7) => name.push('n'), // septen
			(7, 2) | (7, 8)     => name.push('m'), // septem
			(9, 1) | (9, 3...7) => name.push('n'), // noven
			(9, 2) | (9, 8)     => name.push('m'), // novem
			_ => (),
		}

		name.push_str(ZILLION_TENS_PREFIXES[ts]);
		name.push_str(ZILLION_HUNDREDS_PREFIXES[hs]);
	}
	else {
		// Special unit place endings
		match (us, hs) {
			(3, 1) | (3, 3...5) | (3, 8) => name.push('s'), // tres
			(6, 1) | (6, 8) => name.push('x'), // sex
			(6, 3...5)      => name.push('s'), // ses
			(7, 1...7)      => name.push('n'), // septen
			(7, 8)          => name.push('m'), // septem
			(9, 1...7)      => name.push('n'), // noven
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
// Output: a proper name using the conway-weschler system
fn conway_weschler(digits: &str) -> String {
	// Input sanity check
	assert!(digits.chars().all(|c| {
		c.is_digit(10)
	}), "Unrecognized input character in conway_weschler (not 0-9).");

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

fn main() {
	println!("42: {}", conway_weschler("42"));
	println!("9: {}", conway_weschler("009"));
	println!("104: {}", conway_weschler("104"));
	println!("65536: {}", conway_weschler("65536"));
	println!("1000000: {}", conway_weschler("1000000"));
	println!("7000000000: {}", conway_weschler("7000000000"));
	println!("20000000000000: {}", conway_weschler("20000000000000"));
}

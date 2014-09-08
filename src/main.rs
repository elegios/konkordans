extern crate encoding;

use encoding::{Encoding, DecodeStrict};
use encoding::all::ISO_8859_1;
use std::io;

fn main() {
	//let mut file = io::File::open(&Path::new("korpus"));
	//file.seek(-720, io::SeekEnd);
	//let korpus = ISO_8859_1.decode(file.read_to_end().unwrap().as_slice(), DecodeStrict).unwrap();
	gen_index();
}

fn gen_index() {
	let mut latman = io::BufferedWriter::new(io::File::create(&Path::new("index/latman")));
	let mut words = io::BufferedWriter::new(io::File::create(&Path::new("index/words")));
	let mut positions = io::BufferedWriter::new(io::File::create(&Path::new("index/positions")));

	let mut indata = io::BufferedReader::new(io::File::open(&Path::new("sorted")));

	let mut key = "".to_string();
	let mut last_word = "".to_string();

	//let mut indata = io::stdin();
	for line in indata.lines() {
		let realline = line.ok().expect("blub");
		let mut tokens = realline.as_slice().words();
		let word = tokens.next().expect("Malformed word").to_string();
		let pos: i64 = from_str(tokens.next().expect("No pointer")).expect("What is this?");

		if word != last_word {
			if key != get_key(&word) {
				let next_key = get_key(&word);
				for i in range(0, key_distance(&key, &next_key)) {
					latman.write_le_i64(-1);
				}
				words.flush();
				latman.write_le_i64(words.get_ref().tell().unwrap().to_i64().unwrap());
				key = next_key;
			}

			positions.write_le_i64(-1);
			positions.flush();
			words.write_str(word.as_slice());
			words.write_char('\0');
			words.write_le_i64(positions.get_ref().tell().unwrap().to_i64().unwrap());

			last_word = word;
		}

		positions.write_le_i64(pos);
	}
}

fn get_key(key: &String) -> String {
	format!("{:3.3}", key + "   ")
}

fn key_distance(k1: &String, k2: &String) -> int {
	hash(k2) - hash(k1)
}

fn chash(c: char) -> int {
	match c {
		'ä' => 'z'.to_digit(36).unwrap() - 9 + 1,
		'å' => 'z'.to_digit(36).unwrap() - 9 + 2,
		'ö' => 'z'.to_digit(36).unwrap() - 9 + 3,
		' ' => 0,
		_ => c.to_digit(36).unwrap() - 9,
	}.to_int().unwrap()
}

fn hash(key: &String) -> int {
	let mut iter = key.as_slice().chars();
	let a = iter.next().unwrap_or(' ');
	let b = iter.next().unwrap_or(' ');
	let c = iter.next().unwrap_or(' ');
	chash(a) * 30^2 + chash(b) * 30 + chash(c)
}

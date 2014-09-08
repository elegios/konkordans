extern crate encoding;

use encoding::{Encoding, DecodeStrict};
use encoding::all::ISO_8859_1;
use std::io;

fn main() {
	let mut file = io::File::open(&Path::new("korpus"));
	file.seek(-720, io::SeekEnd);
	let korpus = ISO_8859_1.decode(file.read_to_end().unwrap().as_slice(), DecodeStrict).unwrap();
	print!("{}", korpus);
}

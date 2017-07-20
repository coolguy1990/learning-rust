use std::fs::File;
use std::io::ErrorKind;

fn main() {
	let f = File::open("Hello.txt");

	let f = match f {
		Ok(file) => file,
		Err(ref error) if error.kind() == ErrorKind::NotFound => {
			match File::create("Hello.txt") {
				Ok(fc) => fc,
				Err(err) => {
					panic!("Unable to write {:?}", err);
				},
			}
		},
		Err(err) => {
			panic!("Problem opening file {:?}", err);
		},
	};
}

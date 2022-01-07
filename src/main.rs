use std::env;
use std::fs::{self, File};

fn main() {
	let arguments = env::args().collect::<Vec<String>>();
	let mut current_dir = env::current_dir().unwrap();
	current_dir.push(&arguments[1]);
	// let file = File::create(current_dir.as_path()).unwrap();
	// file.write("#include <iostream>\nusing namespace std;\n int main() \n { \n \n return 0\n }");
	fs::write(current_dir.as_path(), 
b"#include <iostream>\nusing namespace std;\nint main()\n{\n\nreturn 0\n}").unwrap();

}
use file_config::FileConfigs;
use std::env;
use std::fs::{self, File};
use std::io::BufWriter;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::str;

mod file_config;

const CONFIG_PATH: &str = "./config.json";

fn main() {
	// let possible_arguments;
	let arguments = env::args().collect::<Vec<String>>();
	let mut users_current_path: PathBuf = env::current_dir().unwrap();
	let mut file_configs = FileConfigs::new();
	file_configs.init();
	let primary_argument: &str = &arguments.get(1).unwrap();
	match primary_argument {
		"add" => {
			if let Some(extension) = arguments.get(2) {
				println!("Please enter the boilerplate to be added:");
				let mut boilerplate = String::new();
				let mut stdin = io::stdin();
				stdin.read_to_string(&mut boilerplate).unwrap();
				file_configs
					.add(String::from(extension), boilerplate);
			}
		}
		"create" => {
			if let Some(file_name) = arguments.get(2) {
				users_current_path.push(PathBuf::from(file_name));
				let file = File::create(users_current_path).unwrap();
				let mut writer = BufWriter::new(file);
				let extension: Vec<&str> =
					file_name.split(".").collect();
				writer
					.write(
						file_configs
							.get(extension[extension.len()])
							.as_bytes(),
					)
					.unwrap();
			}
		}
		_ => {}
	}

	// b"#include <iostream>\nusing namespace std;\nint main()\n{\n\nreturn 0\n}").unwrap();
}

use file_config::FileConfigs;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::{self, Read, Write};
use std::path::PathBuf;
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
				let mut boilerplate: Vec<u8> = Vec::new();
				boilerplate.reserve(500);
				let mut stdin = io::stdin();
				stdin.read_to_end(&mut boilerplate).unwrap();
				file_configs.add(
					String::from(extension),
					String::from_utf8(boilerplate).unwrap(),
				);
			}
		}
		"create" => {
			if let Some(file_name) = arguments.get(2) {
				users_current_path.push(PathBuf::from(file_name));
				let file = File::create(users_current_path).unwrap();
				let mut writer = BufWriter::new(file);
				let extension_index: usize =
					file_name.find(".").unwrap() + 1;
				writer
					.write(
						file_configs
							.get(&file_name[extension_index..])
							.as_bytes(),
					)
					.unwrap();
			}
		}
		"remove" => {
			if let Some(extension) = arguments.get(2) {
				file_configs.remove(extension);
			}
		}
		_ => {}
	}
}

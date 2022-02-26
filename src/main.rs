use std::env;
use std::fs::{self, File};
use std::path::PathBuf;

mod file_config;

const CONFIG_PATH: &str = "./config.json";

fn main() {
	let arguments = env::args().collect::<Vec<String>>();
	let mut users_current_path:PathBuf = env::current_dir().unwrap();
	let mut file_configs = file_config::FileConfigs::new();
	file_configs.init();

	users_current_path.push(&arguments[1]);
	fs::write(users_current_path.as_path(), 
b"#include <iostream>\nusing namespace std;\nint main()\n{\n\nreturn 0\n}").unwrap();

}
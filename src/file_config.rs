use dirs_2::home_dir;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufWriter, Read};
use std::path::PathBuf;

pub struct FileConfigs {
	config_path: PathBuf,
	configs: HashMap<String, String>,
}

impl FileConfigs {
	pub fn new() -> Self {
		let mut config_path = home_dir().unwrap();
		config_path.push("fizzy_config.json");

		let mut config_file = File::options()
			.read(true)
			.write(true)
			.create(true)
			.open(&config_path)
			.expect("Couldn't open/create file");

		let mut content = String::with_capacity(
			config_file
				.metadata()
				.unwrap()
				.len()
				.try_into()
				.expect("Couldn't convert length of file into usize"),
		);

		config_file
			.read_to_string(&mut content)
			.expect("Error in reading to string");

		let configs: HashMap<String, String> =
			serde_json::from_str(&content).unwrap();

		FileConfigs {
			config_path,
			configs,
		}
	}

	fn update(&self) {
		let file = File::create(&self.config_path).unwrap();
		let writer = BufWriter::new(&file);
		serde_json::to_writer(writer, &self.configs).unwrap();
	}

	pub fn add(&mut self, extension: String, boilerplate: String) {
		self.configs.insert(extension, boilerplate);
		self.update();
	}

	pub fn remove(&mut self, extension: &str) {
		self.configs.remove(extension);
		self.update();
	}

	pub fn get(&self, extension: &str) -> &String {
		self.configs.get(extension).unwrap()
	}
}

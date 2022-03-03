use crate::CONFIG_PATH;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Read};
use std::path::Path;

pub struct FileConfigs {
	configs: HashMap<String, String>,
}

impl FileConfigs {
	pub fn new() -> Self {
		FileConfigs {
			configs: HashMap::new(),
		}
	}

	pub fn init(&mut self) {
		if !Path::new(CONFIG_PATH).exists() {
			let file = File::create(CONFIG_PATH).unwrap();
			let mut buf_writer = BufWriter::new(file);

			serde_json::to_writer(&mut buf_writer, &self.configs)
				.unwrap();
		} else {
			let mut config_file =
				File::options().read(true).open(CONFIG_PATH).unwrap();

			let mut content = String::new();
			config_file.read_to_string(&mut content).unwrap();
			let configs: HashMap<String, String> =
				serde_json::from_str(&content).unwrap();
			self.configs = configs;
		}
	}

	fn update(&self) {
		let file = File::create(CONFIG_PATH).unwrap();
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

use dirs_2::home_dir;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter, Read};
use std::path::{Path, PathBuf};

pub struct FileConfigs {
	config_path: PathBuf,
	configs: HashMap<String, String>,
}

impl FileConfigs {
	pub fn new() -> Self {
		let mut config_path = home_dir().unwrap();
		// config_path.push("fizzy");
		config_path.push("fizzy_config.json");
		println!("{:?}", config_path);

		FileConfigs {
			config_path,
			configs: HashMap::new(),
		}
	}

	pub fn init(&mut self) {
		if !Path::new(&self.config_path).exists() {
			// fs::create_dir(&self.config_path).unwrap();
			// let file = File::create(&self.config_path).unwrap();
			let file = File::options()
				.read(true)
				.write(true)
				.create(true)
				.open(&self.config_path)
				.unwrap();
			let mut buf_writer = BufWriter::new(file);

			serde_json::to_writer(&mut buf_writer, &self.configs)
				.unwrap();
		} else {
			let mut config_file = File::options()
				.read(true)
				.open(&self.config_path)
				.unwrap();

			let mut content = String::new();
			config_file.read_to_string(&mut content).unwrap();
			let configs: HashMap<String, String> =
				serde_json::from_str(&content).unwrap();
			self.configs = configs;
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

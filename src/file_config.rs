use crate::CONFIG_PATH;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Read};
use std::path::Path;

#[derive(Deserialize, Serialize)]
struct FileConfig {
	extension: String,
	boilerplate: String,
}

pub struct FileConfigs {
	configs: Option<Vec<FileConfig>>,
	config_file: Option<File>,
}

impl FileConfigs {
	pub fn new() -> Self {
		FileConfigs {
			configs: None,
			config_file: None,
		}
	}

	pub fn init(&mut self) {
		if !Path::new(CONFIG_PATH).exists() {
			let file = File::create(CONFIG_PATH).unwrap();
			let mut buf_writer = BufWriter::new(file);

			serde_json::to_writer(&mut buf_writer, &self.configs)
				.unwrap();
		} else {
			let mut config_file = File::options()
				.read(true)
				.write(true)
				.open(CONFIG_PATH)
				.unwrap();

			let mut content = String::new();
			config_file.read_to_string(&mut content).unwrap();
			let configs: Vec<FileConfig> =
				serde_json::from_str(&content).unwrap();

			self.config_file = Some(config_file);
			self.configs = Some(configs);
		}
	}

	fn add(&mut self, name: String, boilerplate: String) {}
}

use crate::envoy_helpers::EnvoyExport;
use crate::service;
use std::fs::File;
use std::io::Read;

#[derive(Default, Debug)]
pub struct Config {
    services: Vec<service::Service>,
}

impl Config {
    pub fn parse_config(path: &str) -> Config {
        let mut config = Config {
            services: Vec::new(),
        };
        let raw_config = config.read_path(path);
        config.parse_json(raw_config);
        config
    }

    fn parse_json(&mut self, raw_config: std::string::String) {
        let mut result: Vec<service::Service> = Vec::new();

        // @TODO handle error properly here
        let v: Vec<service::Service> = serde_json::from_str(raw_config.as_str()).unwrap();
        for val in v {
            log::info!("Service with id='{}' added to the pool", val.id);
            result.push(val);
        }
        // Update services.
        self.services = result;
    }

    fn read_path(&self, path: &str) -> std::string::String {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(error) => panic!("There was a problem opening the file: {:?}", error),
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Error reading the file");

        contents
    }

    pub fn export_config_to_envoy(&self) -> Vec<EnvoyExport> {
        let mut result = Vec::with_capacity(self.services.len());
        result.extend(self.services.iter().flat_map(service::Service::export));
        result
    }
}

use crate::envoy_helpers::EnvoyExportList;
use crate::service;
use std::fs::File;
use std::io::Read;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use anyhow::Result;

type ServicesList = Vec<service::Service>;

#[derive(Default, Debug, Clone)]
pub struct Config {
    services: ServicesList,
    hash: std::string::String,
    version: u32,
}

impl Config {
    pub fn parse_config(path: &str) -> Config {
        let mut config = Config {
            services: Vec::new(),
            ..Default::default()
        };
        let raw_config = config.read_path(path);
        config.set_hash(&raw_config);
        config.parse_json(raw_config);
        config
    }

    fn set_hash(&mut self, content: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write(content.as_bytes());
        let hash = hasher.finish();
        self.hash = format!("{:x}", hash);
        hash
    }

    pub fn get_hash(&self) -> std::string::String {
        self.hash.clone()
    }

    fn parse_json(&mut self, raw_config: std::string::String) {
        let mut result: Vec<service::Service> = Vec::new();

        // @TODO handle error properly here
        let v: Vec<service::Service> = serde_json::from_str(raw_config.as_str()).unwrap();
        for val in v {
            log::debug!("Service with id='{}' added to the config pool", val.id);
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

    pub fn export_config_to_envoy(&self) -> EnvoyExportList {
        let (exportlist, errorlist): (Vec<_>, Vec<_>) = self
            .services
            .iter()
            .map(service::Service::export)
            .partition(|it| it.is_ok());

        if !errorlist.is_empty() {
            log::error!("Some data could not be exported");
            errorlist
                .into_iter()
                .map(Result::unwrap_err)
                .for_each(|err| log::error!("-> {}", err));
        }

        let mut result = Vec::with_capacity(exportlist.len());
        result.extend(exportlist.into_iter().flat_map(Result::unwrap));

        result
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn get_services(&self) -> ServicesList {
        self.services.clone()
    }

    pub fn import(&mut self, services: ServicesList, hash: std::string::String) {
        self.services = services;
        self.hash = hash;
        self.version += 1;
    }
}

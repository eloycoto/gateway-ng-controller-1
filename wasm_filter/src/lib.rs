use chrono::{DateTime, Utc};
use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MappingRules {
    pattern: std::string::String,
    http_method: std::string::String, // @TODO this should be a enum, maybe from hyper
    metric_system_name: std::string::String,
    delta: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Service {
    pub id: u32,
    pub hosts: Vec<std::string::String>,
    pub policies: Vec<std::string::String>,
    pub target_domain: std::string::String,
    pub proxy_rules: Vec<MappingRules>,
}

thread_local! {
    static CONFIG: RefCell<Service> = RefCell::new(Service::default());
}

pub fn get_config() -> Service {
    let mut config = Service::default();
    CONFIG.with(|c| {
        let oo = c.borrow();
        config = oo.clone();
    });
    return config;
}

pub fn import_config(config: &str) -> Service {
    let service: Service = serde_json::from_str(config).unwrap();
    return service;
}

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|context_id, _| -> Box<dyn HttpContext> {
        Box::new(HttpHeaders { context_id })
    });
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HelloWorld) });
}

struct HttpHeaders {
    context_id: u32,
}

impl Context for HttpHeaders {}

impl HttpContext for HttpHeaders {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        info!("#CONFIG::## {:?} ", get_config());
        for (name, value) in &self.get_http_request_headers() {
            info!("#{} -> {}: {}", self.context_id, name, value);
        }

        match self.get_http_request_header(":path") {
            Some(path) if path == "/hello" => {
                self.send_http_response(
                    200,
                    vec![("Hello", "World"), ("Powered-By", "proxy-wasm")],
                    Some(b"Hello, World!\n"),
                );
                Action::Pause
            }
            _ => Action::Continue,
        }
    }

    fn on_log(&mut self) {
        info!("#{} completed.", self.context_id);
    }
}

struct HelloWorld;

impl Context for HelloWorld {}

impl RootContext for HelloWorld {
    fn on_vm_start(&mut self, _: usize) -> bool {
        let config = self.get_configuration();
        let service = import_config(std::str::from_utf8(&config.unwrap()).unwrap());
        self.set_tick_period(Duration::from_secs(20));
        CONFIG.with(|c| {
            *c.borrow_mut() = service;
        });
        true
    }

    fn on_tick(&mut self) {
        let datetime: DateTime<Utc> = self.get_current_time().into();
        info!("Wasm filter tick: {}", datetime);
    }
}

use chrono::{DateTime, Utc};
use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;

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

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        for (name, value) in &self.get_http_response_headers() {
            info!("#{} <- {}: {}", self.context_id, name, value);
        }
        Action::Continue
    }

    fn on_log(&mut self) {
        info!("#{} completed.", self.context_id);
    }
}

struct HelloWorld;

impl Context for HelloWorld {}

impl RootContext for HelloWorld {
    fn on_vm_start(&mut self, _: usize) -> bool {
        log::info!("Hello, World!");
        self.set_tick_period(Duration::from_secs(20));
        true
    }

    fn on_tick(&mut self) {
        let datetime: DateTime<Utc> = self.get_current_time().into();
        info!("It's {}", datetime);
    }
}

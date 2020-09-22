//#![warn(clippy::all)]
#![deny(clippy::all)]

// use crate::envoy_cds::CDS;
mod cache;
mod configuration;
mod envoy_cds;
mod envoy_helpers;
mod protobuf;
mod service;

use crate::protobuf::envoy::service::cluster::v3::cluster_discovery_service_server::ClusterDiscoveryServiceServer;
use log::info;
use tonic::transport::Server;
use tonic::{Request, Status};

#[macro_use]
extern crate lazy_static;

fn intercept(req: Request<()>) -> Result<Request<()>, Status> {
    println!("Intercepting request: {:?}", req);
    Ok(req)
}

// @TODO review this static lifetime, does not looks correct here.
fn initialize_config_loader() -> bool {
    let mut initial_config = "".to_string();
    tokio::task::spawn_blocking(move || loop {
        let config = configuration::Config::parse_config("./log.json");
        // @TODO This clone here shouldn't happen
        let config_hash = config.clone().get_hash();
        if config_hash != initial_config {
            println!("Updating config at {:?}", std::time::Instant::now());
            cache::add_multiple(&mut config.export_config_to_envoy());
            cache::release();
            initial_config = config_hash
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
    });

    true
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // @TODO read env variable here:
    println!("Starting listening on 5000 port");
    let addr = "0.0.0.0:5000".parse().unwrap();

    let mut cds = envoy_cds::CDS::new();
    cds.subscribe();

    initialize_config_loader();

    info!("Envoy controller service listening on {}", addr);
    // info!("CDS service listening on {:?}", cds);

    Server::builder()
        .add_service(ClusterDiscoveryServiceServer::with_interceptor(
            cds, intercept,
        ))
        .serve(addr)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn we_actually_run_tests() {}
}

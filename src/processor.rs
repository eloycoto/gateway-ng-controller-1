use std::sync::{Arc, RwLock};

use crate::protobuf::envoy::service::cluster::v3::cluster_discovery_service_server::ClusterDiscoveryServiceServer;
use tonic::transport::Server;
use tonic::{Request, Status};

use crate::configuration;
use crate::envoy_cds;

#[derive(Default)]
pub struct MasterProcess {
    config: Arc<RwLock<configuration::Config>>,
}

impl MasterProcess {
    pub fn config_thread(&'_ self) {
        let mut initial_config = "".to_string();
        let cfg = Arc::clone(&self.config);
        tokio::task::spawn_blocking(move || loop {
            let config = &configuration::Config::parse_config("./log.json");
            if config.get_hash() != initial_config {
                initial_config = config.get_hash();

                let mut self_config = cfg.write().unwrap();
                self_config.import(config.get_services(), initial_config.clone());
                println!("Config update to version: {}", self_config.get_version());
            }
            std::thread::sleep(std::time::Duration::from_secs(5));
        });
    }

    pub async fn start(
        &mut self,
        addr: std::net::SocketAddr,
    ) -> Result<(), tonic::transport::Error> {
        {
            self.config_thread();
            //@TODO to delete  this wait until process start.
            std::thread::sleep(std::time::Duration::from_secs(3));

            //@TODO remove this debug section.
            let cfg = self.config.read().unwrap();
            println!(
                "Config--GetServices {:?} {:?}",
                cfg.get_services(),
                cfg.get_version()
            );

            fn intercept(req: Request<()>) -> Result<Request<()>, Status> {
                println!("Intercepting request: {:?}", req);
                Ok(req)
            }

            // Services sections
            let cds = envoy_cds::CDS::new(Arc::clone(&self.config));

            let res = Server::builder()
                .add_service(ClusterDiscoveryServiceServer::with_interceptor(
                    cds, intercept,
                ))
                .serve(addr)
                .await;
            return res;
        }
    }
}

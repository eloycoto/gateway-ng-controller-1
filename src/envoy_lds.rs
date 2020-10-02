use crate::protobuf::envoy::service::discovery::v3::{
    DeltaDiscoveryRequest, DeltaDiscoveryResponse, DiscoveryRequest, DiscoveryResponse,
};
use async_stream::try_stream;
use futures::Stream;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use tonic::{Request, Response, Status, Streaming};

use crate::configuration;
use crate::envoy_helpers;
use crate::protobuf::envoy::config::listener::v3::Listener;
use crate::protobuf::envoy::service::listener::v3::listener_discovery_service_server::ListenerDiscoveryService;

#[derive(Debug, Clone)]
pub struct LDS {
    listeners: Vec<Listener>,
    version: u32,
    config: Arc<RwLock<configuration::Config>>,
}

impl LDS {
    pub fn new(config: Arc<RwLock<configuration::Config>>) -> LDS {
        let mut lds = LDS {
            listeners: Vec::new(),
            version: 0,
            config: config,
        };
        lds.refresh_data_if_needed();
        return lds;
    }

    pub fn refresh_data_if_needed(&mut self) {
        let cfg = self.config.read().unwrap();
        if cfg.get_version() <= self.version {
            return;
        }

        let mut new_listeners: Vec<Listener> = Vec::new();
        let services = cfg.export_config_to_envoy();
        for k in &services {
            match &k.config {
                envoy_helpers::EnvoyResource::Cluster(_) => continue,
                envoy_helpers::EnvoyResource::Listener(l) => new_listeners.push(l.clone()),
            }
        }

        self.listeners = new_listeners.clone();
        self.version += cfg.get_version();
    }
}

#[tonic::async_trait]
impl ListenerDiscoveryService for LDS {
    type DeltaListenersStream = Pin<
        Box<
            dyn Stream<Item = Result<DeltaDiscoveryResponse, tonic::Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    type StreamListenersStream = Pin<
        Box<dyn Stream<Item = Result<DiscoveryResponse, tonic::Status>> + Send + Sync + 'static>,
    >;

    async fn delta_listeners(
        &self,
        _request: Request<Streaming<DeltaDiscoveryRequest>>,
    ) -> Result<Response<Self::DeltaListenersStream>, Status> {
        println!("Delta listener");
        Err(Status::unimplemented("not implemented"))
    }

    async fn stream_listeners(
        &self,
        _request: Request<tonic::Streaming<DiscoveryRequest>>,
    ) -> Result<Response<Self::StreamListenersStream>, Status> {
        let mut listeners: Vec<prost_types::Any> = Vec::new();

        for listener in &self.listeners {
            let mut buf = Vec::new();
            prost::Message::encode(listener, &mut buf).unwrap();
            listeners.push(prost_types::Any {
                type_url: "type.googleapis.com/envoy.config.listener.v3.Listener".to_string(),
                value: buf,
            });
        }

        let discovery = DiscoveryResponse {
            version_info: self.version.to_string(),
            type_url: "type.googleapis.com/envoy.config.listener.v3.Listener".to_string(),
            resources: listeners,
            // nonce: "nonce".to_string(),
            ..Default::default()
        };

        let output = try_stream! {
           yield discovery.clone();
        };

        return Ok(Response::new(
            Box::pin(output) as Self::StreamListenersStream
        ));
    }

    async fn fetch_listeners(
        &self,
        _request: Request<DiscoveryRequest>,
    ) -> Result<Response<DiscoveryResponse>, Status> {
        println!("Fetch_listener");
        Err(Status::unimplemented("not implemented"))
    }
}

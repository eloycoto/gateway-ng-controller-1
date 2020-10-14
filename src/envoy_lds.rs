use std::pin::Pin;
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll};
use std::thread;
use std::time;
use tokio::stream::Stream;
use tonic::{Request, Response, Status, Streaming};

use crate::configuration;
use crate::envoy_helpers;
use crate::protobuf::envoy::config::listener::v3::Listener;
use crate::protobuf::envoy::service::discovery::v3::{
    DeltaDiscoveryRequest, DeltaDiscoveryResponse, DiscoveryRequest, DiscoveryResponse,
};
use crate::protobuf::envoy::service::listener::v3::listener_discovery_service_server::ListenerDiscoveryService;

#[derive(Debug, Clone)]
pub struct LDS {
    listeners: Vec<Listener>,
    version: u32,
    config: Arc<RwLock<configuration::Config>>,
}

impl LDS {
    pub fn new(config: Arc<RwLock<configuration::Config>>) -> LDS {
        LDS {
            listeners: Vec::new(),
            version: 0,
            config,
        }
    }

    pub fn refresh_data(&mut self) {
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

        self.listeners = new_listeners;
        self.version = cfg.get_version();
    }
}

impl tokio::stream::Stream for LDS {
    type Item = Result<DiscoveryResponse, tonic::Status>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        ctx: &mut Context<'_>,
    ) -> Poll<Option<Result<DiscoveryResponse, tonic::Status>>> {
        let mut send_data = false;
        {
            let cfg = self.config.clone();
            let version = cfg.read().unwrap().get_version();
            if self.version != version {
                send_data = true;
            }
        }

        if !(send_data) {
            log::trace!("Sleep LDS because no config changes made");
            let waker = ctx.waker().clone();
            thread::spawn(move || {
                thread::sleep(time::Duration::from_secs(5));
                waker.wake();
            });
            return Poll::Pending;
        }

        log::info!("Refreshing LDS config due a version mistmatch");
        self.refresh_data();

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
            ..Default::default()
        };

        Poll::Ready(Some(Ok(discovery)))
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
        Ok(Response::new(
            Box::pin(self.clone()) as Self::StreamListenersStream
        ))
    }

    async fn fetch_listeners(
        &self,
        _request: Request<DiscoveryRequest>,
    ) -> Result<Response<DiscoveryResponse>, Status> {
        println!("Fetch_listener");
        Err(Status::unimplemented("not implemented"))
    }
}

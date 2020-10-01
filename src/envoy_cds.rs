use async_stream::try_stream;
use futures::Stream;
use std::pin::Pin;
use tonic::{Request, Response, Status, Streaming};

use crate::configuration;
use crate::envoy_helpers;
use std::sync::{Arc, RwLock};
// use crate::envoy_helpers::{EnvoyExport, EnvoyResource, EnvoyService};
use crate::protobuf::envoy::config::cluster::v3::Cluster;
use crate::protobuf::envoy::service::cluster::v3::cluster_discovery_service_server::ClusterDiscoveryService;
use crate::protobuf::envoy::service::discovery::v3::{
    DeltaDiscoveryRequest, DeltaDiscoveryResponse, DiscoveryRequest, DiscoveryResponse,
};

#[derive(Debug, Clone)]
pub struct CDS {
    clusters: Vec<Cluster>,
    version: u32,
    config: Arc<RwLock<configuration::Config>>,
}

impl CDS {
    pub fn new(config: Arc<RwLock<configuration::Config>>) -> CDS {
        let mut cds = CDS {
            clusters: Vec::new(),
            version: 0,
            config: config,
        };
        cds.refresh_data_if_needed();
        return cds;
    }

    pub fn refresh_data_if_needed(&mut self) {
        let cfg = self.config.read().unwrap();
        if cfg.get_version() <= self.version {
            return;
        }

        let mut new_clusters: Vec<Cluster> = Vec::new();
        let services = cfg.export_config_to_envoy();
        for k in &services {
            match &k.config {
                envoy_helpers::EnvoyResource::Cluster(c) => new_clusters.push(c.clone()),
                envoy_helpers::EnvoyResource::Listener(_) => continue,
            }
        }

        self.clusters = new_clusters.clone();
        self.version += cfg.get_version();
    }
}

#[tonic::async_trait]
impl ClusterDiscoveryService for CDS {
    type StreamClustersStream = Pin<
        Box<dyn Stream<Item = Result<DiscoveryResponse, tonic::Status>> + Send + Sync + 'static>,
    >;

    type DeltaClustersStream = Pin<
        Box<
            dyn Stream<Item = Result<DeltaDiscoveryResponse, tonic::Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    async fn stream_clusters(
        &self,
        _request: Request<tonic::Streaming<DiscoveryRequest>>,
    ) -> Result<Response<Self::StreamClustersStream>, Status> {
        let mut clusters: Vec<prost_types::Any> = Vec::new();

        for cds_cluster in &self.clusters {
            let mut buf = Vec::new();
            prost::Message::encode(cds_cluster, &mut buf).unwrap();
            clusters.push(prost_types::Any {
                type_url: "type.googleapis.com/envoy.config.cluster.v3.Cluster".to_string(),
                value: buf,
            });
        }

        let discovery = DiscoveryResponse {
            version_info: self.version.to_string(),
            type_url: "type.googleapis.com/envoy.config.cluster.v3.Cluster".to_string(),
            resources: clusters,
            nonce: "nonce".to_string(),
            ..Default::default()
        };

        let output = try_stream! {
           yield discovery.clone();
        };

        Ok(Response::new(Box::pin(output) as Self::StreamClustersStream))
    }

    async fn delta_clusters(
        &self,
        _request: Request<Streaming<DeltaDiscoveryRequest>>,
    ) -> Result<Response<Self::DeltaClustersStream>, Status> {
        println!("Delta cluster");
        Err(Status::unimplemented("not implemented"))
    }

    async fn fetch_clusters(
        &self,
        _request: Request<DiscoveryRequest>,
    ) -> Result<Response<DiscoveryResponse>, Status> {
        println!("Fetch_cluster");
        Err(Status::unimplemented("not implemented"))
    }
}

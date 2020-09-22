use async_stream::try_stream;
use futures::Stream;
use std::pin::Pin;
use tonic::{Request, Response, Status, Streaming};

use crate::cache;
use crate::envoy_helpers::{EnvoyExport, EnvoyResource};
use crate::protobuf::envoy::config::cluster::v3::Cluster;
use crate::protobuf::envoy::service::cluster::v3::cluster_discovery_service_server::ClusterDiscoveryService;
use crate::protobuf::envoy::service::discovery::v3::{
    DeltaDiscoveryRequest, DeltaDiscoveryResponse, DiscoveryRequest, DiscoveryResponse,
};

// pub fn callback(data: Vec<EnvoyExport>) {
//     println!("Refresh data --CDS CALLBACK",);
// }

#[derive(Debug, Clone)]
pub struct CDS {
    clusters: Vec<Cluster>,
    version: u32,
}

impl CDS {
    pub fn new() -> CDS {
        CDS {
            clusters: Vec::new(),
            version: 0,
        }
    }

    fn refresh_data(&mut self, data: Vec<EnvoyExport>) {
        let mut new_clusters: Vec<Cluster> = Vec::new();
        for k in &data {
            match &k.config {
                EnvoyResource::Cluster(c) => new_clusters.push(c.clone()),
            }
        }
        self.clusters = new_clusters.clone();
        self.version += 1;
    }

    pub fn subscribe(&'_ mut self) {
        let mut object = Box::new(self.clone());
        let callback = move |x| object.refresh_data(x);
        cache::subcribe_release(callback);
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

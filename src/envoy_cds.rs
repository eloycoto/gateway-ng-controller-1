use async_stream::try_stream;
use futures::Stream;
use prost_types::Duration;
use std::pin::Pin;
use tonic::{Request, Response, Status, Streaming};

use crate::cache;

use crate::protobuf::envoy::config::cluster::v3::Cluster;
use crate::protobuf::envoy::service::cluster::v3::cluster_discovery_service_server::ClusterDiscoveryService;
use crate::protobuf::envoy::service::discovery::v3::{
    DeltaDiscoveryRequest, DeltaDiscoveryResponse, DiscoveryRequest, DiscoveryResponse,
};

#[derive(Debug, Clone)]
pub struct CDS {
    clusters: Vec<std::string::String>,
}

impl CDS {
    pub fn new() -> CDS {
        CDS {
            clusters: Vec::new(),
        }
    }

    fn refresh_data(&self) {
        let data = cache::read_all();
        println!("Data on refresh is::{:?}", data);
        // @TODO implement here the list of clusters
    }

    pub fn subscribe(&'_ mut self) {
        let object = Box::new(self.clone());
        let callback = move || object.refresh_data();
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

        // @TODO change here how the info is retrieved using some kind of cache.
        // implement a method for encode that?
        let cluster = Cluster {
            name: "google.com".to_string(),
            connect_timeout: Some(Duration {
                seconds: 1,
                nanos: 0,
            }),
            ..Default::default()
        };

        let mut buf = Vec::new();
        prost::Message::encode(&cluster, &mut buf).unwrap();

        clusters.push(prost_types::Any {
            type_url: "type.googleapis.com/envoy.config.cluster.v3.Cluster".to_string(),
            // value: "foo".to_string().into_bytes(),
            value: buf,
        });

        let discovery = DiscoveryResponse {
            version_info: "1".to_string(),
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

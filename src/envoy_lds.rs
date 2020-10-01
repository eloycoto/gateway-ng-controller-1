// use crate::protobuf::envoy::service::cluster::v3::cluster_discovery_service_server::ClusterDiscoveryService;
use crate::protobuf::envoy::service::listener::v3::listener_discovery_service_server::ListenerDiscoveryService;

use async_stream::try_stream;
use futures::Stream;
use std::pin::Pin;

use crate::protobuf::envoy::service::discovery::v3::{
    DeltaDiscoveryRequest, DeltaDiscoveryResponse, DiscoveryRequest, DiscoveryResponse,
};

use tonic::{Request, Response, Status, Streaming};

#[derive(Debug, Clone)]
pub struct LDS {
    version: u32,
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
        println!("Delta listener");
        Err(Status::unimplemented("not implemented"))
    }

    async fn fetch_listeners(
        &self,
        _request: Request<DiscoveryRequest>,
    ) -> Result<Response<DiscoveryResponse>, Status> {
        println!("Fetch_listener");
        Err(Status::unimplemented("not implemented"))
    }
}

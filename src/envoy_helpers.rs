use crate::protobuf::envoy::config::cluster::v3::Cluster;
use crate::protobuf::envoy::config::listener::v3::Listener;
use url::{ParseError, Url};
pub type EnvoyExportList = Vec<EnvoyExport>;

// These are structs to export config to the config:cache
// Variables shouldn't be public at all.
#[derive(Debug, Clone)]
pub struct EnvoyExport {
    pub key: std::string::String,
    pub config: EnvoyResource,
}

#[derive(Debug, Clone)]
pub enum EnvoyResource {
    Cluster(Cluster),
    Listener(Listener),
}

use crate::protobuf::envoy::config::cluster::v3::cluster::ClusterDiscoveryType;
use crate::protobuf::envoy::config::core::v3::address::Address as AddressType;
use crate::protobuf::envoy::config::core::v3::socket_address::PortSpecifier;
use crate::protobuf::envoy::config::core::v3::Address;
use crate::protobuf::envoy::config::core::v3::SocketAddress;
use crate::protobuf::envoy::config::endpoint::v3::lb_endpoint::HostIdentifier;
use crate::protobuf::envoy::config::endpoint::v3::ClusterLoadAssignment;
use crate::protobuf::envoy::config::endpoint::v3::Endpoint;
use crate::protobuf::envoy::config::endpoint::v3::LbEndpoint;
use crate::protobuf::envoy::config::endpoint::v3::LocalityLbEndpoints;

use crate::util;

use prost_types::Duration;

pub fn get_envoy_cluster(name: std::string::String, target_url: std::string::String) -> Cluster {
    let target_host = Url::parse(target_url.as_str()).unwrap();
    let port = match target_host.port() {
        Some(port) => port,
        _ => {
            if target_host.scheme() == "https" {
                443
            } else {
                80
            }
        }
    };

    let socketaddress = AddressType::SocketAddress(SocketAddress {
        address: target_host.host_str().unwrap().to_string(),
        port_specifier: Some(PortSpecifier::PortValue(port as u32)),
        ..Default::default()
    });

    Cluster {
        name: name.clone(),
        connect_timeout: Some(Duration {
            seconds: 1,
            nanos: 0,
        }),
        cluster_discovery_type: Some(ClusterDiscoveryType::Type(2)),
        dns_refresh_rate: Some(core::time::Duration::from_secs(60).into()),
        load_assignment: Some(ClusterLoadAssignment {
            cluster_name: name.clone(),
            endpoints: vec![LocalityLbEndpoints {
                lb_endpoints: vec![LbEndpoint {
                    host_identifier: Some(HostIdentifier::Endpoint(Endpoint {
                        address: Some(Address {
                            address: Some(socketaddress),
                        }),
                        // hostname: self.target_domain.to_string(),
                        ..Default::default()
                    })),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}

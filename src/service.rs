use prost_types::Duration;
use serde::{Deserialize, Serialize};

use crate::protobuf::envoy::config::cluster::v3::Cluster;
use crate::protobuf::envoy::config::endpoint::v3::lb_endpoint::HostIdentifier;
use crate::protobuf::envoy::config::endpoint::v3::ClusterLoadAssignment;
use crate::protobuf::envoy::config::endpoint::v3::Endpoint;
use crate::protobuf::envoy::config::endpoint::v3::LbEndpoint;
use crate::protobuf::envoy::config::endpoint::v3::LocalityLbEndpoints;

use crate::envoy_helpers::{EnvoyExport, EnvoyResource};

// @TODO target domain connect_timeout
// @TODO optional fields
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Service {
    pub id: u32,
    pub hosts: Vec<std::string::String>,
    pub policies: Vec<std::string::String>,
    pub target_domain: std::string::String,
}

impl Service {
    pub fn export(&self) -> Vec<EnvoyExport> {
        let mut result: Vec<EnvoyExport> = Vec::new();
        let cluster = self.export_clusters();

        result.push(EnvoyExport {
            key: format!("service::id::{}::cluster", self.id),
            config: EnvoyResource::Cluster(cluster),
        });

        return result;
    }

    // TODO unimplemented
    // TODO Remove dead code lint.
    #[allow(dead_code)]
    fn export_listener(&self) -> bool {
        unimplemented!()
    }

    fn export_clusters(&self) -> Cluster {
        Cluster {
            name: self.target_domain.to_string(),
            connect_timeout: Some(Duration {
                seconds: 1,
                nanos: 0,
            }),
            lb_policy: 0,
            load_assignment: Some(ClusterLoadAssignment {
                cluster_name: self.target_domain.to_string(),
                endpoints: vec![LocalityLbEndpoints {
                    lb_endpoints: vec![LbEndpoint {
                        host_identifier: Some(HostIdentifier::Endpoint(Endpoint {
                            hostname: self.target_domain.to_string(),
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
}

use crate::protobuf::envoy::config::cluster::v3::Cluster;

// These are structs to export config to the config:cache
#[derive(Debug, Clone)]
pub struct EnvoyExport {
    pub key: std::string::String,
    pub config: EnvoyResource,
}

#[derive(Debug, Clone)]
pub enum EnvoyResource {
    Cluster(Cluster),
    // Listener(std::string::String), // @TODO to implement listener section.
}

use crate::protobuf::envoy::config::cluster::v3::Cluster;

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
}

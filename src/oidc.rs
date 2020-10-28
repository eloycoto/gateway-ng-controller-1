use crate::protobuf::envoy::config::cluster::v3::Cluster;
use crate::protobuf::envoy::config::core::v3::http_uri::HttpUpstreamType;
use crate::protobuf::envoy::config::core::v3::HttpUri;
use crate::protobuf::envoy::config::route::v3::route_match::PathSpecifier;
use crate::protobuf::envoy::config::route::v3::RouteMatch;
use crate::protobuf::envoy::extensions::filters::http::jwt_authn::v3::jwt_provider::JwksSourceSpecifier;
use crate::protobuf::envoy::extensions::filters::http::jwt_authn::v3::jwt_requirement::RequiresType;
use crate::protobuf::envoy::extensions::filters::http::jwt_authn::v3::JwtAuthentication;
use crate::protobuf::envoy::extensions::filters::http::jwt_authn::v3::JwtHeader;
use crate::protobuf::envoy::extensions::filters::http::jwt_authn::v3::JwtProvider;
use crate::protobuf::envoy::extensions::filters::http::jwt_authn::v3::JwtRequirement;
use crate::protobuf::envoy::extensions::filters::http::jwt_authn::v3::RemoteJwks;
use crate::protobuf::envoy::extensions::filters::http::jwt_authn::v3::RequirementRule;

use crate::envoy_helpers::get_envoy_cluster;
use curl::easy::Easy;
use prost_types::Duration;
use serde_json::{Map, Value};

#[derive(Default)]
pub struct OIDCConfig {
    issuer: std::string::String,
    audiences: Vec<std::string::String>,
    certs: std::string::String,
    cluster: std::string::String,
}

impl OIDCConfig {
    pub fn new(issuer: std::string::String) -> OIDCConfig {
        return OIDCConfig {
            issuer: issuer,
            audiences: Vec::new(),
            ..Default::default()
        };
    }

    fn request(&self, target_url: &str) -> String {
        let mut dst = Vec::new();
        let mut easy = Easy::new();
        {
            easy.url(target_url).unwrap();
            easy.ssl_verify_host(false).unwrap();
            easy.ssl_verify_peer(false).unwrap();
            let mut transfer = easy.transfer();
            transfer
                .write_function(|data| {
                    dst.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }
        return String::from_utf8(dst.to_vec().clone()).unwrap();
    }

    pub fn import_config(&mut self, service_id: u32) {
        let data =
            self.request(format!("{}/.well-known/openid-configuration", self.issuer).as_str());
        let key_values: std::collections::HashMap<String, serde_json::Value> =
            serde_json::from_str(&data.as_str()).unwrap();

        self.certs = key_values
            .get("jwks_uri")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        self.cluster = format!("Service::{}::OIDC", service_id);
        self.audiences.push("admin-cli".to_string());
    }

    pub fn export(&mut self, service_id: u32) -> (JwtAuthentication, Cluster) {
        self.import_config(service_id);

        let cluster = get_envoy_cluster(
            format!("Service::{}::OIDC", service_id),
            self.issuer.clone(),
        );

        let provider = JwtProvider {
            issuer: self.issuer.clone(),
            from_headers: vec![JwtHeader {
                name: "Authorization".to_string(),
                value_prefix: "Bearer ".to_string(),
            }],
            audiences: self.audiences.clone(),
            forward: false,
            jwks_source_specifier: Some(JwksSourceSpecifier::RemoteJwks(RemoteJwks {
                http_uri: Some(HttpUri {
                    uri: self.certs.clone(),
                    timeout: Some(Duration {
                        seconds: 100,
                        nanos: 0,
                    }),
                    http_upstream_type: Some(HttpUpstreamType::Cluster(
                        format!("keycloak").to_string(),
                    )),
                    ..Default::default()
                }),
                cache_duration: None,
            })),
            ..Default::default()
        };

        let provider_name = format!("provider::service::{}", service_id);
        let mut providers = std::collections::HashMap::new();
        providers.insert(provider_name.clone(), provider);

        let filter = JwtAuthentication {
            providers: providers,
            rules: vec![RequirementRule {
                r#match: Some(RouteMatch {
                    path_specifier: Some(PathSpecifier::Prefix("/".to_string())),
                    ..Default::default()
                }),
                requires: Some(JwtRequirement {
                    requires_type: Some(RequiresType::ProviderName(provider_name.clone())),
                }),
            }],
            ..Default::default()
        };
        return (filter, cluster);
    }
}

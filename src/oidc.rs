use crate::protobuf::envoy::config::cluster::v3::Cluster;
use crate::protobuf::envoy::config::core::v3::http_uri::HttpUpstreamType;
use crate::protobuf::envoy::config::core::v3::HttpUri;
use crate::protobuf::envoy::config::route::v3::route_match::PathSpecifier;
use crate::protobuf::envoy::config::route::v3::RouteMatch;
use crate::protobuf::envoy::extensions::filters::http::jwt_authn::v3::jwt_provider::JwksSourceSpecifier;
use crate::protobuf::envoy::extensions::filters::http::jwt_authn::v3::jwt_requirement::RequiresType;
use crate::protobuf::envoy::extensions::filters::http::jwt_authn::v3::JwtAuthentication;
use crate::protobuf::envoy::extensions::filters::http::jwt_authn::v3::JwtProvider;
use crate::protobuf::envoy::extensions::filters::http::jwt_authn::v3::JwtRequirement;
use crate::protobuf::envoy::extensions::filters::http::jwt_authn::v3::RemoteJwks;
use crate::protobuf::envoy::extensions::filters::http::jwt_authn::v3::RequirementRule;
use prost_types::Duration;

use crate::envoy_helpers::get_envoy_cluster;
use std::collections::HashMap;

#[derive(Default)]
pub struct OIDCConfig {
    issuer: std::string::String,
    audiences: Vec<std::string::String>,
    certs: std::string::String,
    cluster: std::string::String,
}

async fn example() -> i32 {
    42
}

impl OIDCConfig {
    pub fn new(issuer: std::string::String) -> OIDCConfig {
        return OIDCConfig {
            issuer: issuer,
            audiences: Vec::new(),
            ..Default::default()
        };
    }

    pub fn import_config(&mut self) {
        // futures::executor::block_on(self.get_config());
        self.certs = "https://keycloak-redhat-sso.apps.dev-eng-ocp4-5.dev.3sca.net/auth/realms/Eloy-wasm-test/protocol/openid-connect/certs".to_string();
        self.cluster = "keycloak".to_string();
        self.audiences.push("admin-cli".to_string());
    }

    pub fn export(&mut self, service_id: u32) -> (JwtAuthentication, Cluster) {
        self.import_config();

        let cluster = get_envoy_cluster(
            format!("Service::{}::OIDC", service_id),
            self.issuer.clone(),
        );

        let provider = JwtProvider {
            issuer: self.issuer.clone(),
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

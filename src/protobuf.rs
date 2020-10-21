// Triggered currently by config::core::v3 with the GoogleGrpc type
#![allow(clippy::large_enum_variant)]

#[path = "protobuf"]
pub mod google {
    #[path = "google.api.rs"]
    pub mod api;
    #[path = "google.protobuf.rs"]
    pub mod protobuf;
    #[path = "google.rpc.rs"]
    pub mod rpc;
}

#[path = "protobuf"]
pub mod udpa {
    #[path = "udpa.annotations.rs"]
    pub mod annotations;
    #[path = "."]
    pub mod core {
        #[path = "udpa.core.v1.rs"]
        pub mod v1;
    }
}

#[path = "protobuf"]
pub mod envoy {
    #[path = "envoy.annotations.rs"]
    pub mod annotations;

    #[path = "."]
    pub mod config {
        #[path = "."]
        pub mod accesslog {
            #[path = "envoy.config.accesslog.v3.rs"]
            pub mod v3;
        }

        #[path = "."]
        pub mod core {
            #[path = "envoy.config.core.v3.rs"]
            pub mod v3;
        }

        #[path = "."]
        pub mod cluster {
            #[path = "envoy.config.cluster.v3.rs"]
            pub mod v3;
        }

        #[path = "."]
        pub mod endpoint {
            #[path = "envoy.config.endpoint.v3.rs"]
            pub mod v3;
        }

        #[path = "."]
        pub mod listener {
            #[path = "envoy.config.listener.v3.rs"]
            pub mod v3;
        }

        #[path = "."]
        pub mod route {
            #[path = "envoy.config.route.v3.rs"]
            pub mod v3;
        }

        #[path = "."]
        pub mod trace {
            #[path = "envoy.config.trace.v3.rs"]
            pub mod v3;
        }
    }

    #[path = "."]
    pub mod extensions {

        #[path = "."]
        pub mod wasm {
            #[path = "envoy.extensions.wasm.v3.rs"]
            pub mod v3;
        }

        #[path = "."]
        pub mod filters {
            #[path = "."]
            pub mod network {
                #[path = "."]
                pub mod http_connection_manager {
                    #[path = "envoy.extensions.filters.network.http_connection_manager.v3.rs"]
                    pub mod v3;
                }
            }

            #[path = "."]
            pub mod http {
                #[path = "."]
                pub mod router {
                    #[path = "envoy.extensions.filters.http.router.v3.rs"]
                    pub mod v3;
                }

                #[path = "."]
                pub mod wasm {
                    #[path = "envoy.extensions.filters.http.wasm.v3.rs"]
                    pub mod v3;
                }
            }
        }
    }

    #[path = "."]
    pub mod service {
        #[path = "."]
        pub mod cluster {
            #[path = "envoy.service.cluster.v3.rs"]
            pub mod v3;
        }

        #[path = "."]
        pub mod discovery {
            #[path = "envoy.service.discovery.v3.rs"]
            pub mod v3;
        }

        #[path = "."]
        pub mod listener {
            #[path = "envoy.service.listener.v3.rs"]
            pub mod v3;
        }
    }

    #[path = "."]
    pub mod r#type {
        #[path = "."]
        pub mod matcher {
            #[path = "envoy.r#type.matcher.v3.rs"]
            pub mod v3;
        }

        #[path = "."]
        pub mod metadata {
            #[path = "envoy.r#type.metadata.v3.rs"]
            pub mod v3;
        }

        #[path = "."]
        pub mod tracing {
            #[path = "envoy.r#type.tracing.v3.rs"]
            pub mod v3;
        }

        #[path = "envoy.r#type.v3.rs"]
        pub mod v3;
    }
}

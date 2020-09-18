// Triggered currently by config::core::v3 with the GoogleGrpc type
#![allow(clippy::large_enum_variant)]

pub mod google {
    pub mod api {
        include!("protobuf/google.api.rs");
    }
    pub mod protobuf {
        include!("protobuf/google.protobuf.rs");
    }
    pub mod rpc {
        include!("protobuf/google.rpc.rs");
    }
}

pub mod udpa {
    pub mod annotations {
        include!("protobuf/udpa.annotations.rs");
    }
    pub mod core {
        pub mod v1 {
            include!("protobuf/udpa.core.v1.rs");
        }
    }
}

pub mod envoy {

    pub mod annotations {
        include!("protobuf/envoy.annotations.rs");
    }

    pub mod config {
        pub mod core {
            pub mod v3 {
                include!("protobuf/envoy.config.core.v3.rs");
            }
        }

        pub mod cluster {
            pub mod v3 {
                include!("protobuf/envoy.config.cluster.v3.rs");
            }
        }

        pub mod endpoint {
            pub mod v3 {
                include!("protobuf/envoy.config.endpoint.v3.rs");
            }
        }
    }

    pub mod service {
        pub mod cluster {
            pub mod v3 {
                include!("protobuf/envoy.service.cluster.v3.rs");
            }
        }

        pub mod discovery {
            pub mod v3 {
                include!("protobuf/envoy.service.discovery.v3.rs");
            }
        }
    }

    pub mod r#type {
        pub mod matcher {
            pub mod v3 {
                include!("protobuf/envoy.r#type.matcher.v3.rs");
            }
        }
        pub mod v3 {
            include!("protobuf/envoy.r#type.v3.rs");
        }
    }
}

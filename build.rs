fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=protos");
    // Best effort in creating the directory - could fail for many reasons
    let _ = std::fs::create_dir("src/protobuf");
    // Note: tonic_build by default uses rustfmt to prettify sources
    tonic_build::configure().out_dir("src/protobuf").compile(
        &[
            "./protos/envoyproxy/data-plane-api/envoy/config/cluster/v3/cluster.proto",
            "./protos/envoyproxy/data-plane-api/envoy/config/listener/v3/listener.proto",
            "./protos/envoyproxy/data-plane-api/envoy/service/cluster/v3/cds.proto",
            "./protos/envoyproxy/data-plane-api/envoy/service/listener/v3/lds.proto",
            "./protos/envoyproxy/data-plane-api/envoy/config/endpoint/v3/endpoint.proto",
        ],
        &[
            "./protos/envoyproxy/data-plane-api/",
            "./protos/googleapis/",
            "./protos/envoyproxy/protoc-gen-validate/",
            "./protos/cncf/udpa/",
        ],
    )?;

    Ok(())
}

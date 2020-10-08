#![deny(clippy::all)]

// use crate::envoy_cds::CDS;
mod configuration;
mod envoy_cds;
mod envoy_helpers;
mod envoy_lds;
mod processor;
// rustfmt stable will break down with #[path = "..."] in modules, so skip
// this module for now. See https://github.com/rust-lang/rustfmt/issues/4446.
#[rustfmt::skip]
mod protobuf;
mod service;

use processor::MasterProcess;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let mut master_process = MasterProcess::default();
    master_process
        .start("0.0.0.0:5000".parse().unwrap())
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn we_actually_run_tests() {}
}

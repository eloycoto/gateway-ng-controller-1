/// Common helpers
use anyhow::{anyhow, Result};

use ring::digest::{Context, Digest, SHA256};
use std::io::Read;

pub(crate) mod file_utils {

    pub(self) use super::*;

    pub fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
        let mut context = Context::new(&SHA256);
        let mut buffer = [0; 1024];

        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            context.update(&buffer[..count]);
        }

        Ok(context.finish())
    }
}

pub(crate) mod host_port {
    pub(self) use super::*;

    // Just pass in host:port string slices - do not use with full URLs
    // "host", "host:80", "127.0.0.1:80" and ":80" are all acceptable,
    // with the latter defaulting to host "0.0.0.0".
    pub fn parse(address: &str) -> Result<(&str, Option<u32>)> {
        let mut host_port = address.split(':');
        let host = match host_port.next() {
            Some(host) if !host.is_empty() => host,
            _ => "0.0.0.0",
        };
        let port = host_port
            .next()
            .map(|portstr| portstr.parse::<u32>())
            .transpose()?;

        match host_port.next() {
            None => Ok((host, port)),
            // More than one ':' character, so error out
            _ => Err(anyhow!(
                "incorrect format for host:port entry '{}'",
                address
            )),
        }
    }

    #[cfg(test)]
    mod tests {
        pub(self) use super::*;

        mod helpers {
            pub(self) use super::*;

            pub fn parse_unwrap(address: &str) -> (&str, Option<u32>) {
                let result = parse(address);
                assert!(
                    result.is_ok(),
                    "failed to parse '{}': {:?}",
                    address,
                    result
                );
                result.unwrap()
            }
        }

        #[test]
        fn parse_defaults_to_0_0_0_0_when_empty_host() {
            for &addr in ["", ":80"].iter() {
                let (host, _) = helpers::parse_unwrap(addr);
                assert_eq!(host, "0.0.0.0");
            }
        }

        #[test]
        fn parse_returns_correct_hostname() {
            for &addr in ["host.abc.xyz", "host.abc.xyz:80"].iter() {
                let (host, port) = helpers::parse_unwrap(addr);
                assert_eq!(host, "host.abc.xyz");
                assert_eq!(port.unwrap_or(80), 80);
            }
        }

        #[test]
        fn parse_returns_correct_ip_address_as_str() {
            for &addr in ["192.168.1.1", "192.168.1.1:80"].iter() {
                let (host, port) = helpers::parse_unwrap(addr);
                assert_eq!(host, "192.168.1.1");
                assert_eq!(port.unwrap_or(80), 80);
            }
        }

        #[test]
        fn parse_returns_none_port_if_not_present() {
            for &addr in ["", "host"].iter() {
                let (_, port) = helpers::parse_unwrap(addr);
                assert!(
                    port.is_none(),
                    "port in {} has been parsed correctly: {:?}",
                    addr,
                    port
                );
            }
        }

        #[test]
        fn parse_returns_error_if_port_isnt_an_unsigned_number() {
            for &addr in [
                "host:",
                "host:abc",
                "host:123a",
                "host:-1",
                "host:0x12",
                "host:0b11011011",
            ]
            .iter()
            {
                let result = parse(addr);
                assert!(
                    result.is_err(),
                    "successfully (and unexpectedly) parsed {}: {:?}",
                    addr,
                    result
                );
            }
        }

        #[test]
        fn parse_returns_some_port_if_it_is_an_unsigned_number() {
            for &addr in [":8080", "host:8080"].iter() {
                let (_, port) = helpers::parse_unwrap(addr);
                assert!(port.is_some(), "failed to parse port in {}", addr);
                let port = port.unwrap();
                assert_eq!(
                    port, 8080,
                    "failed to parse correctly port in {} as 8080: got {:?}",
                    addr, port
                );
            }
        }
    }
}

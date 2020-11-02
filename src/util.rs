/// Common helpers
use anyhow::Result;

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

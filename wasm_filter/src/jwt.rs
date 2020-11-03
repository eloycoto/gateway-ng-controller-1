use proxy_wasm::traits::*;
use proxy_wasm::types::*;

use base64::decode;

#[derive(Debug, Default)]
pub struct Rules {
    path: String,
    claim: String,       // Move to liquid
    claim_value: String, // Move to liquid
    allow: bool,
}
impl Rules {
    pub fn matches(&self, path: String, jwt: PayloadType) -> bool {
        if self.path != path {
            return false;
        }
        match jwt.get(self.claim.as_str()) {
            None => false,
            Some(j) => {
                // @TODO at some point of time, this should be a liquid template.
                if j.as_str().unwrap() == self.claim_value.as_str() {
                    return true;
                }
                false
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct JWTConfig {
    pub rules: Vec<Rules>,
}

#[derive(Debug, Default)]
pub struct JWT {
    pub context_id: u32,
    pub config: JWTConfig,
}

pub type PayloadType = std::collections::HashMap<String, serde_json::Value>;

impl JWT {
    pub fn new(context_id: u32) -> JWT {
        JWT {
            context_id,
            config: JWTConfig {
                ..Default::default()
            },
        }
    }

    pub fn config(&mut self) {
        self.config.rules = Vec::new();
        self.config.rules.push(Rules {
            path: "/headers".to_string(),
            claim: "name".to_string(),
            claim_value: "John Doe".to_string(),
            allow: false,
        });
    }

    pub fn get_jwt_token(&self) -> Result<PayloadType, anyhow::Error> {
        // @TODO: This is a shit-show! Sorry!
        // This should be changed in two different ways:
        // 1) and for me, the best one, is to be able to read this property from envoy.
        // https://github.com/envoyproxy/envoy/blob/bd73f3c4da0efffb2593d7c9ecf87788856dc052/source/extensions/filters/http/jwt_authn/filter.cc#L104
        // 2) Use any decoder from any crate, and here we have the problems with SSL* things, that
        //    are not available on Envoy.
        //
        // as you can imagine, the verification happens on jwt_authn filter :-)
        let raw_header = self.get_http_request_header("Authorization");
        if raw_header.is_none() {
            return Err(anyhow::Error::msg("Failed to get Bearer token"));
        }

        let s = raw_header.unwrap();
        let result: Vec<_> = s.split_whitespace().collect();
        if result.len() != 2 {
            return Err(anyhow::Error::msg("Failed to extract bearer token"));
        }

        let decoded_token: Vec<_> = result.get(1).unwrap().split(".").collect();
        let raw_payload = decode(decoded_token.get(1).unwrap())?;

        let payload: PayloadType = serde_json::from_slice(raw_payload.as_slice())?;

        return Ok(payload);
    }

    fn get_path(&self) -> Option<std::string::String> {
        return self.get_http_request_header(":path");
    }
}

impl Context for JWT {}

impl HttpContext for JWT {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        // @TODO to be removed until HTTP_CONTEXT can have metadata attached
        self.config();

        let jwt_token = self.get_jwt_token();
        if jwt_token.is_err() {
            log::warn!("Error on JWT auth: '{:?}'", jwt_token);
            self.send_http_response(403, vec![], Some(b"Access forbidden.\n"));
            return Action::Pause;
        }
        let token = jwt_token.unwrap();
        let mut result = false;

        for rule in &self.config.rules {
            if rule.matches(self.get_path().unwrap(), token.clone()) {
                result = true;
            }
        }

        if result == true {
            return Action::Continue;
        }

        self.send_http_response(403, vec![], Some(b"Access forbidden.\n"));
        return Action::Pause;
    }
}

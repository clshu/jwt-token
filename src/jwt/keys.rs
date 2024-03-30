#![allow(unused)]

use dotenv_codegen::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = dotenv!("APP_SECRET");
    Keys::new(secret.as_bytes())
});

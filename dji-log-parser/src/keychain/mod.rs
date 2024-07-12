use base64::engine::general_purpose::STANDARD as Base64Standard;
use base64::Engine as _;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod api;
mod feature_point;

pub use api::*;
pub use feature_point::FeaturePoint;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EncodedKeychainEntry {
    pub feature_point: FeaturePoint,
    pub aes_ciphertext: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeychainEntry {
    pub feature_point: FeaturePoint,
    pub aes_key: String,
    pub aes_iv: String,
}

/// `Keychain` serves as more convenient access to decrypt `Record` instances.
/// It associates each `FeaturePoint` with its corresponding AES initialization vector (IV)
/// and encryption key. In this hashmap, each `FeaturePoint` is linked to a tuple containing
/// the AES IV and key as array of bytes.
pub struct Keychain(HashMap<FeaturePoint, (Vec<u8>, Vec<u8>)>);

impl Keychain {
    pub fn empty() -> Self {
        Keychain(HashMap::new())
    }

    pub fn from_entries(keychain_entries: &Vec<KeychainEntry>) -> Self {
        Keychain(
            keychain_entries
                .into_iter()
                .map(|entry| {
                    (
                        entry.feature_point,
                        (
                            Base64Standard.decode(&entry.aes_iv).unwrap_or_default(),
                            Base64Standard.decode(&entry.aes_key).unwrap_or_default(),
                        ),
                    )
                })
                .collect(),
        )
    }

    pub fn get(&self, key: &FeaturePoint) -> Option<&(Vec<u8>, Vec<u8>)> {
        self.0.get(key)
    }

    pub fn insert(
        &mut self,
        key: FeaturePoint,
        value: (Vec<u8>, Vec<u8>),
    ) -> Option<(Vec<u8>, Vec<u8>)> {
        self.0.insert(key, value)
    }
}

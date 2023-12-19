use std::fmt::write;
use serde::{Serialize, Deserialize, ser::Serializer};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum EdgeCipher {
    None = 1,
    Twofish,
    AES,
    ChaCha20,
    Speck,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EdgeConfig {
    pub community: String,
    pub supernode_addr: String,
    pub encryption_key: Option<String>,
    pub address: Option<String>,
    pub mac: Option<String>,
    pub cipher: Option<EdgeCipher>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub public_key: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum EdgeError {
    CommunityIsNull,
    SupernodeAddressIsNull,
    InvalidAddress,
    InvalidMac,
    InvalidCipher,
    Unknown,
}

impl std::fmt::Display for EdgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EdgeError::CommunityIsNull => write!(f, "CommunityIsNull"),
            EdgeError::SupernodeAddressIsNull => write!(f, "SupernodeAddressIsNull"),
            EdgeError::InvalidAddress => write!(f, "InvalidAddress"),
            EdgeError::InvalidCipher => write!(f, "InvalidCipher"),
            EdgeError::InvalidMac => write!(f, "InvalidMac"),
            EdgeError::Unknown => write!(f, "Unknown"),
        }
    }
}

// we must manually implement serde::Serialize
impl Serialize for EdgeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub const DEFAULT_MTU: i32 = 1290;

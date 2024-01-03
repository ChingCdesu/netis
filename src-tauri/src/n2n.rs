use std::cell::RefCell;
use std::ffi::CString;
use std::fmt::write;
use std::str::FromStr;
use cidr::Ipv4Cidr;
use libn2n::{ascii_to_bin, edge_conf_add_supernode, edge_init_conf_defaults, generate_private_key, n2n_edge_conf_t, n2n_edge_t, n2n_tuntap_priv_config_t};
use log::warn;
use serde::{Serialize, Deserialize, ser::Serializer};
use crate::{n2n, platform};

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
    EdgeInitFailed,
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
            EdgeError::EdgeInitFailed => write!(f, "EdgeInitFailed"),
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

pub unsafe fn load_config(config: EdgeConfig, mut conf: n2n_edge_conf_t, mut ec: n2n_tuntap_priv_config_t) -> Result<(), EdgeError> {
    edge_init_conf_defaults(&mut conf);
    ec.mtu = DEFAULT_MTU;
    ec.daemon = 1;
    platform::init(&mut ec);

    let supernode_addr = CString::new(config.supernode_addr).unwrap();

    conf.community_name = [0u8; 20];
    conf.community_name[..config.community.len()].copy_from_slice(config.community.as_bytes());

    edge_conf_add_supernode(std::ptr::addr_of_mut!(conf), supernode_addr.as_ptr());

    if !config.encryption_key.is_none() {
        let encryption_key = RefCell::new(CString::new(config.encryption_key.unwrap()).unwrap());
        conf.encrypt_key = encryption_key.as_ptr();
    }

    if !config.address.is_none() {
        let address = config.address.unwrap();
        let mut cidr_str = address.clone();
        if address.starts_with("static:") {
            ec.ip_mode = std::mem::transmute(CString::new("static").unwrap().as_bytes());
            cidr_str = cidr_str.trim_start_matches("static:").to_string();
        } else if address.starts_with("dhcp:") {
            ec.ip_mode = std::mem::transmute(CString::new("dhcp").unwrap().as_bytes());
            cidr_str = cidr_str.trim_start_matches("dhcp:").to_string();
        }

        let cidr = Ipv4Cidr::from_str(&cidr_str).unwrap();

        ec.ip_addr = std::mem::transmute(
            CString::new(cidr.first_address().to_string())
                .unwrap()
                .as_bytes(),
        );
        ec.netmask =
            std::mem::transmute(CString::new(cidr.mask().to_string()).unwrap().as_bytes());
    }

    if !config.mac.is_none() {
        ec.device_mac = std::mem::transmute::<&[u8], &[i8]>(config.mac.unwrap().as_bytes())
            .try_into()
            .unwrap();
    }

    if !config.cipher.is_none() {
        let cipher = config.cipher.unwrap();
        if n2n::EdgeCipher::None == cipher {
            conf.transop_id = 1;
        } else if n2n::EdgeCipher::Twofish == cipher {
            conf.transop_id = 2;
        } else if n2n::EdgeCipher::AES == cipher {
            conf.transop_id = 3;
        } else if n2n::EdgeCipher::ChaCha20 == cipher {
            conf.transop_id = 4;
        } else if n2n::EdgeCipher::Speck == cipher {
            conf.transop_id = 5;
        } else {
            return Err(EdgeError::InvalidCipher);
        }
    }

    if !config.username.is_none() {
        conf.dev_desc = std::mem::transmute(config.username.unwrap().as_bytes());
    }

    if !config.password.is_none() {
        let mut shared_secret = [[0u8; 32]; 1];
        let password = CString::new(config.password.unwrap()).unwrap();
        conf.shared_secret = shared_secret.as_mut_ptr();
        generate_private_key(shared_secret[0].as_mut_ptr(), password.into_raw());
    }

    if !config.public_key.is_none() {
        let mut federation_public_key = [[0u8; 32]; 1];
        let public_key = CString::new(config.public_key.unwrap()).unwrap();
        conf.federation_public_key = federation_public_key.as_mut_ptr();
        ascii_to_bin(federation_public_key[0].as_mut_ptr(), public_key.into_raw());
    }

    if conf.transop_id == 1 && !conf.encrypt_key.is_null() {
        warn!("switching to AES as key was provided");
        conf.transop_id = 3;
    }

    Ok(())
}

pub unsafe fn clear_config(mut conf: n2n_edge_conf_t, mut ec: n2n_tuntap_priv_config_t) {
    
}
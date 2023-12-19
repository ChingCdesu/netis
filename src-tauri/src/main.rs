// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod n2n;
mod platform;

use cidr::{Ipv4Cidr};
use libn2n::{
    edge_conf_add_supernode, edge_init_conf_defaults, macstr_t, n2n_edge_conf_t, n2n_edge_t,
    n2n_tuntap_priv_config_t, peer_info_t, timeval, tuntap_dev, generate_private_key, ascii_to_bin,
};
use n2n::{EdgeConfig, EdgeError, DEFAULT_MTU};
use std::{ffi::CString, mem, ptr, str::FromStr};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn start_edge(config: EdgeConfig) -> Result<String, EdgeError> {
    let rc = 0;
    let tuntap = tuntap_dev::default();

    let eee: *mut n2n_edge_t = ptr::null_mut();
    let runlevel: u8 = 0;
    let seek_answer: u8 = 0;
    let now = 0;
    let latest_action = 0;
    let mac_buf = macstr_t::default();
    let socket_mask = 0;
    let wait_time = timeval::default();
    let scan: *mut peer_info_t = ptr::null_mut();
    let scan_tmp: *mut peer_info_t = ptr::null_mut();

    let expected: u16 = 2;
    let position: u16 = 0;
    let pktbuf: [u8; 2048 + 2] = [0; 2048 + 2];

    if config.community.is_empty() {
        return Err(EdgeError::CommunityIsNull);
    }

    if config.supernode_addr.is_empty() {
        return Err(EdgeError::SupernodeAddressIsNull);
    }

    unsafe {
        let mut conf: n2n_edge_conf_t = mem::zeroed();
        let mut ec: n2n_tuntap_priv_config_t = mem::zeroed();
        edge_init_conf_defaults(&mut conf);
        ec.mtu = DEFAULT_MTU;
        ec.daemon = 1;
        platform::init(&mut ec);

        let supernode_addr = CString::new(config.supernode_addr).unwrap();
        conf.community_name = config.community.as_bytes().try_into().unwrap();
        edge_conf_add_supernode(std::ptr::addr_of_mut!(conf), supernode_addr.as_ptr());

        if !config.encryption_key.is_none() {
            let encryption_key = CString::new(config.encryption_key.unwrap()).unwrap();
            conf.encrypt_key = encryption_key.into_raw();
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
    }

    Ok("Success".into())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, start_edge])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

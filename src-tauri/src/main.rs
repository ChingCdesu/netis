// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod n2n;
mod platform;

use cidr::{Ipv4Cidr};
use libc::setuid;
use libn2n::{
    edge_conf_add_supernode, edge_init_conf_defaults, macstr_t, n2n_edge_conf_t, n2n_edge_t,
    n2n_tuntap_priv_config_t, peer_info_t, timeval, tuntap_dev, generate_private_key, ascii_to_bin, n2n_seed, n2n_srand, edge_init,
};
use n2n::{EdgeConfig, EdgeError, DEFAULT_MTU};
use platform::init_user;
use std::{ffi::CString, mem, ptr, str::FromStr, os};
use log::{debug, warn, error};
use crate::n2n::load_config;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn start_edge(config: EdgeConfig) -> Result<String, EdgeError> {
    let mut rc = 0;
    let tuntap = tuntap_dev::default();

    let mut eee: *mut n2n_edge_t = ptr::null_mut();
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

        let err = load_config(config, conf, ec).err();
        if !err.is_none() {
            return Err(err.unwrap());
        }

        if !conf.shared_secret.is_null() {
            if conf.federation_public_key.is_null() {
                let mut federation_public_key = [[0u8; 32]; 1];
                // conf.federation_public_key =
            }
        }

        n2n_srand(n2n_seed());
        
        init_user();

        eee = edge_init(&conf, &mut rc);
        if eee.is_null() {
            error!("failed in edge_init");
            return Err(EdgeError::EdgeInitFailed);
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

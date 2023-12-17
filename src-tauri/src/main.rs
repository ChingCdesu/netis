// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod platform;

use libn2n::{
    edge_init_conf_defaults, macstr_t, n2n_edge_conf_t, n2n_edge_t, n2n_tuntap_priv_config_t,
    peer_info_t, timeval, tuntap_dev,
};
use std::{mem, ptr};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
unsafe fn n2n() -> String {
    let rc = 0;
    let tuntap = tuntap_dev::default();
    let mut conf = n2n_edge_conf_t::default();
    let mut ec: n2n_tuntap_priv_config_t = mem::zeroed();
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

    platform::init::init_system();

    edge_init_conf_defaults(&mut conf);
    ec.mtu = 1290;
    ec.daemon = 1;

    return "n2n".to_string();
}

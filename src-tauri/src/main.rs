// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use libn2n::{
    edge_init, n2n_edge_conf_t, n2n_edge_t, n2n_mac_t, n2n_tuntap_priv_config_t, tuntap_dev,
};
use std::ptr;

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
fn n2n() -> String {
    let tuntap = tuntap_dev::default();
    let conf = n2n_edge_conf_t::default();
    let ec = n2n_tuntap_priv_config_t::default();
    let eee: *mut n2n_edge_t = ptr::null_mut();

    return "n2n".to_string();
}

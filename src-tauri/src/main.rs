// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{utils::config::AppUrl, WindowUrl};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let port = portpicker::pick_unused_port().expect("failed to find unused port");

    let mut context = tauri::generate_context!();
    let url = format!("http://localhost:{}", port).parse().unwrap();
    let window_url = WindowUrl::External(url);
    // rewrite the config so the IPC is enabled on this URL
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_localhost::Builder::new(port)
                .on_request(|_req, resp| {
                    resp.add_header("Cross-Origin-Opener-Policy", "same-origin");
                    resp.add_header("Cross-Origin-Embedder-Policy", "require-corp");
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet])
        .run(context)
        .expect("error while running tauri application");
}

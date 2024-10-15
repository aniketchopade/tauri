// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::io::{Read, Write};
use std::net::TcpStream;
use serde::{Deserialize, Serialize};
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_cpu_uid() -> String {
    let output = Command::new("wmic")
        .args(&["cpu", "get", "ProcessorId"])
        .output()
        .expect("failed to execute process");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.split('\n').collect();
    if lines.len() > 1 {
        lines[1].trim().to_string()
    } else {
        "Unable to retrieve CPU UID".to_string()
    }
}

#[derive(Serialize, Deserialize)]
struct RequestResponseData {
    key: String,
    value: String,
}

#[tauri::command]
fn invoke_tcp_socket(request_data: RequestResponseData) -> Result<RequestResponseData, String> {
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            let request_json = serde_json::to_string(&request_data).map_err(|e| e.to_string())?;
            stream.write_all(request_json.as_bytes()).map_err(|e| e.to_string())?;

            let mut buffer = [0; 512];
            let n = stream.read(&mut buffer).map_err(|e| e.to_string())?;
            let response_json = String::from_utf8_lossy(&buffer[..n]);

            serde_json::from_str(&response_json).map_err(|e| e.to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn write_to_registry(key_path: &str, value_name: &str, value_data: &str) -> Result<String, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    match hkcu.create_subkey(key_path) {
        Ok((key, _disp)) => {
            match key.set_value(value_name, &value_data) {
                Ok(_) => Ok("Value written to registry successfully".to_string()),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_cpu_uid, invoke_tcp_socket, write_to_registry])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

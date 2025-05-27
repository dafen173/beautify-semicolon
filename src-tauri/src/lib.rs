// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Add these imports at the top
use serde_json::Value;
use serde_xml_rs::to_string;
use serde_yaml; // For YAML conversion // For XML conversion

// add new commands here =====================================
use serialport::{SerialPort, SerialPortBuilder};
use std::time::Duration;
use tauri::command;
// add new commands here =====================================




// add new commands here =====================================
fn open_serial_port() -> Result<Box<dyn SerialPort>, String> {
    serialport::new("/dev/ttyUSB0", 9600) // Adjust port for your OS
        .timeout(Duration::from_millis(1000))
        .open()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn send_projector_command(cmd: &str) -> Result<String, String> {
    let mut port = open_serial_port()?;
    port.write_all(format!("{}\r", cmd).as_bytes()) // Add `\r` terminator
        .map_err(|e| e.to_string())?;

    // Read response (optional)
    let mut buf = [0; 128];
    let bytes_read = port.read(&mut buf).map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&buf[..bytes_read]).to_string())
}
// add new commands here =====================================

#[tauri::command]
fn beautify(data: &str) -> String {
    let parsed_json: Value = serde_json::from_str(data).unwrap();
    serde_json::to_string_pretty(&parsed_json).unwrap()
}

#[tauri::command]
fn minify(data: &str) -> String {
    let parsed_json: Value = serde_json::from_str(data).unwrap();
    serde_json::to_string(&parsed_json).unwrap()
}

#[tauri::command]
fn json_to_yaml(data: &str) -> String {
    let parsed_json: Value = serde_json::from_str(data).unwrap();
    serde_yaml::to_string(&parsed_json).unwrap()
}

#[tauri::command]
fn json_to_xml(data: &str) -> String {
    let parsed_json: Value = serde_json::from_str(data).unwrap();
    to_string(&parsed_json).unwrap()
}

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()
//         .plugin(tauri_plugin_opener::init())
//         .invoke_handler(tauri::generate_handler![beautify, minify, json_to_yaml, json_to_xml])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

// add new commands here =====================================
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![send_projector_command])
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
// add new commands here =====================================

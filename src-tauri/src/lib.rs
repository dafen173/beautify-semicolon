// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Add these imports at the top
use serde_json::Value;
use serde_yaml;  // For YAML conversion
use serde_xml_rs::to_string;  // For XML conversion




#[tauri::command]
fn beautify(data: &str) -> String {
    let parsed_json : Value = serde_json::from_str(data).unwrap();
    serde_json::to_string_pretty(&parsed_json).unwrap() 
}

#[tauri::command]
fn minify(data: &str) -> String {
    let parsed_json : Value = serde_json::from_str(data).unwrap();
    serde_json::to_string(&parsed_json).unwrap() 
}

#[tauri::command]
fn json_to_yaml(data: &str) -> String {
    let parsed_json : Value = serde_json::from_str(data).unwrap();
    serde_yaml::to_string(&parsed_json).unwrap()
}

#[tauri::command]
fn json_to_xml(data: &str) -> String {
    let parsed_json : Value = serde_json::from_str(data).unwrap();
    to_string(&parsed_json).unwrap()
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![beautify, minify, json_to_yaml, json_to_xml])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

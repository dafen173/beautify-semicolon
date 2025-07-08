use serialport::SerialPort;
use std::time::Duration;

fn open_serial_port() -> Result<Box<dyn SerialPort>, String> {
    serialport::new("/dev/ttyUSB0", 9600) // Adjust port for your OS ('COM3' - for Windows; '/dev/ttyUSB0' - for Linux)
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![send_projector_command])
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}

use tauri::command;

#[command]
pub fn get_bridge_url() -> String {
    "http://127.0.0.1:4310".to_string()
}

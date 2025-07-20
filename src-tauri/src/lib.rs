// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod feature;
use feature::{backup_configs, restore_configs, find_lol_root}; // 导入函数
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            backup_configs,
            restore_configs,
            find_lol_root
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


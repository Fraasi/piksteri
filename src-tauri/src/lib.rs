use tauri::Manager; // need for devtools?
use std::fs;
use tauri_plugin_window_state::StateFlags;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_files(path: String) -> Result<Vec<String>, String> {
    match fs::read_dir(path) {
        Ok(files) => Ok(files
            .map(|f| f.unwrap().path().to_str().unwrap().to_string())
            .collect()),
        Err(e) => Err(format!("Failed to read directory: {}", e)),
    }
}


#[tauri::command]
fn get_metadata(path: &str) -> String {
    // how
    format!("rust: {}", path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    fn saved_states() -> StateFlags {
        StateFlags::VISIBLE|StateFlags::POSITION|StateFlags::SIZE
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default()
            .with_state_flags(saved_states())
            .build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![get_files, get_metadata])
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_webview_window("main").unwrap().open_devtools(); // `main` is the first window from tauri.conf.json without an explicit label
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

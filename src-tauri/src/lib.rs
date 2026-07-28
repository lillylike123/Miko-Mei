use serde::{Deserialize, Serialize};
use tauri::command;
use std::fs::{self, OpenOptions};
use std::io::Write;
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct JournalEntry {
    date: String,
    content: String,
    ai_score: Option<f32>,
}

#[command]
fn add_journal_entry(app: tauri::AppHandle, content: String) -> Result<JournalEntry, String> {
    let date = chrono::Local::now().format("%b %d, %Y - %H:%M").to_string();
    
    let entry = JournalEntry {
        date,
        content,
        ai_score: Some(9.5),
    };

    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let file_path = app_dir.join("journal_logs.json");

    let mut entries: Vec<JournalEntry> = if file_path.exists() {
        let data = fs::read_to_string(&file_path).unwrap_or_else(|_| "[]".to_string());
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        Vec::new()
    };

    entries.insert(0, entry.clone());

    let serialized = serde_json::to_string_pretty(&entries).map_err(|e| e.to_string())?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .map_err(|e| e.to_string())?;
    
    file.write_all(serialized.as_bytes()).map_err(|e| e.to_string())?;

    Ok(entry)
}

#[command]
fn get_journal_entries(app: tauri::AppHandle) -> Result<Vec<JournalEntry>, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let file_path = app_dir.join("journal_logs.json");

    if !file_path.exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let entries: Vec<JournalEntry> = serde_json::from_str(&data).unwrap_or_else(|_| vec![]);
    Ok(entries)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![add_journal_entry, get_journal_entries])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
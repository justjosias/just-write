#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs, io::Write, process};

use jw_internals::{config, notebooks, Notebook};

use serde_derive::{Deserialize, Serialize};

mod utils;

#[tauri::command]
fn save_post(notebook: &str, contents: &str) -> bool {
    println!("Sending: {}", &contents);
    let notebook = Notebook::load(notebook).unwrap();
    if let Ok(_) = notebook.post(contents.to_string()) {
        true
    } else {
        false
    }
}

#[tauri::command]
fn notebook_list() -> Vec<String> {
    let notebooks: Vec<String> = notebooks::list_notebooks()
        .unwrap_or_else(|_| Vec::new())
        .iter()
        .map(|n| n.id.clone())
        .collect();
    println!("{:?}", &notebooks);
    notebooks
}

#[tauri::command]
fn open_folder(id: &str) -> bool {
    if let Some(notebook) = Notebook::load(id) {
        process::Command::new(utils::file_manager())
            .arg(notebook.path)
            .spawn()
            .unwrap();
        true
    } else {
        false
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct State {
    text: Option<String>,
    notebook: Option<String>,
}

#[tauri::command]
fn save_state(text: &str, notebook: &str) -> bool {
    let path = config::get_config_path(Some("gui.toml")).unwrap();

    let state = State {
        text: Some(text.to_owned()),
        notebook: Some(notebook.to_owned()),
    };
    let conf_str = toml::to_string(&state).unwrap();

    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    f.write_all(conf_str.as_bytes()).unwrap();

    println!("Saved state!");

    true
}

#[tauri::command]
fn load_state() -> State {
    let path = config::get_config_path(Some("gui.toml")).unwrap();
    let conf_str = fs::read_to_string(path).unwrap_or("".to_string());
    let config = toml::from_str(&conf_str).unwrap();

    config
}

#[tauri::command]
fn print_cli(msg: &str) {
    println!("Message: {}", msg);
}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_post,
            notebook_list,
            open_folder,
            load_state,
            save_state,
            print_cli,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

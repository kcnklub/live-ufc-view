#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::vec;

use app::get_fight_card;
use app::Fights;
use log::info;
use reqwest::blocking::Client;
use tauri::{State, Window};

struct Storage {
    is_fetching_data: Mutex<bool>,
}

fn main() {
    log4rs::init_file("log.yml", Default::default()).expect("Cannot init logger.");

    info!("//////////////////////////////////");
    info!("///// STARTING UFC LIVE VIEW /////");
    info!("//////////////////////////////////");

    tauri::Builder::default()
        .manage(Storage {
            is_fetching_data: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![init_fetching_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn init_fetching_data(
    window: Window,
    storage: State<Storage>,
) {
    let mut is_fetching = storage.is_fetching_data.lock().unwrap();
    if *is_fetching {
        return;
    }
    thread::spawn(move || loop {
        let client: reqwest::blocking::Client = Client::new();
        let current_content = client
            .get("https://espn.com/mma/fightcenter")
            .send()
            .unwrap()
            .text()
            .unwrap();
        let data = match get_fight_card(&current_content) {
            Ok(result) => result,
            Err(_error) => Fights::default(),
        };
        window.emit("update_data", data).unwrap();
        thread::sleep(Duration::new(5, 0));
    });
    info!("Data fetching Thread started!");
    *is_fetching = true;
}


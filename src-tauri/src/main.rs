#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::error::Error;
use std::vec;

use log::info;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

fn main()
{
    log4rs::init_file("log.yml", Default::default()).expect("Cannot init logger.");

    info!("//////////////////////////////////");
    info!("///// STARTING UFC LIVE VIEW /////");
    info!("//////////////////////////////////");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn fetch_data() -> Result<Fights, String>
{
    match load_data()
    {
        Ok(result) => Ok(result),
        Err(_error) => Err("Error while getting data".to_string()),
    }
}

#[derive(Serialize, Deserialize)]
struct Fights
{
    name: String,
    fights: Vec<Fight>,
}

#[derive(Serialize, Deserialize)]
struct Fight
{
    id: String,
    left_fighter: Fighter,
    right_fighter: Fighter,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct Fighter
{
    name: String,
    record: String,
    headshot_source: String,
}

fn load_data() -> Result<Fights, Box<dyn Error>>
{
    let client: reqwest::blocking::Client = Client::new();
    let current_content = client
        .get("https:/espn.com/mma/fightcenter")
        .send()?
        .text()?;

    parse_document(current_content)
}

const FIGHTER_SELECTOR_STRING: &str =
    r#"div[class="MMACompetitor relative flex flex-uniform pr3"]"#;

const NAME_SELECTOR: &str = r#"span[class="truncate tc db"]"#;

const RECORD_SELECTOR_LEFT: &str =
    r#"div[class="flex items-center n9 nowrap justify-end clr-gray-04"]"#;

const RECORD_SELECTOR_RIGHT: &str = r#"div[class="flex items-center n9 nowrap clr-gray-04"]"#;

fn parse_document(content: String) -> Result<Fights, Box<dyn Error>>
{
    let document = Html::parse_document(&content);
    let fighter_selector = Selector::parse(FIGHTER_SELECTOR_STRING)?;

    let mut fight_list = vec![];
    let mut counter = 0;
    let mut left_fighter = Fighter::default();
    let mut right_fighter = Fighter::default();
    
    for element in document.select(&fighter_selector)
    {
        info!("Fighter found");
        let is_left = counter % 2 == 0;
        let name_selector = Selector::parse(NAME_SELECTOR)?;
        for name in element.select(&name_selector)
        {
            match is_left
            {
                true => left_fighter.name = name.inner_html(),
                false => right_fighter.name = name.inner_html(),
            }
        }

        let record_selector = Selector::parse(RECORD_SELECTOR_LEFT)?;
        for record in element.select(&record_selector)
        {
            left_fighter.record = record.inner_html().trim().to_string();
        }

        let record_selector_right = Selector::parse(RECORD_SELECTOR_RIGHT)?;
        for record in element.select(&record_selector_right)
        {
            right_fighter.record = record.inner_html().trim().to_string();
        }

        if counter % 2 != 0
        {
            fight_list.push(Fight {
                id: counter.to_string(),
                left_fighter: left_fighter.clone(),
                right_fighter: right_fighter.clone(),
            })
        }
        counter += 1;
    }

    Ok(Fights {
        name: "UFC Fight Night: Krylov vs. Spann".to_string(),
        fights: fight_list,
    })
}

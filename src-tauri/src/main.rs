#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::error::Error;
use std::vec;

use easy_scraper::Pattern;
use log::info;
use reqwest::blocking::Client;
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

#[derive(Serialize, Deserialize, Default)]
struct Fights
{
    name: String,
    fights: Vec<Fight>,
}

#[derive(Serialize, Deserialize, Default)]
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

    parse_html_easy_scraper(&current_content)
}

const CARD_TITLE: &str = r#"<h1 class="headline headline__h1 mb3">{{title}}</h1>"#;

const FIGHTER_ROW_PATTERN_STRING: &str = r#"
<div class="MMAGamestrip flex items-center justify-center">
    <div class="MMACompetitor relative flex flex-uniform pr6 flex-row-reverse MMACompetitor--desktop"> 
        <div class="flex w-100 flex-row-reverse">
            <div class="MMACompetitor__Detail flex flex-column justify-center">
                <h2 class="h4 clr-gray-02">
                    <span class="truncate tc db">{{fighter_name_left}}</span>
                </h2>
                <div class="flex items-center n9 nowrap justify-end clr-gray-04">{{fighter_record_left}}</div>
            </div>
        </div>
    </div>
    <div></div>
    <div class="MMACompetitor relative flex flex-uniform pl6 MMACompetitor--desktop">
        <div class="flex w-100">
            <div class="MMACompetitor__Detail flex flex-column justify-center">
                <h2 class="h4 clr-gray-02">
                    <span class="truncate tc db">{{fighter_name_right}}</span>
                </h2>
                <div class="flex items-center n9 nowrap clr-gray-04">{{fighter_record_right}}</div>
            </div>
        </div>
    </div>
</div>
"#;

const FIGHTER_NAME_RIGHT: &str = "fighter_name_right";
const FIGHTER_RECORD_RIGHT: &str = "fighter_record_right";
const FIGHTER_NAME_LEFT: &str = "fighter_name_left";
const FIGHTER_RECORD_LEFT: &str = "fighter_record_left";

fn parse_html_easy_scraper(content: &String) -> Result<Fights, Box<dyn Error>>
{
    info!("parsing with easy scraper");
    let fighter_pattern = Pattern::new(FIGHTER_ROW_PATTERN_STRING)?;

    info!("created pattern");

    let fighter_rows = fighter_pattern.matches(content);
    info!("{:?}", fighter_rows);

    let mut counter = 0;
    let mut fights = vec![];
    for row in fighter_rows
    {
        info!("{:?}", row);

        let fighter_left = Fighter {
            name: row[FIGHTER_NAME_RIGHT].to_string(),
            record: row[FIGHTER_RECORD_RIGHT].to_string(),
            headshot_source: "".to_string(),
        };

        let fighter_right = Fighter {
            name: row[FIGHTER_NAME_LEFT].to_string(),
            record: row[FIGHTER_RECORD_LEFT].to_string(),
            headshot_source: "".to_string(),
        };

        fights.push(Fight {
            id: counter.to_string(),
            left_fighter: fighter_left,
            right_fighter: fighter_right,
        });
        counter += 1;
    }

    let title_pattern = Pattern::new(CARD_TITLE)?;
    let title = title_pattern.matches(content)[0]["title"].to_string();

    Ok(Fights {
        name: title, 
        fights: fights
    })
}

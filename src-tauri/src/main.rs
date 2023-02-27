#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use std::time::Duration;
use std::vec;
use std::{error::Error, thread};

use easy_scraper::Pattern;
use log::{info, warn};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use tauri::{State, Window};

struct Storage
{
    is_fetching_data: Mutex<bool>,
}

fn main()
{
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
)
{
    let mut is_fetching = storage.is_fetching_data.lock().unwrap();
    if *is_fetching
    {
        return;
    }
    thread::spawn(move || {
        loop
        {
            let data = match load_data()
            {
                Ok(result) => result,
                Err(_error) => Fights::default(),
            };
            thread::sleep(Duration::new(5, 0));

            window.emit("update_data", data).unwrap();
        }
    });
    info!("Data fetching Thread started!");
    *is_fetching = true;
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
struct Fights
{
    name: String,
    current_fight: Fight,
    fights: Vec<Fight>,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq)]
struct Fight
{
    id: String,
    left_fighter: Fighter,
    right_fighter: Fighter,
    odds: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
struct Fighter
{
    name: String,
    record: String,
}

fn load_data() -> Result<Fights, Box<dyn Error>>
{
    let start_time = chrono::Local::now().timestamp_millis();
    let client: reqwest::blocking::Client = Client::new();
    let current_content = client
        .get("https://www.espn.com/mma/fightcenter/_/id/600027449/league/ufc")
        .send()?
        .text()?;

    match parse_html_easy_scraper(&current_content)
    {
        Ok(result) =>
        {
            let total_time = chrono::Local::now().timestamp_millis() - start_time;
            info!("Total data fetching and parsing in {total_time}ms");
            Ok(result)
        }
        Err(error) =>
        {
            warn!("Error parsing content from ESPN: {error}");
            Err(error)
        }
    }
}

const CARD_TITLE: &str = r#"<h1 class="headline headline__h1 mb3">{{title}}</h1>"#;

const CURRENT_FIGHT_LEFT: &str = r#"
<div class="MMAFightCard__Gamestrip br-5 mh4 relative MMAFightCard__Gamestrip--open">
    <div class="MMACompetitor__Detail flex flex-column justify-center">
        <h2 class="h4 clr-gray-02">
            <span class="truncate tc db">{{fighter_name_left}}</span>
        </h2>
        <div class="flex items-center n9 nowrap justify-end clr-gray-04">{{fighter_record_left}}</div>
    </div>
    <div class="Gamestrip__Overview relative items-center clr-gray-04 flex justify-center flex-column n8 MMAGamestrip__Overview">
        <div class="ScoreCell__Time Gamestrip__Time h9 clr-negative">{{game_time}}</div>
    </div>
    <div class="flex w-100">
        <div class="MMACompetitor__Detail flex flex-column justify-center">
            <h2 class="h4 clr-gray-02">
                <span class="truncate tc db">{{fighter_name_right}}</span>
            </h2>
            <div class="flex items-center n9 nowrap clr-gray-04">{{fighter_record_right}}</div>
        </div>
    </div>
</div>
"#;

const CURRENT_FIGHT_MIDDLE: &str = r#"<div class="ScoreCell__Time Gamestrip__Time h9 clr-negative">{{round_and_time}}</div>"#;

const CURRENT_FIGHT_RIGHT: &str = r#"
<div class="MMAFightCard__Gamestrip br-5 mh4 relative MMAFightCard__Gamestrip--open">
{{info}}
</div>
"#;

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
    <div class="Gamestrip__Overview relative items-center clr-gray-04 flex justify-center flex-column n8 MMAGamestrip__Overview">
        <div class="ScoreCell__Odds Gamestrip__Odds clr-gray-03 n9">{{odds}}</div>
    </div>
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
    let fighter_pattern = Pattern::new(FIGHTER_ROW_PATTERN_STRING)?;
    let fighter_rows = fighter_pattern.matches(content);

    let mut counter = 0;
    let mut fights = vec![];
    for row in fighter_rows
    {
        println!("{:?}", row);
        let fighter_left = Fighter {
            name: row[FIGHTER_NAME_RIGHT].to_string(),
            record: row[FIGHTER_RECORD_RIGHT].to_string(),
        };

        let fighter_right = Fighter {
            name: row[FIGHTER_NAME_LEFT].to_string(),
            record: row[FIGHTER_RECORD_LEFT].to_string(),
        };

        fights.push(Fight {
            id: counter.to_string(),
            left_fighter: fighter_left,
            right_fighter: fighter_right,
            odds: row["odds"].to_string(),
        });
        counter += 1;
    }

    let title_pattern = Pattern::new(CARD_TITLE)?;
    let title = title_pattern.matches(content)[0]["title"].to_string();
    println!("{title}");

    // let current_title_pattern = Pattern::new(CURRENT_FIGHT_LEFT)?;
    // let current_fight = current_title_pattern.matches(content);
    // println!("{:?}", current_fight);

    let current_title_pattern = Pattern::new(CURRENT_FIGHT_MIDDLE)?;
    let current_fight = current_title_pattern.matches(content);
    println!("{:?}", current_fight);

    // let current_title_pattern = Pattern::new(CURRENT_FIGHT_RIGHT)?;
    // let current_fight = current_title_pattern.matches(content);
    // println!("{:?}", current_fight);

    Ok(Fights {
        name: title,
        current_fight: Fight::default(),
        fights: fights,
    })
}

#[cfg(test)]
mod test
{
    use crate::{load_data, get_main_fight, Fight, Fighter};

    #[test]
    fn test()
    {
        let output = load_data().unwrap();

        // println!("{:?}", output);
    }

    #[test]
    fn pre_fight_current_fight_header_parsing() 
    {
        let pre_fight_current_fight = std::fs::read_to_string("./test_resources/pre-fight-current-fight.html").unwrap();
        
        let main_fight = get_main_fight(&pre_fight_current_fight);

        let expected_fight = Fight {
            id: "".to_string(),
            left_fighter: Fighter {
                name: "".to_string(),
                record: "".to_string(),
            }, 
            right_fighter: Fighter {
                name: "".to_string(),
                record: "".to_string(),
            }, 
            odds: "".to_string(),
        };

        assert_eq!(expected_fight, main_fight.unwrap());
    }
}

fn get_main_fight(content: &String) -> Result<Fight, Box<dyn Error>>
{
    let pattern_str = r#"
<div class="MMAFightCard__Gamestrip br-5 mh4 relative MMAFightCard__Gamestrip--open">
    <span>{{name}}</span>
    <div class="flex items-center n9 nowrap clr-gray-04">{{record}}</div> 
</div>
"#;
    let pat = Pattern::new(pattern_str)?;
    let main_fight = pat.matches(content);

    println!("{:?}", main_fight);
    Ok(Fight::default())
}

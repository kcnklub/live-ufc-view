use serde::{Serialize, Deserialize}; 
use std::error::Error;
use easy_scraper::Pattern;

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Fights {
    name: String,
    current_fight: Fight,
    fights: Vec<Fight>,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Fight {
    id: String,
    left_fighter: Fighter,
    right_fighter: Fighter,
    odds: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Fighter {
    name: String,
    record: String,
    stats: FighterStats
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct FighterStats {
    height: String, 
    weight: String, 
    reach: String, 
    stance: String, 
    sig_str_lpm: String, 
    sig_str_acc: String, 
    td_avg: String, 
    td_acc: String, 
    sub_avg: String
}

#[cfg(test)]
mod get_fight_card_test {
    use crate::get_fight_card; 

    #[test]
    fn getting_raw_row_data() {
        let content = reqwest::blocking::get("https://espn.com/mma/fightcenter")
            .unwrap()
            .text()
            .unwrap();

        let card = get_fight_card(&content).unwrap();
        println!("{:?}", card);
    }
}


pub fn get_fight_card(content: &String) -> Result<Fights, Box<dyn Error>>
{

    let card_name = get_card_name(&content)?;
    let current_fight = get_current_fight(&content)?;
    let fights = get_all_fights(&content)?;

    let fights = Fights {
        name: card_name,
        current_fight, 
        fights, 
    };

    Ok(fights)
}

fn get_card_name(content: &String) -> Result<String, Box<dyn Error>>
{
    let card_name_pat_string = r#"<h1 class="headline__h1 mb3">{{content}}</h1>"#;
    let pat = Pattern::new(card_name_pat_string)?;
    let matches = pat.matches(&content);

    Ok(matches[0]["content"].to_string())
}

fn get_current_fight(content: &String) -> Result<Fight, Box<dyn Error>>
{
    let current_fight_string = r#"<div class="MMAFightCard__Gamestrip br-5 mh4 relative MMAFightCard__Gamestrip--open">{{content:*}}</div>"#;
    let pat = Pattern::new(current_fight_string)?; 
    let current_fight_data = pat.matches(&content); 
   

    Ok(get_fight_details(current_fight_data[0]["content"].to_string(), "main_card".to_string())?)
}

fn get_all_fights(content: &String) -> Result<Vec<Fight>, Box<dyn Error>>
{
    let all_fight_string = r#"<div class="MMAFightCard__Gamestrip br-5 mh4 relative">{{content:*}}</div>"#;
    let pat = Pattern::new(all_fight_string)?; 
    let raw_fight_data = pat.matches(&content);
    
    let mut counter = 1;
    let mut all_fights: Vec<Fight> = vec![];
    for fight in raw_fight_data {
        let details = get_fight_details(fight["content"].to_string(), counter.to_string())?;
        all_fights.push(details);
        counter += 1;
    }
    Ok(all_fights)
}

fn get_fight_details(content: String, id: String) -> Result<Fight, Box<dyn Error>>
{
    let fighter_pattern_string = r#"<div>{{content}}</div>"#; 
    let pat = Pattern::new(fighter_pattern_string).unwrap();
    let fighter_info = pat.matches(&content);


    let fight = Fight {
        id,
        left_fighter: Fighter {
            name: fighter_info[0]["content"].to_string(),
            record: fighter_info[1]["content"].to_string(),
            stats: FighterStats::default(),
        },
        right_fighter: Fighter {
            name: fighter_info[5]["content"].to_string(),
            record: fighter_info[6]["content"].to_string(),
            stats: FighterStats::default(),
        },
        odds: fighter_info[4]["content"].to_string(),
    };

    Ok(fight)
}

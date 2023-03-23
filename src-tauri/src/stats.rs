use easy_scraper::Pattern;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct FighterStats {
    kd: String,
    total_strikes: String,
    sig_strikes: String,
    head: String,
    body: String,
    legs: String,
    control: String,
    take_downs: String,
    sub_att: String,
    height: String,
    weight: String,
    reach: String,
    stance: String,
    sig_str_acc: String,
    sig_str_lpm: String,
    td_avg: String,
    td_acc: String,
    sub_avg: String,
}

pub fn get_stats(content: &String) -> Result<(FighterStats, FighterStats), Box<dyn Error>> {
    let (raw_left, raw_right) = parse_fighter_stats(content)?;
    let left = reparse_stats(raw_left);
    let right = reparse_stats(raw_right);
    Ok((left, right))
}

fn parse_fighter_stats(
    content: &String
) -> Result<(HashMap<String, String>, HashMap<String, String>), Box<dyn Error>> {
    let top_stats_string = r#"<ul class="MMAMatchup list">{{content:*}}</ul>"#;
    let pat = Pattern::new(top_stats_string)?;
    let matches = pat.matches(&content);
    let match_data = matches[0]["content"].to_string();

    let all_data = r#"<li>
        <div class="MMAMatchup__Basis">
            <div>
                <div class="MMAMatchup__Stat ns8 MMAMatchup__Stat__Text">
                    {{left_stat_normal}}
                </div>
            </div>
        </div>
        <div class="ns9 fw-medium ttu nowrap clr-gray-04">{{stat}}</div>
        <div class="MMAMatchup__Basis tar">
            <div>
                <div class="MMAMatchup__Stat ns8 MMAMatchup__Stat__Text">
                    {{right_stat_normal}}
                </div>
            </div>
        </div>
    <li>"#;

    let pat = Pattern::new(all_data)?;
    let normal_data = pat.matches(&match_data);

    let all_data = r#"<li>
        <div class="MMAMatchup__Basis">
            <div>
                <div class="MMAMatchup__Stat ns8 MMAMatchup__Stat__Text">
                    {{left_stat_normal}}
                    <span>{{left_stat_fraction}}</span>
                </div>
            </div>
        </div>
        <div class="ns9 fw-medium ttu nowrap clr-gray-04">{{stat}}</div>
        <div class="MMAMatchup__Basis tar">
            <div>
                <div class="MMAMatchup__Stat ns8 MMAMatchup__Stat__Text">
                    {{right_stat_normal}}
                    <span>{{right_stat_fraction}}</span>
                </div>
            </div>
        </div>
    <li>"#;

    let pat = Pattern::new(all_data)?;
    let fraction_data = pat.matches(&match_data);

    let mut right_fighter_stats = HashMap::default();
    let mut left_fighter_stats = HashMap::default();
    for stat in normal_data {
        let stat_name = stat["stat"].to_string();
        let right_stat = stat["right_stat_normal"].to_string();
        let left_stat = stat["left_stat_normal"].to_string();

        right_fighter_stats.insert(stat_name.clone(), right_stat);
        left_fighter_stats.insert(stat_name.clone(), left_stat);
    }

    for stat in fraction_data {
        let stat_name = stat["stat"].to_string();

        let mut right_stat = stat["right_stat_normal"].to_string();
        right_stat.push_str(&stat["right_stat_fraction"].to_string());

        let mut left_stat = stat["left_stat_normal"].to_string();
        left_stat.push_str(&stat["left_stat_fraction"].to_string());

        right_fighter_stats.insert(stat_name.clone(), right_stat);
        left_fighter_stats.insert(stat_name.clone(), left_stat);
    }
    Ok((left_fighter_stats, right_fighter_stats))
}

fn reparse_stats(map: HashMap<String, String>) -> FighterStats {
    let mut final_stats = FighterStats::default();

    map.iter().for_each(|(key, value)| {
        let key_upper = key.to_uppercase();
        match key_upper.as_str() {
            "KD" => final_stats.kd = value.to_string(),
            "TOT STRIKES" => final_stats.total_strikes = value.to_string(),
            "SIG STRIKES" => final_stats.sig_strikes = value.to_string(),
            "HEAD" => final_stats.head = value.to_string(),
            "BODY" => final_stats.body = value.to_string(),
            "LEGS" => final_stats.legs = value.to_string(),
            "CONTROL" => final_stats.control = value.to_string(),
            "TAKE DOWNS" => final_stats.take_downs = value.to_string(),
            "SUB ATT" => final_stats.sub_att = value.to_string(),
            "HEIGHT" => final_stats.height = value.to_string(),
            "WEIGHT" => final_stats.weight = value.to_string(),
            "REACH" => final_stats.reach = value.to_string(),
            "STANCE" => final_stats.stance = value.to_string(),
            "SIG STR LPM" => final_stats.sig_str_lpm = value.to_string(),
            "SIG STR ACC" => final_stats.sig_str_acc = value.to_string(),
            "TD AVG" => final_stats.td_avg = value.to_string(),
            "TD ACC" => final_stats.td_acc = value.to_string(),
            "SUB AVG" => final_stats.sub_avg = value.to_string(),
            _ => println!(""),
        };
    });

    final_stats
}

#[cfg(test)]
mod fighter_stat_parsing {
    use crate::stats::{parse_fighter_stats, reparse_stats, FighterStats};
    use std::collections::HashMap;

    #[test]
    fn stat_parsing() {
        let content = reqwest::blocking::get("https://espn.com/mma/fightcenter")
            .unwrap()
            .text()
            .unwrap();

        parse_fighter_stats(&content).unwrap();
    }

    #[test]
    fn raw_stats_to_stats() {
        let mut map: HashMap<String, String> = HashMap::default();
        map.insert("KD".to_string(), "0".to_string());
        map.insert("TOT STRIKES".to_string(), "0/1".to_string());
        map.insert("SIG STRIKES".to_string(), "0/1".to_string());
        map.insert("HEAD".to_string(), "0/1".to_string());
        map.insert("BODY".to_string(), "0/1".to_string());
        map.insert("LEGS".to_string(), "0/1".to_string());
        map.insert("CONTROL".to_string(), "1:00".to_string());
        map.insert("TAKE DOWNS".to_string(), "0/1".to_string());
        map.insert("SUB ATT".to_string(), "0".to_string());

        let output = reparse_stats(map);

        let mut expected_value = FighterStats::default();
        expected_value.kd = "0".to_string();
        expected_value.total_strikes = "0/1".to_string();
        expected_value.sig_strikes = "0/1".to_string();
        expected_value.head = "0/1".to_string();
        expected_value.body = "0/1".to_string();
        expected_value.legs = "0/1".to_string();
        expected_value.control = "1:00".to_string();
        expected_value.take_downs = "0/1".to_string();
        expected_value.sub_att = "0".to_string();
        assert_eq!(expected_value, output);
    }
}

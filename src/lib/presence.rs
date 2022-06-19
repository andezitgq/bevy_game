use std::io::Write;
use std::fs;
use std::fs::File;
use toml;

use bevy::prelude::*;
use bevy_discord_presence::state::ActivityState;

use crate::lib::components::*;

pub fn ds_menu(
	mut state: ResMut<ActivityState>,
){
	state.instance = Some(true);
    state.details = Some(format!("Poentaro: {}", score_get()));
    state.state = Some(format!("En menuo"));
	state.assets = Some(discord_presence::models::rich_presence::ActivityAssets {
		large_image: Some("kamplud_".to_string()),
		large_text: Some("Kamplud': esperanta ludo!".to_string()),
		..default()
	});
}

pub fn ds_level(
	mut state: ResMut<ActivityState>,
	level: Option<Res<CurrentLevel>>
){
	state.instance = Some(true);
    state.details = Some(format!("Poentaro: {}", score_get()));
    if let Some(level) = level {
		state.state = Some(format!("Ludante `{}`", &level.0[7..]));
	}
	state.assets = Some(discord_presence::models::rich_presence::ActivityAssets {
		large_image: Some("kamplud_".to_string()),
		large_text: Some("Kamplud': esperanta ludo!".to_string()),
		..default()
	});
}

pub fn score_get() -> usize{
	let contents = match fs::read_to_string("assets/player.toml") {
        Ok(c) => c,
        Err(_) => {
            let mut file = File::create("assets/player.toml").expect("Maleble krei la dosieron!");
            let null = "[playerinfo]\nscore = 0\n";
            if let Err(e) = file.write_all(format!("{}", null).as_bytes()) {
				error!("{}", e);
				return 0;
			}
			
            null.to_string()
        }
    };
        
    let player_toml: PlayerTOML = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            error!("Maleble ŝarĝi datumon de ludanto!");
            return 0;
        }
    };
    
    return player_toml.playerinfo.score;
}

pub fn score_add(points: usize){
	let score = format!("[playerinfo]\nscore = {}\n", score_get() + points);
	let mut file = File::create("assets/player.toml").expect("Maleble krei la dosieron!");
    if let Err(e) = file.write_all(score.as_bytes()) {
		error!("{}", e);
	}
}

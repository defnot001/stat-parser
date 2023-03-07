use serde::{Deserialize, Serialize};
use serde_json::Result as SerdeResult;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct PlayerProfile {
    name: String,
    uuid: Uuid,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Stats {
    #[serde(rename = "minecraft:mined")]
    #[serde(default)]
    pub mined: HashMap<String, i32>,

    #[serde(rename = "minecraft:crafted")]
    #[serde(default)]
    pub crafted: HashMap<String, i32>,

    #[serde(rename = "minecraft:used")]
    #[serde(default)]
    pub used: HashMap<String, i32>,

    #[serde(rename = "minecraft:broken")]
    #[serde(default)]
    pub broken: HashMap<String, i32>,

    #[serde(rename = "minecraft:dropped")]
    #[serde(default)]
    pub dropped: HashMap<String, i32>,

    #[serde(rename = "minecraft:picked_up")]
    #[serde(default)]
    pub picked_up: HashMap<String, i32>,

    #[serde(rename = "minecraft:killed")]
    #[serde(default)]
    pub killed: HashMap<String, i32>,

    #[serde(rename = "minecraft:killed_by")]
    #[serde(default)]
    pub killed_by: HashMap<String, i32>,

    #[serde(rename = "minecraft:custom")]
    #[serde(default)]
    pub custom: HashMap<String, i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StatFile {
    stats: Stats,
}

pub fn get_whitelist(path: &Path) -> SerdeResult<Vec<PlayerProfile>> {
    let content = read_to_string(&path).expect("Failed to read whitelist");
    let data: Vec<PlayerProfile> = serde_json::from_str(&content)?;

    Ok(data)
}

pub fn get_stat_file_contents(paths: Vec<PathBuf>) -> Vec<String> {
    let mut file_strings = Vec::new();

    for path in paths.iter() {
        if let Ok(content) = read_to_string(path) {
            file_strings.push(content)
        } else {
            println!("Could not read file: {}", path.to_string_lossy())
        }
    }

    file_strings
}

pub fn parse_stats(filecontent: &str) -> SerdeResult<Stats> {
    let stat_file: StatFile = serde_json::from_str(filecontent)?;
    Ok(stat_file.stats)
}

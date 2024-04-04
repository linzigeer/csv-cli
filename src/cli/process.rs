use std::fs;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit_number: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut file_vec: Vec<Player> = Vec::with_capacity(128);

    for record in reader.deserialize() {
        let player: Player = record?;
        file_vec.push(player);
    }

    println!("{:?}", file_vec);

    let json = serde_json::to_string_pretty(&file_vec)?;
    fs::write(output, json)?;

    Ok(())
}

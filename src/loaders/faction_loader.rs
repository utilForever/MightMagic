use crate::models::faction::Faction;

use serde_json::Value;

use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, Display};

pub struct FactionLoader
{
    // Has nothing
}

impl FactionLoader {
    pub fn load(factions: &mut Vec<Faction>, filename: &str, faction_name:&str) {
        // Create a path to the desired file
        let path: &Path = Path::new(&filename);
        let display: Display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file: File = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
    
        // Read the file contents into a string, returns `io::Result<usize>`
        let mut str = String::new();
        match file.read_to_string(&mut str) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => (),
        }

        // Deserialize an value from a string of JSON text
        let json_str: Value = match serde_json::from_str(&str) {
            Err(why) => panic!("couldn't parse {}: {}", filename, why),
            Ok(s) => s,
        };

        // Parse creatures data from an value
        let creatures: &Vec<Value> = json_str[faction_name]["town"]["creatures"].as_array().unwrap();
        for creature in creatures {
            let mut faction: Faction = Faction::new();

            let pair: &Vec<Value> = creature.as_array().unwrap();
            faction.creatures.0 = String::from(pair[0].as_str().unwrap());
            faction.creatures.1 = String::from(pair[1].as_str().unwrap());

            factions.push(faction);
        }
    }
}
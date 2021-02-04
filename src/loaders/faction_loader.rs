use crate::models::faction::Faction;

use serde_json::Value;

use std::fs::File;
use std::io::prelude::*;
use std::path::{Display, Path, PathBuf};

pub struct FactionLoader {
    // Has nothing
}

impl FactionLoader {
    pub fn load(factions: &mut Vec<Faction>) {
        let mut res_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        res_path.push("../res/factions/");
        res_path.push("castle.json");
        FactionLoader::load_internal(factions, res_path.as_path(), "castle");
        res_path.pop();

        println!("{}", res_path.display());
    }

    fn load_internal(factions: &mut Vec<Faction>, filename: &Path, faction_name: &str) {
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
            Err(why) => panic!("couldn't parse {}: {}", filename.display(), why),
            Ok(s) => s,
        };

        // Parse creatures data from an value
        let creatures: &Vec<Value> = json_str[faction_name]["town"]["creatures"]
            .as_array()
            .unwrap();
        for creature in creatures {
            let mut faction: Faction = Faction::new();

            let pair: &Vec<Value> = creature.as_array().unwrap();
            faction.creatures.0 = String::from(pair[0].as_str().unwrap());
            faction.creatures.1 = String::from(pair[1].as_str().unwrap());

            factions.push(faction);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let mut factions: Vec<Faction> = Vec::new();
        FactionLoader::load(&mut factions);

        assert_eq!(factions.len(), 7);
    }
}

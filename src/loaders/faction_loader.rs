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

        // Castle
        res_path.push("castle.json");
        FactionLoader::load_internal(factions, res_path.as_path(), "castle");
        res_path.pop();

        // Conflux
        res_path.push("conflux.json");
        FactionLoader::load_internal(factions, res_path.as_path(), "conflux");
        res_path.pop();

        // Dungeon
        res_path.push("dungeon.json");
        FactionLoader::load_internal(factions, res_path.as_path(), "dungeon");
        res_path.pop();

        // Fortress
        res_path.push("fortress.json");
        FactionLoader::load_internal(factions, res_path.as_path(), "fortress");
        res_path.pop();

        // Inferno
        res_path.push("inferno.json");
        FactionLoader::load_internal(factions, res_path.as_path(), "inferno");
        res_path.pop();

        // Necropolis
        res_path.push("necropolis.json");
        FactionLoader::load_internal(factions, res_path.as_path(), "necropolis");
        res_path.pop();

        // Rampart
        res_path.push("rampart.json");
        FactionLoader::load_internal(factions, res_path.as_path(), "rampart");
        res_path.pop();

        // Stronghold
        res_path.push("stronghold.json");
        FactionLoader::load_internal(factions, res_path.as_path(), "stronghold");
        res_path.pop();

        // Tower
        res_path.push("tower.json");
        FactionLoader::load_internal(factions, res_path.as_path(), "tower");
        res_path.pop();
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

        let mut faction: Faction = Faction::new(String::from(faction_name));

        // Parse creatures data from an value
        let creatures: &Vec<Value> = json_str[faction_name]["town"]["creatures"]
            .as_array()
            .unwrap();
        for creature in creatures {
            let pair: &Vec<Value> = creature.as_array().unwrap();
            faction.creatures.push((
                String::from(pair[0].as_str().unwrap()),
                String::from(pair[1].as_str().unwrap()),
            ));
        }

        factions.push(faction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let mut factions: Vec<Faction> = Vec::new();
        FactionLoader::load(&mut factions);

        assert_eq!(factions.len(), 9);
    }
}

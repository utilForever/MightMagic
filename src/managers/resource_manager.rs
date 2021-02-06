use crate::loaders::faction_loader::FactionLoader;
use crate::models::faction::Faction;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref FACTIONS: Mutex<Vec<Faction>> = Mutex::new(vec![]);
}

pub struct ResourceManager {
    // Do nothing
}

impl ResourceManager {
    pub fn initialize() {
        FactionLoader::load(&mut FACTIONS.lock().unwrap());
    }

    pub fn get_all_factions() -> Vec<Faction>
    {
        return FACTIONS.lock().unwrap().to_vec();
    }
}
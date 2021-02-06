use crate::models::faction::Faction;
use crate::models::resource::Resource;

pub struct Player {
    pub factions: Vec<Faction>,
    pub resources: Resource,
}

impl Player {
    #[inline]
    pub const fn new() -> Player {
        Player {
            factions: Vec::new(),
            resources: Resource::new(),
        }
    }
}

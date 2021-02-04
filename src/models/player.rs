use crate::models::faction::Faction;

pub struct Player {
    pub factions: Vec<Faction>
}

impl Player {
    #[inline]
    pub const fn new() -> Player {
        Player { factions: Vec::new() }
    }
}
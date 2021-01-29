pub struct Faction {
    pub creatures: (String, String)
}

impl Faction {
    #[inline]
    pub const fn new() -> Faction {
        Faction { creatures: (String::new(), String::new()) }
    }
}
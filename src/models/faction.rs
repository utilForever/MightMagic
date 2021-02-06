#[derive(Clone)]
pub struct Faction {
    pub name: String,
    pub creatures: Vec<(String, String)>,
}

impl Faction {
    #[inline]
    pub const fn new(_name: String) -> Faction {
        Faction {
            name: _name,
            creatures: Vec::new(),
        }
    }
}

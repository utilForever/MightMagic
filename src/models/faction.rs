#[derive(Clone)]
pub struct Faction {
    pub name: String,
    pub creatures: Vec<(String, String)>,
}

impl Faction {
    #[inline]
    pub const fn new(name: String) -> Faction {
        Faction {
            name: name,
            creatures: Vec::new(),
        }
    }
}

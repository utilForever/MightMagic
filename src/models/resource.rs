pub struct Resource {
    pub gold: i64,
    pub wood: i64,
    pub ore: i64,
    pub mercury: i64,
    pub sulfur: i64,
    pub crystal: i64,
    pub gems: i64,
}

impl Resource {
    #[inline]
    pub const fn new() -> Resource {
        Resource {
            gold: 0,
            wood: 0,
            ore: 0,
            mercury: 0,
            sulfur: 0,
            crystal: 0,
            gems: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    id: u64,
}

impl Entity {
    pub fn new() -> Entity {
        Entity { id: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub address: String,
    pub status: String, // "active", "down"
}

impl Node {
    pub fn new(id: String, address: String) -> Self {
        Self {
            id,
            address,
            status: "active".to_string(),
        }
    }
}

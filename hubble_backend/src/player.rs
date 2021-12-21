use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub name: String,
    pub rating: u32
}

impl Player {
    pub fn empty() -> Self {
        Self {
            name: "".to_string(),
            rating: 0
        }
    }
}

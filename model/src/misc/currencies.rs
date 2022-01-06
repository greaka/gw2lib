use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    pub id:          u64,
    pub name:        String,
    pub description: String,
    pub icon:        String,
    pub order:       u8,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Link {
    pub original_url: String,
    pub link_id: String,
}

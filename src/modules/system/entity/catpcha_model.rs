use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CatpchaRequest {
    pub account: Option<String>,
}
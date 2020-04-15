use serde::Deserialize;

#[derive(Deserialize)]
pub struct PageRequest {
    pub start: Option<u32>,
    pub count: Option<u32>,
}


use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct FirstRow {
    dateTime: String,
    class_vec: Vec<String>,
}

impl FirstRow {
    pub fn class_vec(self) -> Vec<String> {
        self.class_vec
    }
}

#[derive(Serialize)]
pub struct Row {
    today: String,
    data: Vec<u32>,
}

impl Row {
    pub fn new(today: String, data: Vec<u32>) -> Self {
        Self { today, data }
    }
}

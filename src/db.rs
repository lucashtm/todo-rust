use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub text: String,
    pub position: usize
}

pub struct Column<'a> {
    pub title: String,
    pub tasks: &'a Vec<Task>
}

#[derive(Serialize, Deserialize)]
pub struct JsonData {
    pub todo: Vec<Task>,
    pub doing: Vec<Task>,
    pub done: Vec<Task>,
}

pub fn create_db() {
    fs::create_dir_all("data").expect("Cannot create data dir");
    let empty_data: JsonData = JsonData { todo: vec![], doing: vec![], done: vec![] };
    let empty_data = serde_json::to_string(&empty_data).expect("Invalid json");
    fs::write("data/data.json", empty_data).expect("Cannot write to data/data.json");
}

pub fn load_data() -> JsonData {
    let data = fs::read_to_string("data/data.json").expect("Couldnt open file");
    serde_json::from_str(data.as_str()).expect("Couldnt parse json")
}

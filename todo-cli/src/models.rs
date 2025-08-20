#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct Item {
    pub id: i32,
    pub desc: String,
    pub done: Progress,
}

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub enum Progress {
    Added,
    Doing,
    Done
}
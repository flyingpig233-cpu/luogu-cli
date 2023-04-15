use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LuoguProblem {
    pub tags: Vec<i64>,
    pub wantsTranslation: bool,
    pub totalSubmit: i64,
    pub totalAccepted: i64,
    pub flag: i64,
    pub pid: String,
    pub title: String,
    pub difficulty: i64,
    pub fullScore: i64,
}

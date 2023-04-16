use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LuoguProblem {
    pub tags: Vec<i64>,
    #[serde(alias = "wantsTranslation")]
    pub wants_translation: bool,
    #[serde(alias = "totalSubmit")]
    pub total_submit: i64,
    #[serde(alias = "totalAccepted")]
    pub total_accepted: i64,
    pub flag: i64,
    pub pid: String,
    pub title: String,
    pub difficulty: i64,
    #[serde(alias = "fullScore")]
    pub full_score: i64,
    #[serde(alias = "type")]
    pub problem_type: String,
}

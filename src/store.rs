use crate::types::{
    answer::{Answer, AnswerId},
    question::{Question, QuestionId},
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    pub fn new() -> Self {
        let file = include_str!("../questions.json");
        let questions = serde_json::from_str(file).expect("can't read questions.json");

        Store {
            questions: Arc::new(RwLock::new(questions)),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

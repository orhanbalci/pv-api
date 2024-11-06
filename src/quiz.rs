use serde::Serialize;

#[derive(Serialize)]
pub struct Question {
    pub proverb: String,
    pub options: Vec<String>,
    pub correct_meaning: String, // correct meaning
                                 // pub user_answer: String,
}

#[derive(Serialize)]
pub struct Quiz {
    pub questions: Vec<Question>,
}

impl Quiz {
    pub fn new() -> Self {
        Self {
            questions: Vec::new(),
        }
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
    }
}

impl Question {
    pub fn new(proverb: String, options: Vec<String>, correct_meaning: String) -> Question {
        Question {
            proverb,
            options,
            correct_meaning,
        }
    }
}

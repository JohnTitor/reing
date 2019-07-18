use crate::entity::{Answer, Question};
use uuid::Uuid;

pub trait AnswerRepository {
  fn store(&self, answer: Answer);
  fn find(&self, id: Uuid) -> Option<Answer>;
  fn find_all(&self) -> Vec<Answer>;
}

pub trait QuestionRepository {
  fn store(&self, question: Question);
  fn find(&self, id: Uuid) -> Option<Question>;
  fn find_all_not_answered_yet(&self) -> Vec<Question>;
}

pub mod mock {
  use std::collections::HashMap;
  use uuid::Uuid;
  use std::sync::Mutex;
  use crate::entity::{Answer, Question};
  use super::{QuestionRepository, AnswerRepository};

  pub mod answer_respository {
    use super::*;

    pub struct MockAnswerRepository {
      pub answers: Mutex<HashMap<Uuid, Answer>>
    }

    impl MockAnswerRepository {
      pub fn new() -> Box<AnswerRepository> {
        Box::new(
          Self {
            answers: Mutex::new(HashMap::new())
          }
        )
      }
    }

    impl AnswerRepository for MockAnswerRepository {
      fn find_all(&self) -> Vec<Answer> {
        self.answers.lock().unwrap().values().map(|a| a.clone()).collect()
      }

      fn store(&self, answer: Answer) {
        let mut answers = self.answers.lock().unwrap();
        answers.insert(answer.id, answer);
      }

      fn find(&self, id: Uuid) -> Option<Answer> {
        self.answers.lock().unwrap().get(&id).cloned()
      }
    }
  }
}
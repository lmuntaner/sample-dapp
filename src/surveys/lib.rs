use ic_cdk_macros::*;
use ic_cdk::export::{candid::{CandidType, Deserialize}};
use std::cell::RefCell;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Question {
  title: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Survey {
  id: String,
  title: String,
  questions: Vec<Question>,
}

thread_local! {
    static SURVEYS: RefCell<Vec<Survey>> = RefCell::new(Vec::new());
}

#[init]
fn init() {
    ic_cdk::println!("in da surveys init");
}

#[query]
fn read_all() -> Vec<Survey> {
    ic_cdk::println!("in da read_all");
    SURVEYS.with(|surveys| {
        return surveys.borrow_mut().clone();
    })
}

#[update]
fn create(title: String, question_inputs: Vec<String>) -> Survey {
    SURVEYS.with(|surveys_ref| {
        let mut surveys = surveys_ref.borrow_mut();
        let questions = question_inputs
            .into_iter()
            .map(|question_input| {
                Question {
                    title: question_input,
                }
            })
            .collect();
        let new_survey = Survey {
            id: "1".to_string(),
            title: title,
            questions: questions,
        };
        surveys.push(new_survey);
        return surveys[surveys.len() - 1].clone();
    })
}

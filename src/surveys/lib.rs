use ic_cdk_macros::*;
use ic_cdk::export::{candid::{CandidType, Deserialize}};
use std::cell::{Cell, RefCell};

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Question {
  title: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Survey {
  id: u64,
  title: String,
  questions: Vec<Question>,
}

thread_local! {
    static SURVEYS: RefCell<Vec<Survey>> = RefCell::new(Vec::new());
    static NEXT_USER_ID: Cell<u64> = Cell::new(0);
}

#[init]
fn init() {
    ic_cdk::println!("in da surveys init");
}

#[query]
fn read_all() -> Vec<Survey> {
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
        ic_cdk::println!("in da surveys create");
        NEXT_USER_ID.with(|next_user_id_ref| {
            let mut counter = next_user_id_ref.get();
            counter += 1u64;
            let new_survey = Survey {
                id: counter,
                title: title,
                questions: questions,
            };
            surveys.push(new_survey);
            return surveys[surveys.len() - 1].clone();
        })
    })
}

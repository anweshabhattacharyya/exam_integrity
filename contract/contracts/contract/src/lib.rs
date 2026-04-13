#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, Map};

#[contracttype]
#[derive(Clone)]
pub struct Exam {
    pub creator: String,
    pub question_hash: String,
}

#[contracttype]
#[derive(Clone)]
pub struct Submission {
    pub student: String,
    pub answer_hash: String,
}

#[contract]
pub struct ExamIntegrityContract;

#[contractimpl]
impl ExamIntegrityContract {

    // Create a new exam
    pub fn create_exam(env: Env, exam_id: Symbol, creator: String, question_hash: String) {
        let mut exams: Map<Symbol, Exam> = env.storage().instance().get(&Symbol::short("EXAMS")).unwrap_or(Map::new(&env));

        if exams.contains_key(exam_id.clone()) {
            panic!("Exam already exists");
        }

        let exam = Exam { creator, question_hash };
        exams.set(exam_id, exam);

        env.storage().instance().set(&Symbol::short("EXAMS"), &exams);
    }


    // Submit answers (only once per student)
    pub fn submit_answer(env: Env, exam_id: Symbol, student: String, answer_hash: String) {
        let key = (exam_id.clone(), student.clone());

        if env.storage().instance().has(&key) {
            panic!("Already submitted");
        }

        let submission = Submission { student, answer_hash };
        env.storage().instance().set(&key, &submission);
    }

    // Get exam details
    pub fn get_exam(env: Env, exam_id: Symbol) -> Exam {
        let exams: Map<Symbol, Exam> = env.storage().instance().get(&Symbol::short("EXAMS")).unwrap();

        exams.get(exam_id).unwrap()
    }
}
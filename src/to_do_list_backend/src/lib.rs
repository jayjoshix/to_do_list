use candid::{CandidType, Decode, Encode};
use ic_cdk::{api::caller, storage};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Clone, Debug)]
pub struct Task {
    id: u64,
    description: String,
    completed: bool,
    important: bool,
    due_date: Option<u64>,
    owner: candid::Principal,
}

thread_local! {
    static TASKS: RefCell<HashMap<candid::Principal, Vec<Task>>> = RefCell::new(HashMap::new());
    static COUNTER: RefCell<u64> = RefCell::new(0);
}

#[ic_cdk::init]
fn init() {
    COUNTER.with(|counter| *counter.borrow_mut() = 0);
    TASKS.with(|tasks| tasks.borrow_mut().clear());
}

#[ic_cdk::update]
async fn add_task(description: String, due_date: Option<u64>, important: bool) -> Task {
    let caller = caller();
    
    let task_id = COUNTER.with(|counter| {
        let current = *counter.borrow();
        *counter.borrow_mut() = current + 1;
        current
    });

    let task = Task {
        id: task_id,
        description,
        completed: false,
        important,
        due_date,
        owner: caller,
    };

    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        let user_tasks = tasks.entry(caller).or_insert_with(Vec::new);
        user_tasks.push(task.clone());
    });

    task
}

#[ic_cdk::update]
fn toggle_task_completion(task_id: u64) -> bool {
    let caller = caller();
    let mut success = false;

    TASKS.with(|tasks| {
        if let Some(user_tasks) = tasks.borrow_mut().get_mut(&caller) {
            if let Some(task) = user_tasks.iter_mut().find(|t| t.id == task_id) {
                task.completed = !task.completed;
                success = true;
            }
        }
    });

    success
}

#[ic_cdk::update]
fn toggle_task_importance(task_id: u64) -> bool {
    let caller = caller();
    let mut success = false;

    TASKS.with(|tasks| {
        if let Some(user_tasks) = tasks.borrow_mut().get_mut(&caller) {
            if let Some(task) = user_tasks.iter_mut().find(|t| t.id == task_id) {
                task.important = !task.important;
                success = true;
            }
        }
    });

    success
}

#[ic_cdk::query]
fn get_tasks() -> Vec<Task> {
    let caller = caller();
    TASKS.with(|tasks| {
        tasks
            .borrow()
            .get(&caller)
            .cloned()
            .unwrap_or_default()
    })
}

#[ic_cdk::query]
fn get_important_tasks() -> Vec<Task> {
    let caller = caller();
    TASKS.with(|tasks| {
        tasks
            .borrow()
            .get(&caller)
            .map(|user_tasks| {
                user_tasks
                    .iter()
                    .filter(|task| task.important)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    })
}

#[ic_cdk::query]
fn get_completed_tasks() -> Vec<Task> {
    let caller = caller();
    TASKS.with(|tasks| {
        tasks
            .borrow()
            .get(&caller)
            .map(|user_tasks| {
                user_tasks
                    .iter()
                    .filter(|task| task.completed)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    })
}

#[ic_cdk::update]
fn delete_task(task_id: u64) -> bool {
    let caller = caller();
    let mut success = false;

    TASKS.with(|tasks| {
        if let Some(user_tasks) = tasks.borrow_mut().get_mut(&caller) {
            if let Some(pos) = user_tasks.iter().position(|t| t.id == task_id) {
                user_tasks.remove(pos);
                success = true;
            }
        }
    });

    success
}

// Required by Candid
ic_cdk::export_candid!();
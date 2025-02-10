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
    date_added: String,
    importance_level: String,
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

/// Add a new task
/// @param {text} description - Task Description
/// @param {text} date - Due Date (YYYY-MM-DD)
/// @param {bool} importance - Is Important?
#[ic_cdk::update]
async fn add_task(
    description: String,  // Task Description
    date: String,        // Due Date (YYYY-MM-DD)
    importance: bool     // Is Important?
) -> Task {
    let caller = caller();
    
    let task_id = COUNTER.with(|counter| {
        let current = *counter.borrow();
        *counter.borrow_mut() = current + 1;
        current
    });

    let importance_level = if importance {
        "High Priority".to_string()
    } else {
        "Normal Priority".to_string()
    };

    let task = Task {
        id: task_id,
        description,
        completed: false,
        important: importance,
        date_added: date,
        importance_level,
        owner: caller,
    };

    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        let user_tasks = tasks.entry(caller).or_insert_with(Vec::new);
        user_tasks.push(task.clone());
    });

    task
}

/// Get a specific task by ID
/// @param {nat64} task_id - Task ID
#[ic_cdk::query]
fn get_task(task_id: u64) -> Option<Task> {
    let caller = caller();
    TASKS.with(|tasks| {
        tasks
            .borrow()
            .get(&caller)
            .and_then(|user_tasks| {
                user_tasks
                    .iter()
                    .find(|task| task.id == task_id)
                    .cloned()
            })
    })
}

/// Toggle task completion status
/// @param {nat64} task_id - Task ID
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

/// Toggle task importance
/// @param {nat64} task_id - Task ID
#[ic_cdk::update]
fn toggle_task_importance(task_id: u64) -> bool {
    let caller = caller();
    let mut success = false;

    TASKS.with(|tasks| {
        if let Some(user_tasks) = tasks.borrow_mut().get_mut(&caller) {
            if let Some(task) = user_tasks.iter_mut().find(|t| t.id == task_id) {
                task.important = !task.important;
                task.importance_level = if task.important {
                    "High Priority".to_string()
                } else {
                    "Normal Priority".to_string()
                };
                success = true;
            }
        }
    });

    success
}

/// Get all tasks for the current user
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

/// Get all important tasks
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

/// Get all completed tasks
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

/// Delete a task
/// @param {nat64} task_id - Task ID
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
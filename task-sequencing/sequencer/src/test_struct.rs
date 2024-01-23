#[path = "tasks.rs"] mod tasks;
#[path = "test_tasks_functions.rs"] mod test_tasks_functions;



pub fn init_tasks() -> [tasks::Task ; 2]{

    let tasks = [
        tasks::Task {name: String::from("Tache 1"), f: test_tasks_functions::task1},
        tasks::Task {name: String::from("Tache 2"), f: test_tasks_functions::task1}
    ];

    return tasks;
}
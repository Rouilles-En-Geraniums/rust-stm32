#[path = "tasks.rs"] mod tasks;
#[path = "user_tasks.rs"] mod user_tasks;



pub fn init_tasks() -> [tasks::OrdoTask ; 2]{

    let tasks = [
        tasks::OrdoTask {name: String::from("Tache 1"), task: Box::new(user_tasks::Task1 {count: 12})},
        tasks::OrdoTask {name: String::from("Tache 2"), task: Box::new(user_tasks::Task2 {})}
    ];

    return tasks;
}
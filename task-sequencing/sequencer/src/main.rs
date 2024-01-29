//mod tasks;
mod ordo_tab;

fn main() {
    println!("Hello, world!");

    let tasks = ordo_tab::init_tasks();

    for task in tasks {
        println!("{}", task.name);
    }

    //println!("{}", task.name);
    //(task.f)();
}
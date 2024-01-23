//mod tasks;
mod test_struct;

fn main() {
    println!("Hello, world!");

    let tasks = test_struct::init_tasks();

    for task in tasks {
        println!("{}", task.name);
    }

    //println!("{}", task.name);
    //(task.f)();
}
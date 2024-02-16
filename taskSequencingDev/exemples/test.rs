
trait Ordonancable {
    fn run(&mut self) {}
}

struct Sequencer {
    jobs: Vec<Box<dyn Ordonancable>>
}

impl Sequencer {
    fn launch(&mut self) {
        for component in self.jobs.iter_mut() {
            component.run();
        }
    }
}

#[derive(Debug)]
struct Task1 {
    label: String,
    item1: i32,
    item2: u32
}

#[derive(Debug)]
struct Task2 {
    label: String,
    item1: String,
    item2: String
}

impl Ordonancable for Task1 {
    fn run(&mut self) {
        println!("Hello from Task1::{}", self.label);
        println!("item1 {}", self.item1);
        println!("item2 {}", self.item2);
    }
}

impl Ordonancable for Task2 {
    fn run(&mut self) {
        println!("Hello from Task1::{}", self.label);
        println!("item1 {}", self.item1);
        println!("item2 {}", self.item2);
    }
}

fn main () {
    let task1 = Task1 {label: String::from("Tache1"), item1: -10, item2: 20};
    let task2 = Task2 {label: String::from("Tache2"), item1: String::from("item1"), item2: String::from("item2")};

    println!("Task1 : {task1:?}");
    println!("Task2 : {task2:?}");

    let jobs: Vec<Box<dyn Ordonancable>> = vec![
       Box::new(task1),
       Box::new(task2)
    ];
    let mut seq = Sequencer {jobs};

    seq.launch();
}

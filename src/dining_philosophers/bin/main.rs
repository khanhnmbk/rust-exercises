use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::{thread, vec};
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    // left_fork: ...
    // right_fork: ...
    // thoughts: ...
    left_fork : Arc<Mutex<Fork>>,
    right_fork : Arc<Mutex<Fork>>,
    thoughts: Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks...
        let _left = self.left_fork.lock().unwrap();
        let _right = self.right_fork.lock().unwrap();

        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    // Create forks
    let forks = (0..PHILOSOPHERS.len()).map(|_| Arc::new(Mutex::<Fork>::new(Fork{}))).collect::<Vec<_>>();

    // Create channels
    let (tx, rx) = mpsc::channel();

    // Create philosophers
    for i in 0..forks.len() {
        let tx_i = tx.clone();
        let mut left_fork = forks[i].clone();
        let mut right_fork = forks[(i+1) % forks.len()].clone();
        if i == forks.len() - 1 {
            std::mem::swap(&mut left_fork, &mut right_fork);
        }

        let p = Philosopher{
            name: String::from(PHILOSOPHERS[i]),
            left_fork,
            right_fork,
            thoughts: tx_i,
        };

        thread::spawn(move || {
            for _ in 0..100 {
                p.eat();
                p.think();
            }
        });
    }

    // Drop original sender to avoid blocking
    drop(tx);

    // Output their thoughts
    for v in rx.iter() {
        println!("{v}");
    }

}

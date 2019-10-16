use crossbeam::channel::{select, unbounded, Receiver, Sender};
use ctrlc;

use std::collections::HashMap;
use std::io::stdout;
use std::io::Write;
use std::thread;
use std::time::Duration;

use crate::config::Config;
use crate::status::Status;

enum Event {
    Status(usize, Status),
    Interrupt,
}

fn render(distribution: &HashMap<Status, usize>) -> String {
    distribution
        .iter()
        .map(|(status, count)| format!("{} => {}", status, count))
        .collect::<Vec<_>>()
        .join("\n")
}

fn worker(id: usize, config: Config, tx: Sender<Event>) -> (Sender<()>, thread::JoinHandle<()>) {
    let (int_tx, int_rx): (Sender<()>, Receiver<()>) = unbounded();
    let worker = thread::spawn(move || {
        eprintln!("Worker {} started", id);

        loop {
            select! {
                recv(int_rx) -> _ => break,
                default(Duration::from_secs(0)) => (),
            }

            if tx.send(Event::Status(id, Status::check(&config))).is_err() {
                break;
            }
        }

        // Sending is a non-blocking operation, the thread will continue
        // immediately after sending its message
        eprintln!("Worker {} finished", id);
    });

    (int_tx, worker)
}

pub fn workers(config: Config) {
    let (tx, rx): (Sender<Event>, Receiver<Event>) = unbounded();
    let mut children = Vec::new();
    let mut interrupt_channels = Vec::new();
    let mut distribution = HashMap::new();

    for id in 0..config.parallelism {
        let (int_tx, child) = worker(id, config.clone(), tx.clone());
        interrupt_channels.push(int_tx);
        children.push(child);
    }

    ctrlc::set_handler(move || {
        for int_tx in &interrupt_channels {
            int_tx.send(()).unwrap();
        }

        tx.clone().send(Event::Interrupt).unwrap();
    })
    .unwrap();

    loop {
        match rx.recv().unwrap() {
            Event::Status(_id, status) => {
                let count = match distribution.get(&status) {
                    Some(count) => count + 1,
                    None => 1,
                };
                distribution.insert(status, count);
                print!("{}\r", render(&distribution));
                stdout().flush().unwrap();
            }
            Event::Interrupt => {
                break;
            }
        };
    }

    for child in children {
        child.join().unwrap();
    }

    println!("{}", render(&distribution));
}

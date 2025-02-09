use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;
use tokio::sync::oneshot;

// Define the task type that includes its response channel
#[derive(Debug)]
struct ProcessData {
    id: u32,
    data: String,
    response_tx: oneshot::Sender<String>,
}

#[derive(Debug)]
enum Task {
    Work(ProcessData),
    Exit,
}

// Worker thread function
fn worker(id: u32, task_rx: Receiver<Task>) {
    println!("Worker {} started", id);

    while let Ok(task) = task_rx.recv() {
        match task {
            Task::Exit => {
                println!("Worker {} exit", id);
                break;
            }
            Task::Work(task) => {
                println!("Worker {} processing task {}", id, task.id);

                // Simulate some processing work
                thread::sleep(Duration::from_millis(500));

                // Process the task and send result through the oneshot channel
                let result = format!(
                    "Task {} with data '{}' processed by worker {}",
                    task.id, task.data, id
                );

                // Send result through the task's oneshot channel
                let _ = task.response_tx.send(result);
            }
        }
    }

    println!("Worker {} shutting down", id);
}

#[tokio::main]
async fn main() {
    const NUM_WORKERS: usize = 4;
    const NUM_TASKS: usize = 10;

    let mut task_txs = Vec::new();
    // Spawn worker threads
    let mut workers = Vec::new();
    for id in 0..NUM_WORKERS {
        // Create mpsc channel for distributing tasks
        let (task_tx, task_rx) = channel();
        workers.push(thread::spawn(move || {
            worker(id as u32, task_rx);
        }));
        task_txs.push(task_tx);
    }

    // Create and send tasks, collecting response receivers
    let mut response_rxs = Vec::new();

    for i in 0..NUM_TASKS {
        // Create a oneshot channel for this task's response
        let (response_tx, response_rx) = oneshot::channel();

        let task = Task::Work(ProcessData {
            id: i as u32,
            data: format!("Data for task {}", i),
            response_tx,
        });

        let task_tx = task_txs.get(i % NUM_WORKERS).unwrap();
        task_tx.send(task).unwrap();
        response_rxs.push((i, response_rx));

        // Small delay between sending tasks
        thread::sleep(Duration::from_millis(100));
    }

    // Collect all results
    for (task_id, response_rx) in response_rxs {
        match response_rx.await {
            Ok(result) => println!("Received result for task {}: {}", task_id, result),
            Err(_) => println!("Failed to receive result for task {}", task_id),
        }
    }

    // Send exit requests
    task_txs.into_iter().for_each(|t| {
        t.send(Task::Exit).unwrap();
    });

    // Wait for all workers to complete
    for (id, worker) in workers.into_iter().enumerate() {
        worker.join().unwrap();
        println!("Worker {} has shut down", id);
    }

    println!("All tasks completed!");
}

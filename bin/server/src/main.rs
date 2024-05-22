use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Number of executors
    let num_executors = 2;

    // Create a channel for server communication with a buffer size of 32
    let (server_tx, server_rx) = mpsc::channel(32);

    // Spawn the server thread
    let server_handle = task::spawn(run_server(server_rx));

    // Create channels for manager to executor communication
    let mut executor_txs = Vec::new();
    let mut executor_handles = Vec::new();

    for i in 0..num_executors {
        let (executor_tx, executor_rx) = mpsc::channel(32);
        executor_txs.push(executor_tx);
        let handle = task::spawn(run_executor(executor_rx, i));
        executor_handles.push(handle);
    }

    // Spawn the manager thread
    let manager_handle = task::spawn(run_manager(server_tx, executor_txs));

    // Wait for all tasks to complete
    let handles = vec![server_handle, manager_handle];
    let _ = join_all(handles, executor_handles).await;
}

async fn join_all(
    handles: Vec<tokio::task::JoinHandle<()>>,
    executor_handles: Vec<tokio::task::JoinHandle<()>>,
) {
    for handle in handles {
        let _ = handle.await;
    }
    for handle in executor_handles {
        let _ = handle.await;
    }
}

// Server logic
async fn run_server(mut rx: mpsc::Receiver<String>) {
    while let Some(message) = rx.recv().await {
        println!("Server received: {}", message);

        // Simulate processing time
        sleep(Duration::from_secs(1)).await;

        println!("Server processed message: {}", message);
    }
}

// Manager logic
async fn run_manager(server_tx: mpsc::Sender<String>, executor_txs: Vec<mpsc::Sender<String>>) {
    for i in 0..5 {
        let message = format!("Message {}", i);
        println!("Manager sending to server: {}", message);
        if server_tx.send(message.clone()).await.is_err() {
            println!("Manager failed to send message to server");
        }

        let executor_message = format!("Executor Task {}", i);
        println!("Manager sending to executors: {}", executor_message);

        for (index, executor_tx) in executor_txs.iter().enumerate() {
            if executor_tx.send(executor_message.clone()).await.is_err() {
                println!("Manager failed to send message to executor {}", index);
            }
        }

        // Simulate time between sending messages
        sleep(Duration::from_secs(2)).await;
    }
}

// Executor logic
async fn run_executor(mut rx: mpsc::Receiver<String>, id: usize) {
    while let Some(message) = rx.recv().await {
        println!("Executor {} received: {}", id, message);

        // Simulate processing time
        sleep(Duration::from_secs(1)).await;

        println!("Executor {} processed message: {}", id, message);
    }
}

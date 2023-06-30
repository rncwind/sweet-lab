use rand::distributions::{Alphanumeric, DistString};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    // Create an empty message queue.
    let messagequeue: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let write_for = std::time::Duration::from_secs(5);
    let start = std::time::Instant::now();

    loop {
        let messagequeue = messagequeue.clone();

        tokio::spawn(async move {
            message_write(messagequeue.clone()).await;
        });
        if start.elapsed() >= write_for {
            break;
        }
    }
    message_consumer(messagequeue.clone()).await;
}

async fn message_write(messagequeue: Arc<Mutex<Vec<String>>>) {
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    messagequeue.lock().unwrap().push(string);
    std::thread::sleep(std::time::Duration::from_millis(100));
}

async fn message_consumer(messagequeue: Arc<Mutex<Vec<String>>>) {
    let mq = messagequeue.lock().unwrap();
    println!("Wrote {} messages", mq.len());
}

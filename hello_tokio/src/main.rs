use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // The first thing a redis server needs to do is accept TCP sockets.
    // We will bind to a tokio::net::TcpListener to do this.
    let listener = TcpListener::bind("0.0.0.0:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        // A new task is spawned for each inbound socket.
        // The socket is moved to the new task and processed there.

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // // Create a new hashmap that stores our data.
    // let mut db = HashMap::new();

    // A connection handles the parsing of frames from our socket.
    let mut connection = Connection::new(socket);

    // While we can read a command from the connection
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(v) = db.get(cmd.key()) {
                    // If the key exists in our DB
                    Frame::Bulk(v.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        // Write the response
        connection.write_frame(&response).await.unwrap();
    }
}

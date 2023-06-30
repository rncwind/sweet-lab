use tokio::sync::mpsc;

#[tokio::main]
fn main() {
    let (tx, mut rx) = mpsc::channel(128);
}

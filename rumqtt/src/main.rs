use log::*;
use rand::Rng;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use tokio::{task, time};
use tracing::instrument;

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut mqttoptions = MqttOptions::new("rumqtt-test", "0.0.0.0", 1883);
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe("hello/rumqtt", QoS::AtMostOnce)
        .await
        .unwrap();

    info!("Spawning");
    task::spawn(async move {
        for i in 0..10 {
            client
                .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize])
                .await
                .unwrap();
            time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    info!("Polling loop");
    while let Ok(notification) = eventloop.poll().await {
        println!("Got {:?}", notification);
    }
}

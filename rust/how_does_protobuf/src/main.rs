use log::*;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use tracing::instrument;

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt::init();
}

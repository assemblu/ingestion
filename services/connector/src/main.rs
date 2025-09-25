use anyhow::*;
use governor::{Quota, RateLimiter, state::InMemoryState, clock::DefaultClock};
use rdkafka::{ClientConfig, producer::{FutureProducer, FutureRecord}};
use serde::Serialize;
use std::{num::NonZeroU32, time::Duration};
use tokio::{select, time};
use tracing::{info, warn, error};
mod venues;
use venues::hyperliquid::run_hyperliquid;

#[derive(Clone)]
struct Cfg {
    brokers: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let cfg = Cfg {
        brokers: std::env::var("KAFKA_BROKERS").unwrap_or("localhost:9092".into()),
    };

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &cfg.brokers)
        .set("enable.idempotence", "true")
        .set("acks", "all")
        .set("compression.type", "zstd")
        .set("linger.ms", "5")
        .set("retries", "2147483647")
        .set("socket.keepalive.enable", "true")
        .create()?;

    let rest_rl = RateLimiter::<InMemoryState, DefaultClock>::direct(
        Quota::per_second(NonZeroU32::new(8).unwrap()).allow_burst(NonZeroU32::new(16).unwrap()));

    // Stub: start Hyperliquid connector. Real endpoints moved to config.
    let res = run_hyperliquid(producer, rest_rl).await;
    if let Err(e) = res { error!(?e, "connector error"); }
    Ok(())
}

#[derive(Serialize)]
struct Trade {
    venue: String, symbol: String, channel: String, seq: u64,
    ts_exchange: String, ts_gateway: String,
    px: f64, qty: f64, aggressor: String, trade_id: String,
    src_conn_id: u64
}

async fn produce_json<T: Serialize>(producer: &FutureProducer, topic: &str, key: &str, v: &T) -> Result<()> {
    let payload = serde_json::to_vec(v)?;
    producer.send(
        FutureRecord::to(topic).key(key).payload(&payload),
        Duration::from_secs(5)
    ).await.map_err(|(e, _)| anyhow!(e))?;
    Ok(())
}

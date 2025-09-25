use anyhow::*;
use futures_util::{SinkExt, StreamExt};
use governor::{RateLimiter, clock::DefaultClock, state::InMemoryState};
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, num::NonZeroU32, time::{Duration, SystemTime, UNIX_EPOCH}};
use tokio::{sync::Mutex, time::sleep};
use tokio_tungstenite::connect_async;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

const TOPIC_TRADES: &str = "trades";

pub async fn run_hyperliquid(
    producer: FutureProducer,
    _rest_rl: RateLimiter<InMemoryState, DefaultClock>
) -> Result<()> {
    let mock = std::env::var("MOCK").unwrap_or_default();
    if matches!(mock.as_str(), "1" | "true" | "TRUE" | "yes") {
        info!("starting connector in MOCK mode");
        run_mock(producer).await
    } else {
        info!("starting connector in REAL WS mode");
        run_real_ws(producer).await
    }
}

async fn run_real_ws(producer: FutureProducer) -> Result<()> {
    let venue = std::env::var("VENUE").unwrap_or_else(|_| "hyperliquid".to_string());
    let ws_url = std::env::var("WS_URL").unwrap_or_else(|_| "wss://api.hyperliquid.xyz/ws".to_string());
    let symbols_env = std::env::var("SYMBOLS").unwrap_or_else(|_| "BTC".to_string());
    let symbols: Vec<String> = symbols_env.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    if symbols.is_empty() { bail!("SYMBOLS is empty"); }

    let src_conn_id: u64 = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64) ^ (Uuid::new_v4().as_u128() as u64);

    let (ws_stream, _resp) = connect_async(&ws_url).await.context("connect ws")?;
    info!(%ws_url, symbols=?symbols, "connected to WS");
    let (mut write, mut read) = ws_stream.split();

    // Best-effort subscribe message. Adjust per Hyperliquid docs via SUBSCRIBE_MSG template if needed.
    // If SUBSCRIBE_MSG is provided, it will be sent as-is (with `$SYMBOLS` placeholder replaced by array JSON).
    let default_sub = serde_json::json!({
        "method": "subscribe",
        "subscriptions": symbols.iter().map(|s| serde_json::json!({"channel":"trades","symbol": s})).collect::<Vec<_>>()
    });
    let subscribe_raw = std::env::var("SUBSCRIBE_MSG").ok();
    let subscribe_msg = if let Some(tpl) = subscribe_raw {
        let syms_json = serde_json::to_string(&symbols).unwrap_or("[]".to_string());
        let payload = tpl.replace("$SYMBOLS", &syms_json);
        payload
    } else {
        serde_json::to_string(&default_sub)?
    };
    write.send(tokio_tungstenite::tungstenite::Message::Text(subscribe_msg.clone())).await?;
    info!("sent subscribe: {}", subscribe_msg);

    let counters: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
    loop {
        match read.next().await {
            Some(Ok(msg)) => {
                if !msg.is_text() { continue; }
                let txt = msg.into_text().unwrap_or_default();
                if txt.is_empty() { continue; }
                match serde_json::from_str::<Value>(&txt) {
                    Ok(v) => {
                        // Attempt to extract one or many trades
                        let items = flatten_trade_items(&v);
                        for item in items {
                            if let Some(mut te) = extract_trade(&item) {
                                if !symbols.contains(&te.symbol) { continue; }
                                // sequence handling
                                if te.seq.is_none() {
                                    let mut map = counters.lock().await;
                                    let c = map.entry(te.symbol.clone()).or_insert(0);
                                    *c += 1; te.seq = Some(*c);
                                }
                                let env = TradeEnvelope::from_trade_event(&venue, te, src_conn_id);
                                let key = format!("{}|{}", env.venue, env.symbol);
                                let payload = serde_json::to_vec(&env)?;
                                producer.send(
                                    FutureRecord::to(TOPIC_TRADES).key(&key).payload(&payload),
                                    Duration::from_secs(5)
                                ).await.map_err(|(e, _)| anyhow!(e))?;
                            }
                        }
                    }
                    Err(e) => {
                        debug!(err=?e, msg=%txt, "json parse error");
                    }
                }
            }
            Some(Err(e)) => {
                warn!(err=?e, "ws read error");
                break;
            }
            None => {
                warn!("ws closed by server");
                break;
            }
        }
    }
    Ok(())
}

async fn run_mock(producer: FutureProducer) -> Result<()> {
    let venue = std::env::var("VENUE").unwrap_or_else(|_| "hyperliquid".to_string());
    let symbols_env = std::env::var("SYMBOLS").unwrap_or_else(|_| "BTC,ETH".to_string());
    let symbols: Vec<String> = symbols_env.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    let src_conn_id: u64 = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64) ^ (Uuid::new_v4().as_u128() as u64);
    let rate_ms_per_symbol: u64 = std::env::var("MOCK_RATE_MS").ok().and_then(|s| s.parse().ok()).unwrap_or(600); // ~100/min

    let mut handles = Vec::new();
    for sym in symbols.clone() {
        let prod = producer.clone();
        let v = venue.clone();
        handles.push(tokio::spawn(async move {
            let mut seq: u64 = 0;
            loop {
                seq += 1;
                let now_ns = now_ns();
                let price = 10000.0 + (seq as f64 % 100.0) * 0.01;
                let qty = 0.001 + ((seq % 10) as f64) * 0.0001;
                let aggressor = if seq % 2 == 0 { "buy" } else { "sell" };
                let env = TradeEnvelope {
                    venue: v.clone(),
                    symbol: sym.clone(),
                    channel: "trades".to_string(),
                    seq,
                    ts_exchange: now_ns,
                    ts_gateway: now_ns,
                    px: price,
                    qty,
                    aggressor: aggressor.to_string(),
                    trade_id: format!("{}-{}-{}", &sym, seq, now_ns),
                    src_conn_id,
                };
                let key = format!("{}|{}", env.venue, env.symbol);
                let payload = serde_json::to_vec(&env).unwrap();
                if let Err((e, _)) = prod.send(FutureRecord::to(TOPIC_TRADES).key(&key).payload(&payload), Duration::from_secs(5)).await {
                    warn!(err=?e, "mock send error");
                }
                sleep(Duration::from_millis(rate_ms_per_symbol)).await;
            }
        }));
    }
    for h in handles { let _ = h.await; }
    Ok(())
}

#[derive(Debug, Clone)]
struct TradeEvent {
    symbol: String,
    price: f64,
    qty: f64,
    aggressor: String,
    trade_id: String,
    ts_exchange: u128,
    seq: Option<u64>,
}

#[derive(Serialize)]
struct TradeEnvelope {
    venue: String,
    symbol: String,
    channel: String,
    seq: u64,
    ts_exchange: u128,
    ts_gateway: u128,
    px: f64,
    qty: f64,
    aggressor: String,
    trade_id: String,
    src_conn_id: u64,
}

impl TradeEnvelope {
    fn from_trade_event(venue: &str, te: TradeEvent, src_conn_id: u64) -> Self {
        Self {
            venue: venue.to_string(),
            symbol: te.symbol,
            channel: "trades".to_string(),
            seq: te.seq.unwrap_or(0),
            ts_exchange: te.ts_exchange,
            ts_gateway: now_ns(),
            px: te.price,
            qty: te.qty,
            aggressor: te.aggressor,
            trade_id: te.trade_id,
            src_conn_id,
        }
    }
}

fn now_ns() -> u128 {
    let ns = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (ns.as_secs() as u128) * 1_000_000_000u128 + (ns.subsec_nanos() as u128)
}

fn flatten_trade_items(v: &Value) -> Vec<Value> {
    // Try to extract arrays or single objects under common keys
    if let Some(arr) = v.as_array() { return arr.clone(); }
    if let Some(data) = v.get("data") {
        if let Some(arr) = data.as_array() { return arr.clone(); }
        return vec![data.clone()];
    }
    if let Some(trades) = v.get("trades") { if let Some(arr) = trades.as_array() { return arr.clone(); } }
    vec![v.clone()]
}

fn extract_trade(v: &Value) -> Option<TradeEvent> {
    // Symbol
    let symbol = v.get("symbol").and_then(Value::as_str)
        .or_else(|| v.get("s").and_then(Value::as_str))
        .or_else(|| v.get("coin").and_then(Value::as_str))?
        .to_string();
    // Price
    let price = get_num(v, &["px","price","p"])?.as_f64()?;
    // Qty
    let qty = get_num(v, &["qty","size","q"]).and_then(|n| n.as_f64()).unwrap_or(0.0);
    // Side/Aggressor
    let aggressor = v.get("aggressor").and_then(Value::as_str)
        .or_else(|| v.get("side").and_then(Value::as_str))
        .map(|s| s.to_lowercase())
        .or_else(|| v.get("isBuyerMaker").and_then(Value::as_bool).map(|b| if b { "sell".to_string() } else { "buy".to_string() }))
        .unwrap_or_else(|| "buy".to_string());
    // Trade id
    let trade_id = v.get("trade_id").and_then(Value::as_str)
        .or_else(|| v.get("id").and_then(Value::as_str))
        .or_else(|| v.get("tid").and_then(Value::as_str))
        .unwrap_or("")
        .to_string();
    // Timestamp
    let ts_raw = get_num(v, &["ts","timestamp","time","T","t"]).and_then(|n| n.as_u64()).unwrap_or(0);
    let ts_exchange = scale_to_ns(ts_raw as u128);
    // Sequence if present
    let seq = get_num(v, &["seq","sequence","event_id"]).and_then(|n| n.as_u64());
    Some(TradeEvent { symbol, price, qty, aggressor, trade_id, ts_exchange, seq })
}

fn get_num<'a>(v: &'a Value, keys: &[&str]) -> Option<&'a Value> {
    for k in keys { if let Some(x) = v.get(*k) { if x.is_number() { return Some(x); } } }
    None
}

fn scale_to_ns(ts: u128) -> u128 {
    match ts {
        0 => now_ns(),
        t if t < 1_000_000_000u128 => t * 1_000_000_000u128, // seconds
        t if t < 1_000_000_000_000u128 => t * 1_000_000u128, // milliseconds
        t if t < 1_000_000_000_000_000u128 => t * 1_000u128, // microseconds
        t => t, // already ns
    }
}

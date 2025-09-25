import os, json, time, random
from datetime import datetime, timezone
from kafka import KafkaProducer

brokers = os.getenv("KAFKA_BROKERS","redpanda:9092")
symbols = int(os.getenv("SYMBOL_SET_SIZE","4"))
producer = KafkaProducer(bootstrap_servers=brokers, value_serializer=lambda v: json.dumps(v).encode())

syms = [f"SYM{i}" for i in range(symbols)]
seq = {s:0 for s in syms}
src_id = 1

def now_ns():
    return int(time.time_ns())

while True:
    s = random.choice(syms)
    seq[s]+=1
    ts = now_ns()
    trade = {
      "venue":"sim", "symbol":s, "channel":"trades", "seq":seq[s],
      "ts_exchange": ts, "ts_gateway": ts,
      "px": round(10000+random.random()*10, 9),
      "qty": round(random.random(), 9),
      "aggressor":"buy" if random.random()>0.5 else "sell",
      "trade_id": f"{s}-{seq[s]}",
      "src_conn_id": src_id
    }
    producer.send("trades", key=f"sim|{s}".encode(), value=trade)
    if random.random()<0.2:
        q = {
          "venue":"sim","symbol":s,"channel":"quotes","seq":seq[s],
          "ts_exchange": ts,"ts_gateway": ts,
          "bid_px": trade["px"]-0.5, "bid_qty": 1.0,
          "ask_px": trade["px"]+0.5, "ask_qty": 1.0,
          "src_conn_id": src_id
        }
        producer.send("quotes", key=f"sim|{s}".encode(), value=q)
    producer.flush()
    time.sleep(0.01)


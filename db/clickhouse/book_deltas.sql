CREATE TABLE IF NOT EXISTS mart.book_deltas
(
  venue LowCardinality(String),
  symbol LowCardinality(String),
  ts_exchange DateTime64(9),
  ts_gateway  DateTime64(9),
  ts_ingest   DateTime64(9) DEFAULT now64(9),
  channel LowCardinality(String),
  seq UInt64,
  side LowCardinality(String),   -- "bid"/"ask"
  level UInt16,
  px Decimal64(9),
  qty Decimal64(9),
  action LowCardinality(String), -- "upsert"/"delete"
  src_conn_id UInt64
)
ENGINE = ReplicatedReplacingMergeTree('/ch/tables/{shard}/mart/book_deltas','{replica}', seq)
PARTITION BY toDate(ts_exchange)
ORDER BY (venue, symbol, ts_exchange, seq)
TTL ts_exchange + toIntervalDay({RETENTION_DAYS:UInt16}) DELETE
SETTINGS storage_policy='hdd_only', index_granularity=8192;


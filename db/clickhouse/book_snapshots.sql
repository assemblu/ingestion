CREATE TABLE IF NOT EXISTS mart.book_snapshots
(
  venue LowCardinality(String),
  symbol LowCardinality(String),
  ts_exchange DateTime64(9),
  ts_gateway  DateTime64(9),
  ts_ingest   DateTime64(9) DEFAULT now64(9),
  snapshot_seq UInt64,
  book_ipc_bytes String CODEC(ZSTD(6)),
  src_conn_id UInt64
)
ENGINE = ReplicatedReplacingMergeTree('/ch/tables/{shard}/mart/book_snapshots','{replica}', snapshot_seq)
PARTITION BY toDate(ts_exchange)
ORDER BY (venue, symbol, ts_exchange, snapshot_seq)
TTL ts_exchange + toIntervalDay({RETENTION_DAYS:UInt16}) DELETE
SETTINGS storage_policy='hdd_only', index_granularity=8192;


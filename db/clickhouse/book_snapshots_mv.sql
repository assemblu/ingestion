CREATE MATERIALIZED VIEW IF NOT EXISTS staging.book_snapshots_mv
TO mart.book_snapshots
AS SELECT
  venue, symbol, toDateTime64(ts_exchange,9), toDateTime64(ts_gateway,9),
  snapshot_seq, book_ipc_bytes, src_conn_id
FROM staging.book_snapshots_kafka;


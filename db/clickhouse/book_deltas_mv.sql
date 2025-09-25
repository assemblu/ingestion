CREATE MATERIALIZED VIEW IF NOT EXISTS staging.book_deltas_mv
TO mart.book_deltas
AS SELECT
  venue, symbol, toDateTime64(ts_exchange,9), toDateTime64(ts_gateway,9),
  now64(9), channel, seq, side, level, px, qty, action, src_conn_id
FROM staging.book_deltas_kafka;


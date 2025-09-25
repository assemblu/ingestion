CREATE MATERIALIZED VIEW IF NOT EXISTS staging.quotes_mv
TO mart.quotes
AS SELECT
  venue, symbol, toDateTime64(ts_exchange,9), toDateTime64(ts_gateway,9),
  now64(9), channel, seq, bid_px, bid_qty, ask_px, ask_qty, src_conn_id
FROM staging.quotes_kafka;


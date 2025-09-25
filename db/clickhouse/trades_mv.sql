CREATE MATERIALIZED VIEW IF NOT EXISTS staging.trades_mv
TO mart.trades
AS SELECT
  venue, symbol, toDateTime64(ts_exchange,9) AS ts_exchange,
  toDateTime64(ts_gateway,9) AS ts_gateway,
  now64(9) AS ts_ingest,
  channel, seq, px, qty, aggressor, trade_id, src_conn_id
FROM staging.trades_kafka;


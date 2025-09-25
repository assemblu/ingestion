CREATE TABLE IF NOT EXISTS staging.quotes_kafka
(
  venue String, symbol String, channel String, seq UInt64,
  ts_exchange DateTime64(9), ts_gateway DateTime64(9),
  bid_px Decimal64(9), bid_qty Decimal64(9),
  ask_px Decimal64(9), ask_qty Decimal64(9),
  src_conn_id UInt64
)
ENGINE = Kafka
SETTINGS kafka_broker_list = '{KAFKA_BROKERS}',
         kafka_topic_list = 'quotes',
         kafka_group_name = 'ch_quotes',
         kafka_format = 'JSONEachRow',
         kafka_num_consumers = 1,
         kafka_handle_error_mode = 'stream';


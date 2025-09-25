CREATE TABLE IF NOT EXISTS staging.book_deltas_kafka
(
  venue String, symbol String, channel String, seq UInt64,
  ts_exchange DateTime64(9), ts_gateway DateTime64(9),
  side String, level UInt16, px Decimal64(9), qty Decimal64(9), action String,
  src_conn_id UInt64
)
ENGINE = Kafka
SETTINGS kafka_broker_list='{KAFKA_BROKERS}',
         kafka_topic_list='book_deltas',
         kafka_group_name='ch_book_deltas',
         kafka_format='JSONEachRow',
         kafka_num_consumers=1,
         kafka_handle_error_mode='stream';


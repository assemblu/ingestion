CREATE TABLE IF NOT EXISTS staging.book_snapshots_kafka
(
  venue String, symbol String,
  ts_exchange DateTime64(9), ts_gateway DateTime64(9),
  snapshot_seq UInt64, book_ipc_bytes String, src_conn_id UInt64
)
ENGINE = Kafka
SETTINGS kafka_broker_list='{KAFKA_BROKERS}',
         kafka_topic_list='book_snapshots',
         kafka_group_name='ch_book_snapshots',
         kafka_format='JSONEachRow',
         kafka_num_consumers=1,
         kafka_handle_error_mode='stream';


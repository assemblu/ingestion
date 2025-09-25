CREATE DATABASE IF NOT EXISTS staging;
INCLUDE '/db/storage_policy_hdd.sql';
INCLUDE '/db/settings_hdd_ingest.sql';
INCLUDE '/db/trades.sql';
INCLUDE '/db/quotes.sql';
INCLUDE '/db/book_deltas.sql';
INCLUDE '/db/book_snapshots.sql';
INCLUDE '/db/trades_kafka.sql';
INCLUDE '/db/trades_mv.sql';
INCLUDE '/db/quotes_kafka.sql';
INCLUDE '/db/quotes_mv.sql';
INCLUDE '/db/book_deltas_kafka.sql';
INCLUDE '/db/book_deltas_mv.sql';
INCLUDE '/db/book_snapshots_kafka.sql';
INCLUDE '/db/book_snapshots_mv.sql';


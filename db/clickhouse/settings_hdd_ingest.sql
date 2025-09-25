-- HDD friendly settings
SET max_bytes_before_external_group_by = 0;
SET max_bytes_before_external_sort = 0;
SET merge_tree = '{
  "parts_to_delay_insert": 200,
  "parts_to_throw_insert": 500,
  "max_bytes_to_merge_at_min_space_in_pool": 1073741824
}';
SET background_pool_size = 8;
SET background_move_pool_size = 2;
SET background_fetches_pool_size = 4;
SET max_insert_threads = 4;
SET input_format_parallel_parsing = 1;
SET insert_deduplicate = 1;


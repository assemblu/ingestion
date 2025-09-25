# Fault injection ideas
- Kill connector task per shard.
- Drop 1% of messages before Kafka (simulate loss → ensure CH dedupe + gap repair).
- Burst 10x messages for 2s.


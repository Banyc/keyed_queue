# `keyed_queue`

Manage `HashMap<K, VecDeque<V>>` for you.

## Usage

```rust
let mut keyed_queue = KeyedQueue::new();
keyed_queue.push_back(&1, 12);
keyed_queue.push_front(&1, 11);
keyed_queue.push_back(&1, 13);

keyed_queue.push_back(&2, 21);
keyed_queue.push_back(&2, 22);
keyed_queue.push_back(&2, 23);

assert_eq!(keyed_queue.pop_front(&1), Some(11));
assert_eq!(keyed_queue.pop_back(&1), Some(13));
assert_eq!(keyed_queue.pop_back(&1), Some(12));

assert_eq!(keyed_queue.pop_front(&2), Some(21));
assert_eq!(keyed_queue.pop_front(&2), Some(22));
assert_eq!(keyed_queue.pop_front(&2), Some(23));
```

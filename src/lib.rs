use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

#[derive(Debug, Clone)]
pub struct KeyedQueue<K, V> {
    inner: HashMap<K, VecDeque<V>>,
}

impl<K, V> KeyedQueue<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}
impl<K, V> Default for KeyedQueue<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> KeyedQueue<K, V>
where
    K: Clone + Hash + Eq,
{
    pub fn push_front(&mut self, key: &K, value: V) {
        let queue = self.get_or_create_queue_mut(key);
        queue.push_front(value);
    }

    pub fn push_back(&mut self, key: &K, value: V) {
        let queue = self.get_or_create_queue_mut(key);
        queue.push_back(value);
    }

    fn get_or_create_queue_mut(&mut self, key: &K) -> &mut VecDeque<V> {
        if self.inner.get(key).is_none() {
            let queue = VecDeque::new();
            self.inner.insert(key.clone(), queue);
        }
        self.inner.get_mut(key).unwrap()
    }

    pub fn pop_front(&mut self, key: &K) -> Option<V> {
        self.pop(key, PopEnd::Front)
    }

    pub fn pop_back(&mut self, key: &K) -> Option<V> {
        self.pop(key, PopEnd::Back)
    }

    fn pop(&mut self, key: &K, end: PopEnd) -> Option<V> {
        let queue = match self.inner.get_mut(key) {
            Some(queue) => queue,
            None => return None,
        };
        let value = match end {
            PopEnd::Front => queue.pop_front(),
            PopEnd::Back => queue.pop_back(),
        };
        if queue.is_empty() {
            self.inner.remove(key);
        }
        value
    }
}

enum PopEnd {
    Front,
    Back,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
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
        assert!(keyed_queue.inner.get(&1).is_none());

        assert_eq!(keyed_queue.pop_front(&2), Some(21));
        assert_eq!(keyed_queue.pop_front(&2), Some(22));
        assert_eq!(keyed_queue.pop_front(&2), Some(23));
        assert!(keyed_queue.inner.get(&2).is_none());
    }
}

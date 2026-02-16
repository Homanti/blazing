use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct Broadcaster<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    channels: Arc<DashMap<K, broadcast::Sender<V>>>,
    capacity: usize,
}

impl<K, V> Broadcaster<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            channels: Arc::new(DashMap::new()),
            capacity: 1000,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            channels: Arc::new(DashMap::new()),
            capacity,
        }
    }

    pub fn subscribe(&self, key: &K) -> broadcast::Receiver<V> {
        self.channels
            .entry(key.clone())
            .or_insert_with(|| broadcast::channel(self.capacity).0)
            .subscribe()
    }

    pub fn broadcast(&self, key: &K, message: V) -> Result<usize, String> {
        if let Some(sender) = self.channels.get(key) {
            let receiver_count = sender.receiver_count();

            if receiver_count > 0 {
                sender.send(message)
                    .map_err(|e| format!("Broadcast error: {}", e))?;
            }

            Ok(receiver_count)
        } else {
            Ok(0)
        }
    }

    pub fn active_channels(&self) -> usize {
        self.channels.len()
    }

    pub fn cleanup_empty_channels(&self) {
        self.channels.retain(|_, sender| sender.receiver_count() > 0);
    }
}
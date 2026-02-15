use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

#[derive(Clone)]
pub struct Broadcaster<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    channels: Arc<RwLock<HashMap<K, broadcast::Sender<V>>>>,
    capacity: usize,
}

impl<K, V> Broadcaster<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            capacity: 100,
        }
    }

    pub async fn subscribe(&self, key: &K) -> broadcast::Receiver<V> {
        let mut channels = self.channels.write().await;

        let sender = channels
            .entry(key.clone())
            .or_insert_with(|| broadcast::channel(self.capacity).0);

        sender.subscribe()
    }

    pub async fn broadcast(&self, key: &K, message: V) -> Result<usize, String> {
        let channels = self.channels.read().await;

        if let Some(sender) = channels.get(key) {
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
}
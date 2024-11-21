use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::{mpsc, oneshot};

/// Commands that can be sent to the key-value store
#[derive(Debug, Clone, PartialEq)]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Delete { key: String },
}

/// Responses from the key-value store
#[derive(Debug, Clone, PartialEq)]
enum Response {
    Value(Option<String>),
    Set,
    Deleted,
}

/// Trait that all our implementations will share
#[async_trait]
trait KeyValueStore: Send + Sync + 'static {
    async fn get(&self, key: String) -> Option<String>;
    async fn set(&self, key: String, value: String);
    async fn delete(&self, key: String);
}

/// Test harness that all implementations must pass
async fn test_kvstore<S: KeyValueStore + Clone>(store: S) {
    // Basic set and get
    store.set("hello".to_string(), "world".to_string()).await;
    assert_eq!(
        store.get("hello".to_string()).await,
        Some("world".to_string())
    );

    // Delete
    store.delete("hello".to_string()).await;
    assert_eq!(store.get("hello".to_string()).await, None);

    // Concurrent operations
    let store2 = store.clone();
    let store3 = store.clone();

    let handle1 = tokio::spawn(async move {
        for i in 0..100 {
            store.set(format!("key{}", i), format!("value{}", i)).await;
        }
    });

    let handle2 = tokio::spawn(async move {
        for i in 0..100 {
            store2
                .set(format!("key{}", i), format!("newvalue{}", i))
                .await;
        }
    });

    let handle3 = tokio::spawn(async move {
        for i in 0..100 {
            let _value = store3.get(format!("key{}", i)).await;
        }
    });

    // Wait for all operations to complete
    let _ = tokio::join!(handle1, handle2, handle3);
}
#[derive(Clone)]
struct ChannelKeyValueStore {
    sender: mpsc::Sender<(Command, oneshot::Sender<Response>)>,
}

impl ChannelKeyValueStore {
    fn new() -> Self {
        let (tx, mut rx) = mpsc::channel::<(Command, oneshot::Sender<Response>)>(32);

        // Spawn background task to manage state
        tokio::spawn(async move {
            let mut data = HashMap::new();

            while let Some((cmd, resp)) = rx.recv().await {
                let response = match cmd {
                    Command::Get { key } => Response::Value(data.get(&key).cloned()),
                    Command::Set { key, value } => {
                        data.insert(key, value);
                        Response::Set
                    }
                    Command::Delete { key } => {
                        data.remove(&key);
                        Response::Deleted
                    }
                };
                let _ = resp.send(response);
            }
        });

        Self { sender: tx }
    }
}

#[async_trait]
impl KeyValueStore for ChannelKeyValueStore {
    async fn get(&self, key: String) -> Option<String> {
        let (tx, rx) = oneshot::channel();
        let cmd = Command::Get { key };
        self.sender.send((cmd, tx)).await.ok()?;
        match rx.await.ok()? {
            Response::Value(v) => v,
            _ => None,
        }
    }

    async fn set(&self, key: String, value: String) {
        let (tx, rx) = oneshot::channel();
        let cmd = Command::Set { key, value };
        if self.sender.send((cmd, tx)).await.is_ok() {
            let _ = rx.await;
        }
    }

    async fn delete(&self, key: String) {
        let (tx, rx) = oneshot::channel();
        let cmd = Command::Delete { key };
        if self.sender.send((cmd, tx)).await.is_ok() {
            let _ = rx.await;
        }
    }
}

#[tokio::main]
async fn main() {
    let store = ChannelKeyValueStore::new();
    test_kvstore(store).await;
}

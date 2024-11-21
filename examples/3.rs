use async_trait::async_trait;

/// Trait that all our implementations will share
#[async_trait]
trait KeyValueStore: Send + Sync + 'static {
    async fn get(&self, key: String) -> Option<String>;
    async fn set(&self, key: String, value: String);
    async fn delete(&self, key: String);
}

/// Test harness that all implementations must pass
#[tokio::test]
async fn test_kvstore<S: KeyValueStore>(store: S) {
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

#[tokio::main]
async fn main() {
    // Task: Implement a key-value store that satisfies the KeyValueStore trait
    todo!()
}

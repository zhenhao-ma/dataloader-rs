use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait BatchFn<K, V, C> {
    async fn load(&mut self, keys: &[K], context: &C) -> HashMap<K, V>
    where
        K: 'async_trait,
        V: 'async_trait,
        C: 'async_trait;
}

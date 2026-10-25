use core::fmt::Debug;
use core::hash::{BuildHasher, Hash};
use hashbrown::hash_table::Entry;
use hashbrown::{DefaultHashBuilder, HashTable};

#[derive(Clone, Debug)]
struct Item<K, V> {
    key: K,
    val: V,
    ts: u32,
    hash: u64,
}

#[derive(Clone, Debug)]
pub struct Cache<K, V> {
    hash: DefaultHashBuilder,
    map: HashTable<Item<K, V>>,
    now: u32,
    max: u32,
    target_len: usize,
}

impl<K: Hash + Eq, V> Cache<K, V> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            hash: DefaultHashBuilder::default(),
            map: HashTable::new(),
            now: 0,
            max: 2048,
            target_len: 128,
        }
    }

    pub fn fetch<E, F>(&mut self, key: K, default: F) -> Result<(), E>
    where
        F: FnOnce(&K) -> Result<V, E>,
    {
        let hash = self.hash.hash_one(&key);

        match self.map.entry(hash, |other| other.key == key, |e| e.hash) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().ts = self.now;
            }
            Entry::Vacant(entry) => {
                let val = default(&key)?;
                entry.insert(Item {
                    key,
                    val,
                    ts: self.now,
                    hash,
                });
            }
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = self.hash.hash_one(key);

        self.map
            .find(hash, |other| &other.key == key)
            .map(|e| &e.val)
    }

    pub fn tick(&mut self) {
        self.now = self.now.wrapping_add(1);

        if self.map.len() > self.target_len {
            self.map.retain(|e| self.now.wrapping_sub(e.ts) <= self.max);
        }

        self.target_len = self.target_len.midpoint(self.map.len());
    }
}

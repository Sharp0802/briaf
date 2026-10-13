use core::ops::Range;

mod slab;

use slab::Slab;

#[derive(Clone, Debug)]
struct Item<K, V> {
    range: Range<K>,
    key: Range<K>,
    val: V,
    pri: usize,
    lhs: usize,
    rhs: usize,
}

#[derive(Clone, Debug)]
pub struct Treap<K, V> {
    salt: usize,
    slab: Slab<Item<K, V>>,
    root: usize,
}

impl<K: Ord + Clone, V> Treap<K, V> {
    #[cfg(feature = "std")]
    pub fn new() -> Self {
        use core::hash::{BuildHasher, Hasher};
        use std::hash::RandomState;

        let state = RandomState::new();
        let salt = state.build_hasher().finish();

        #[allow(clippy::cast_possible_truncation)]
        Self::with_salt(salt as usize)
    }

    #[must_use]
    pub const fn with_salt(salt: usize) -> Self {
        Self {
            salt,
            slab: Slab::new(),
            root: 0,
        }
    }

    fn alloc(&mut self, key: Range<K>, val: V) -> usize {
        let id = self.slab.push(Item {
            range: key.clone(),
            key,
            val,
            pri: 0,
            lhs: usize::MAX,
            rhs: usize::MAX,
        });

        let pri = splitmix(id ^ self.salt);
        self.slab[id].pri = pri;

        id
    }
}

const fn splitmix(x: usize) -> usize {
    let x64 = x as u64;
    let z64 = splitmix64(x64);
    #[allow(clippy::cast_possible_truncation)]
    (z64 as usize)
}

const fn splitmix64(x: u64) -> u64 {
    let z = x + 0x9e37_79b9_7f4a_7c15;
    let z = (z ^ (z >> 30)) * 0xbf58_476d_1ce4_e5b9;
    let z = (z ^ (z >> 27)) * 0x94d0_49bb_1331_11eb;
    z ^ (z >> 31)
}

mod slab;

use core::ops::Range;

use slab::Slab;

pub trait Split {
    fn len(&self) -> usize;
    fn split_off(&mut self, at: usize) -> Self;
}

impl Split for String {
    fn len(&self) -> usize {
        self.len()
    }

    fn split_off(&mut self, at: usize) -> Self {
        self.split_off(at)
    }
}

impl<T> Split for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn split_off(&mut self, at: usize) -> Self {
        self.split_off(at)
    }
}

#[derive(Clone, Debug)]
struct Item<T> {
    len: usize,
    val: T,
    pri: usize,
    lhs: usize,
    rhs: usize,
}

#[derive(Clone, Debug)]
pub struct Treap<T> {
    salt: usize,
    slab: Slab<Item<T>>,
    root: usize,
}

impl<T: Split> Treap<T> {
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

    #[must_use]
    pub fn len(&self) -> usize {
        self.slab.get(self.root).map_or(0, |root| root.len)
    }

    fn alloc(&mut self, val: T) -> usize {
        let id = self.slab.push(Item {
            len: val.len(),
            val,
            pri: 0,
            lhs: usize::MAX,
            rhs: usize::MAX,
        });

        let pri = splitmix(id ^ self.salt);
        self.slab[id].pri = pri;

        id
    }

    fn update(&mut self, id: usize) {
        let Some(item) = self.slab.get(id) else {
            return;
        };

        let lhs = self.slab.get(item.lhs).map_or(0, |lhs| lhs.len);
        let rhs = self.slab.get(item.rhs).map_or(0, |rhs| rhs.len);

        self.slab[id].len = self.slab[id].val.len() + lhs + rhs;
    }

    fn split(&mut self, p: usize, key: usize) -> (usize, usize) {
        let Some(pitem) = self.slab.get(p) else {
            return (usize::MAX, usize::MAX);
        };

        let lhs = self.slab.get(pitem.lhs).map_or(0, |lhs| lhs.len);
        if key <= lhs {
            let (ll, lr) = self.split(pitem.lhs, key);
            self.slab[p].lhs = lr;
            self.update(p);
            (ll, p)
        } else if (key - lhs) <= pitem.val.len() {
            let cut = self.slab[p].val.split_off(key - lhs);
            let new = self.alloc(cut);
            self.slab[new].rhs = self.slab[p].rhs;
            self.slab[p].rhs = usize::MAX;
            self.update(new);
            self.update(p);

            (p, new)
        } else {
            let (rl, rr) = self.split(pitem.rhs, key);
            self.slab[p].rhs = rl;
            self.update(p);
            (p, rr)
        }
    }

    fn merge(&mut self, l: usize, r: usize) -> usize {
        if self.slab.get(l).is_none() {
            r
        } else if self.slab.get(r).is_none() {
            l
        } else if self.slab[l].pri > self.slab[r].pri {
            let lr = self.merge(self.slab[l].rhs, r);
            self.slab[l].rhs = lr;
            self.update(l);
            l
        } else {
            let rl = self.merge(l, self.slab[r].lhs);
            self.slab[r].lhs = rl;
            self.update(r);
            r
        }
    }

    pub fn insert(&mut self, at: usize, val: T) {
        let id = self.alloc(val);

        let (l, r) = self.split(self.root, at);
        let acc = self.merge(l, id);
        self.root = self.merge(acc, r);
    }

    fn kill_unchecked(&mut self, id: usize) {
        let Some(&Item { lhs, rhs, .. }) = self.slab.get(id) else {
            return;
        };

        self.kill_unchecked(lhs);
        self.kill_unchecked(rhs);

        self.slab.pop(id);
    }

    pub fn remove(&mut self, range: Range<usize>) {
        let (l_mid, r) = self.split(self.root, range.end);
        let (l, mid) = self.split(l_mid, range.start);
        self.kill_unchecked(mid);
        self.root = self.merge(l, r);
        self.update(self.root);
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

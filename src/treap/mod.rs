use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{self, Debug};
use core::hash::Hash;
use core::iter::FusedIterator;
use core::ops::Range;

use crate::rand::splitmix;

mod slab;

use slab::Slab;

pub trait Split {
    #[must_use]
    fn len(&self) -> usize;

    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[allow(clippy::return_self_not_must_use)]
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

impl<T> Treap<T> {
    #[cfg(feature = "std")]
    #[must_use]
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
            root: usize::MAX,
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.slab.get(self.root).map_or(0, |root| root.len)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn iter(&self) -> TreapIter<'_, T> {
        TreapIter::new(self)
    }
}

impl<T: Split> Treap<T> {
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
        let Some(item) = self.slab.get(p) else {
            return (usize::MAX, usize::MAX);
        };

        let lhs = item.lhs;
        let rhs = item.rhs;
        let val_len = item.val.len();

        let lhs_len = self.slab.get(lhs).map_or(0, |item| item.len);

        if key <= lhs_len {
            let (ll, lr) = self.split(lhs, key);

            self.slab[p].lhs = usize::MAX;
            self.update(p);

            (ll, self.merge(lr, p))
        } else if key - lhs_len < val_len {
            let cut = self.slab[p].val.split_off(key - lhs_len);

            self.slab[p].rhs = usize::MAX;
            self.update(p);

            let new = self.alloc(cut);

            (p, self.merge(new, rhs))
        } else {
            let (rl, rr) = self.split(rhs, key - lhs_len - val_len);

            self.slab[p].rhs = usize::MAX;
            self.update(p);

            (self.merge(p, rl), rr)
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
        let (l, r) = self.split(self.root, at);
        let id = self.alloc(val);

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

    #[must_use]
    pub fn slice_mut(&mut self, range: Range<usize>) -> TreapSliceMut<'_, T> {
        assert!(range.start <= range.end);
        assert!(range.end <= self.len());

        let (l_mid, r) = self.split(self.root, range.end);
        let (l, mid) = self.split(l_mid, range.start);
        self.root = mid;

        TreapSliceMut::new(self, l, r)
    }
}

#[cfg(feature = "std")]
impl<T> Default for Treap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq> PartialEq for Treap<T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T: Eq> Eq for Treap<T> {}

impl<T: Hash> Hash for Treap<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        for item in self {
            item.hash(state);
        }
    }
}

impl<'a, T> IntoIterator for &'a Treap<T> {
    type Item = &'a T;
    type IntoIter = TreapIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone, Debug)]
pub struct TreapIter<'a, T> {
    treap: &'a Treap<T>,
    front: Vec<usize>,
    remaining: usize,
}

impl<'a, T> TreapIter<'a, T> {
    fn new(treap: &'a Treap<T>) -> Self {
        let mut iter = Self {
            treap,
            front: Vec::new(),
            remaining: treap.slab.len(),
        };

        iter.push_left(treap.root);

        iter
    }

    fn push_left(&mut self, mut id: usize) {
        while let Some(item) = self.treap.slab.get(id) {
            self.front.push(id);
            id = item.lhs;
        }
    }
}

impl<'a, T> Iterator for TreapIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.front.pop()?;

        debug_assert!(self.remaining >= 1);
        self.push_left(self.treap.slab[id].rhs);
        self.remaining -= 1;

        Some(&self.treap.slab[id].val)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T> FusedIterator for TreapIter<'_, T> {}

impl<T> ExactSizeIterator for TreapIter<'_, T> {
    fn len(&self) -> usize {
        self.remaining
    }
}

pub struct TreapSliceMut<'a, T: Split> {
    treap: &'a mut Treap<T>,
    front: Vec<&'a mut Item<T>>,
    lhs: usize,
    rhs: usize,
}

impl<'a, T: Split> TreapSliceMut<'a, T> {
    fn new(treap: &'a mut Treap<T>, lhs: usize, rhs: usize) -> Self {
        let mut this = Self {
            treap,
            front: Vec::new(),
            lhs,
            rhs,
        };

        let root = this.treap.root;
        this.push_left(root);

        this
    }

    fn push_left(&mut self, mut id: usize) {
        while let Some(item) = self.treap.slab.get_mut_ptr(id) {
            // SAFETY: The exclusive treap borrow keeps the slab alive and fixed
            // for 'a. Each node is visited once, including nodes held in front.
            #[expect(unsafe_code)]
            let item = unsafe { &mut *item };

            id = item.lhs;
            self.front.push(item);
        }
    }
}

impl<T: Split + Debug> Debug for TreapSliceMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: Formatting the treap would borrow values already yielded.
        f.debug_struct("TreapSliceMut")
            .field("front", &self.front)
            .field("lhs", &self.lhs)
            .field("rhs", &self.rhs)
            .finish_non_exhaustive()
    }
}

impl<'a, T: Split> Iterator for TreapSliceMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.front.pop()?;
        self.push_left(item.rhs);
        Some(&mut item.val)
    }
}

impl<T: Split> FusedIterator for TreapSliceMut<'_, T> {}

impl<T: Split> Drop for TreapSliceMut<'_, T> {
    fn drop(&mut self) {
        let mid = self.treap.root;
        let l_mid = self.treap.merge(self.lhs, mid);
        self.treap.root = self.treap.merge(l_mid, self.rhs);
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    #[derive(Clone, Debug)]
    enum Op {
        Insert(usize, String),
        Remove(usize, usize),
        Slice(usize, usize),
    }

    impl Op {
        fn apply(&self, mock: &mut String, treap: &mut Treap<String>) {
            match self {
                Self::Insert(off, str) => {
                    let off = off % (mock.len() + 1);
                    mock.insert_str(off, str);
                    treap.insert(off, str.clone());
                }
                Self::Remove(off, len) => {
                    let off = off % (mock.len() + 1);
                    let max = mock.len() - off;
                    let len = *len % (max + 1);

                    let range = off..(off + len);
                    mock.drain(range.clone());
                    treap.remove(range);
                }
                Self::Slice(off, len) => {
                    let off = off % (mock.len() + 1);
                    let max = mock.len() - off;
                    let len = *len % (max + 1);

                    let range = off..(off + len);

                    let mut buffer = String::with_capacity(range.len());
                    for item in treap.slice_mut(range.clone()) {
                        buffer.push_str(item);
                    }

                    assert_eq!(&buffer, &mock[range]);
                }
            }

            let mut buffer = String::with_capacity(treap.len());
            for item in treap.iter() {
                buffer.push_str(item);
            }

            assert_eq!(mock.as_str(), buffer.as_str());
        }
    }

    fn arb_op() -> impl Strategy<Value = Op> {
        prop_oneof![
            (any::<usize>(), "[a-zA-Z0-9]{1,16}").prop_map(|(off, str)| Op::Insert(off, str)),
            (any::<usize>(), 1..16usize).prop_map(|(off, len)| Op::Remove(off, len)),
            (any::<usize>(), any::<usize>()).prop_map(|(off, len)| Op::Slice(off, len)),
        ]
        .boxed()
    }

    proptest! {
        #[test]
        fn test_models(ops in prop::collection::vec(arb_op(), 1..2048)) {
            let mut mock = String::new();
            let mut treap = Treap::default();

            for op in ops {
                op.apply(&mut mock, &mut treap);
            }
        }
    }
}

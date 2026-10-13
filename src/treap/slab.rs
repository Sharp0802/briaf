use alloc::vec::Vec;
use core::mem::replace;
use core::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
struct Slot<T> {
    next: usize,
    value: Option<T>,
}

#[derive(Clone, Debug)]
pub struct Slab<T> {
    vec: Vec<Slot<T>>,
    free: usize,
}

impl<T> Slab<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            vec: Vec::new(),
            free: 0,
        }
    }

    pub fn push(&mut self, value: T) -> usize {
        if let Some(slot) = self.vec.get_mut(self.free) {
            #[allow(unused_variables)]
            let old = slot.value.replace(value);
            debug_assert!(old.is_none());

            replace(&mut self.free, slot.next)
        } else {
            debug_assert_eq!(self.free, self.vec.len());

            let next = self.free + 1;
            self.vec.push(Slot {
                next,
                value: Some(value),
            });

            replace(&mut self.free, next)
        }
    }

    pub fn pop(&mut self, key: usize) -> Option<T> {
        let slot = self.vec.get_mut(key)?;
        let old = slot.value.take();

        if old.is_some() {
            slot.next = self.free;
            self.free = key;
        }

        old
    }

    pub fn get(&self, key: usize) -> Option<&T> {
        self.vec.get(key)?.value.as_ref()
    }

    pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        self.vec.get_mut(key)?.value.as_mut()
    }
}

impl<T> Index<usize> for Slab<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index out of range")
    }
}

impl<T> IndexMut<usize> for Slab<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("index out of range")
    }
}

#[cfg(test)]
mod tests {
    use hashbrown::HashMap;
    use proptest::prelude::*;

    use super::*;

    #[derive(Clone, Copy, Debug)]
    enum Op {
        Push(u64),
        Pop(usize),
        Get(usize),
    }

    impl Op {
        fn apply(self, mock: &mut HashMap<usize, u64>, slab: &mut Slab<u64>) {
            match self {
                Self::Push(value) => {
                    let key = slab.push(value);
                    let old = mock.insert(key, value);
                    assert_eq!(old, None);
                }
                Self::Pop(key) => {
                    let key = key % (slab.vec.len() + 1);

                    let old_a = slab.pop(key);
                    let old_b = mock.remove(&key);
                    assert_eq!(old_a, old_b);
                }
                Self::Get(key) => {
                    let key = key % (slab.vec.len() + 1);

                    let old_a = slab.get(key);
                    let old_b = mock.get(&key);
                    assert_eq!(old_a, old_b);

                    let old_a = slab.get_mut(key);
                    let old_b = mock.get_mut(&key);
                    assert_eq!(old_a, old_b);
                }
            }
        }
    }

    fn arb_op() -> impl Strategy<Value = Op> {
        prop_oneof![
            any::<u64>().prop_map(Op::Push),
            any::<usize>().prop_map(Op::Pop),
            any::<usize>().prop_map(Op::Get),
        ]
        .boxed()
    }

    proptest! {
        #[test]
        fn test_models(ops in prop::collection::vec(arb_op(), 1..4096)) {
            let mut mock = HashMap::new();
            let mut slab = Slab::<u64>::new();

            for op in ops {
                op.apply(&mut mock, &mut slab);
            }
        }
    }
}

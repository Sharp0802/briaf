use alloc::string::String;
use alloc::vec::Vec;
use core::hash::Hash;
use core::ops::Deref;
use hashbrown::Equivalent;

use crate::{FontName, Tilt};

/// A hint for querying fonts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Hint {
    /// Font weight.
    pub weight: Option<u16>,
    /// Font tilting.
    pub tilt: Option<Tilt>,
}

/// A query of the font chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Query<'a> {
    /// Font names.
    pub names: &'a [FontName<'a>],
    /// Query hint.
    pub hint: Hint,
}

impl Query<'_> {
    pub(crate) fn to_owned(self) -> OwnedQuery {
        OwnedQuery {
            names: self.names.iter().copied().map(Into::into).collect(),
            hint: self.hint,
        }
    }
}

impl Hash for Query<'_> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        for name in self.names {
            name.deref().hash(state);
        }

        self.hint.hash(state);
    }
}

impl Equivalent<OwnedQuery> for Query<'_> {
    fn equivalent(&self, key: &OwnedQuery) -> bool {
        self.hint == key.hint
            && self
                .names
                .iter()
                .map(Deref::deref)
                .eq(key.names.iter().map(String::as_str))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedQuery {
    pub names: Vec<String>,
    pub hint: Hint,
}

impl Hash for OwnedQuery {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        for name in &self.names {
            name.as_str().hash(state);
        }

        self.hint.hash(state);
    }
}

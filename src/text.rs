use core::ops::Range;

use crate::{BlockStyle, TextStyle, Treap};

#[derive(Clone, Debug)]
struct Part {
    style: TextStyle,
    range: Range<usize>,
}

/// A styled text block.
#[derive(Clone, Debug)]
pub struct TextBlock {
    style: BlockStyle,
    source: String,
    parts: Treap<usize, Part>,
}

impl TextBlock {
    /// Creates a new [`TextBlock`].
    #[must_use]
    #[cfg(feature = "std")]
    pub fn new(style: BlockStyle) -> Self {
        let mut this = Self::with_salt(style, 0);
        this.parts = Treap::new();
        this
    }

    /// Creates a new [`TextBlock`] with given salt value.
    #[must_use]
    pub const fn with_salt(style: BlockStyle, salt: usize) -> Self {
        Self {
            style,
            source: String::new(),
            parts: Treap::with_salt(salt),
        }
    }
}

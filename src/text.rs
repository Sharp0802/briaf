use core::ops::Range;

use crate::treap::Split;
use crate::{BlockStyle, Span, TextStyle, Treap};

#[derive(Clone, Debug)]
struct Part {
    style: TextStyle,
    range: Range<usize>,
}

impl Split for Part {
    fn len(&self) -> usize {
        self.range.len()
    }

    fn split_off(&mut self, at: usize) -> Self {
        assert!(self.range.start <= at && at <= self.range.end);

        let end = self.range.end;
        self.range.end = at;

        Self {
            style: self.style.clone(),
            range: at..end,
        }
    }
}

/// A styled text block.
#[derive(Clone, Debug)]
pub struct TextBlock {
    style: BlockStyle,
    source: Span,
    parts: Treap<Part>,
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
            source: Span::new(),
            parts: Treap::with_salt(salt),
        }
    }
}

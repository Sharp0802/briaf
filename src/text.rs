use core::ops::Range;

use crate::{BlockStyle, Glyph, GlyphCache, Split, TextStyle, Treap};

/// A text element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Text<'a> {
    /// Text style.
    pub style: &'a TextStyle,
    /// Text content.
    pub str: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Span {
    style: TextStyle,
    range: Range<usize>,
}

impl Split for Span {
    fn len(&self) -> usize {
        self.range.len()
    }

    fn split_off(&mut self, at: usize) -> Self {
        assert!(at <= self.len());

        let end = self.range.end;
        self.range.end = at;

        Self {
            style: self.style.clone(),
            range: at..end,
        }
    }
}

/// A styled text block.
#[derive(Debug)]
pub struct TextBlock {
    style: BlockStyle,
    buffer: String,
    spans: Treap<Span>,
}

impl TextBlock {
    /// Creates a new [`TextBlock`].
    #[must_use]
    #[cfg(feature = "std")]
    pub fn new(style: BlockStyle) -> Self {
        let mut this = Self::with_salt(style, 0);
        this.spans = Treap::new();
        this
    }

    /// Creates a new [`TextBlock`] with given salt value.
    #[must_use]
    pub const fn with_salt(style: BlockStyle, salt: usize) -> Self {
        Self {
            style: style.norm(),
            buffer: String::new(),
            spans: Treap::with_salt(salt),
        }
    }

    fn is_grapheme_boundary(&self, at: usize) -> bool {
        todo!()
    }

    /// Inserts given text at the specified byte offset.
    ///
    /// # Panics
    ///
    /// Panics if `at` is not valid grapheme boundary.
    pub fn insert(&mut self, at: usize, text: &Text) {
        assert!(self.is_grapheme_boundary(at));

        let start = self.buffer.len();
        self.buffer.push_str(text.str);
        let end = self.buffer.len();

        let span = Span {
            style: text.style.clone().norm(),
            range: start..end,
        };

        self.spans.insert(at, span);
    }

    /// Appends given text at the end.
    pub fn push(&mut self, text: &Text) {
        self.insert(self.spans.len(), text);
    }

    /// Removes text within the specified byte range.
    ///
    /// # Panics
    ///
    /// Panics if any end of specified range is not valid grapheme boundary.
    pub fn remove(&mut self, range: Range<usize>) {
        assert!(self.is_grapheme_boundary(range.start));
        assert!(self.is_grapheme_boundary(range.end));

        self.spans.remove(range);
    }

    /// Modifies the text style within the specified byte range.
    ///
    /// # Panics
    ///
    /// Panics if any end of specified range is not valid grapheme boundary.
    pub fn style<F>(&mut self, range: Range<usize>, style: F)
    where
        F: Fn(&mut TextStyle),
    {
        assert!(self.is_grapheme_boundary(range.start));
        assert!(self.is_grapheme_boundary(range.end));

        for span in self.spans.slice_mut(range) {
            style(&mut span.style);
        }
    }

    /// Returns an iterator over shaped glyphs of `self`.
    #[must_use]
    pub const fn shapes<'a, C>(&'a mut self, cache: &'a C) -> GlyphIter<'a, C>
    where
        C: GlyphCache,
    {
        GlyphIter::new(self, cache)
    }
}

/// An iterator over shaped glyphs.
#[derive(Debug)]
pub struct GlyphIter<'a, C> {
    block: &'a mut TextBlock,
    cache: &'a C,
}

impl<'a, C> GlyphIter<'a, C> {
    const fn new(block: &'a mut TextBlock, cache: &'a C) -> Self {
        Self { block, cache }
    }
}

impl<'a, C> Iterator for GlyphIter<'a, C>
where
    C: GlyphCache,
{
    type Item = &'a Glyph<C::Data>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

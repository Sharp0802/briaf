use core::ops::Range;

use crate::treap::Split;
use crate::{BlockStyle, TextStyle, Treap};

/// A text element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Text<'a> {
    /// Text style.
    pub style: TextStyle,
    /// Text content.
    pub str: &'a str,
}

/// A shaped glyph.
#[derive(Clone, Debug)]
pub struct Glyph<'a> {
    /// Position from origin, in pixels, in Y-up coordinate system.
    pub base: [f32; 2],
    /// Bitmap size, in pixels.
    pub size: [u32; 2],
    /// Origin advancement, in pixels, in Y-up coordinate system.
    pub advance: [f32; 2],
    /// Bitmap image, in `RGBA8Unorm` format.
    pub bitmap: &'a [u8],
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct OwnedText {
    style: TextStyle,
    string: String,
}

impl Split for OwnedText {
    fn len(&self) -> usize {
        self.string.len()
    }

    fn split_off(&mut self, at: usize) -> Self {
        assert!(at <= self.string.len());

        Self {
            style: self.style.clone(),
            string: self.string.split_off(at),
        }
    }
}

/// A styled text block.
#[derive(Clone, Debug)]
pub struct TextBlock {
    style: BlockStyle,
    parts: Treap<OwnedText>,
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
    pub fn with_salt(style: BlockStyle, salt: usize) -> Self {
        Self {
            style: style.norm(),
            parts: Treap::with_salt(salt),
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
        let text = Text {
            style: text.style.clone().norm(),
            str: text.str,
        };

        assert!(self.is_grapheme_boundary(at));
        todo!()
    }

    /// Appends given text at the end.
    pub fn push(&mut self, text: &Text) {
        let text = Text {
            style: text.style.clone().norm(),
            str: text.str,
        };

        todo!()
    }

    /// Removes text within the specified byte range.
    ///
    /// # Panics
    ///
    /// Panics if any end of specified range is not valid grapheme boundary.
    pub fn remove(&mut self, range: Range<usize>) {
        assert!(self.is_grapheme_boundary(range.start));
        assert!(self.is_grapheme_boundary(range.end));
        todo!()
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
        todo!()
    }

    /// Returns an iterator over shaped glyphs of `self`.
    #[must_use]
    pub const fn shapes(&mut self) -> GlyphIter<'_> {
        GlyphIter::new(self)
    }
}

/// An iterator over shaped glyphs.
#[derive(Debug)]
pub struct GlyphIter<'a> {
    block: &'a mut TextBlock,
}

impl<'a> GlyphIter<'a> {
    const fn new(block: &'a mut TextBlock) -> Self {
        Self { block }
    }
}

impl<'a> Iterator for GlyphIter<'a> {
    type Item = Glyph<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

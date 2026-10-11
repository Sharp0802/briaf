use core::ops::Range;

use crate::{SpanStyle, TextStyle};

#[derive(Debug)]
pub struct Span {
    style: SpanStyle,
}

impl Span {
    /// Creates a new [`Span`].
    #[must_use]
    pub fn new(style: SpanStyle) -> Self {
        Self { style }
    }

    /// Returns the length, in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Returns whether the length is zero.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Appends given string at last.
    pub fn push_str(&mut self, str: &str) {
        todo!()
    }

    fn is_glyph_boundary(&self, index: usize) -> bool {
        todo!()
    }

    fn split_off(&mut self, index: usize) -> Self {
        assert!(index <= self.len());
        assert!(self.is_glyph_boundary(index));

        todo!()
    }
}

#[derive(Debug)]
pub struct Text {
    style: TextStyle,
}

impl Text {
    /// Creates a new [`Text`].
    #[must_use]
    pub fn new(style: TextStyle) -> Self {
        Self { style }
    }

    /// Returns the length, in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Returns whether the length is zero.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn is_glyph_boundary(&self, index: usize) -> bool {
        todo!()
    }

    /// Inserts given span at specified byte index.
    ///
    /// # Panics
    ///
    /// Panics if any of the following conditions are met:
    ///
    /// - `index` is out of `0..=self.len()`.
    /// - `index` is not valid glyph boundary.
    pub fn insert(&mut self, index: usize, span: Span) {
        assert!(index <= self.len());
        assert!(self.is_glyph_boundary(index));

        if index == self.len() {
            self.append(span);
            return;
        }

        todo!()
    }

    /// Appends given span at last.
    ///
    /// It's same to [`insert`] with using index as `self.len()`.
    pub fn append(&mut self, span: Span) {
        todo!()
    }

    #[track_caller]
    fn assert_range(&self, range: Range<usize>) {
        assert!(range.start <= range.end);
        assert!(range.end <= self.len());
        assert!(self.is_glyph_boundary(range.start));
        assert!(self.is_glyph_boundary(range.end));
    }

    /// Removes given byte range.
    ///
    /// # Panics
    ///
    /// Panics if any of the following conditions are met:
    ///
    /// - The start of range is larger than the end of range.
    /// - The end of range is out of `..=self.len()`.
    /// - The end or start of range is not valid glyph boundary.
    pub fn remove(&mut self, range: Range<usize>) {
        self.assert_range(range);

        todo!()
    }

    /// Style the span on given byte range.
    ///
    /// # Panics
    ///
    /// Panics if any of the following conditions are met:
    ///
    /// - The start of range is larger than the end of range.
    /// - The end of range is out of `..=self.len()`.
    /// - The end or start of range is not valid glyph boundary.
    pub fn style<F>(&mut self, range: Range<usize>, patch: F)
    where
        F: Fn(&mut SpanStyle),
    {
        self.assert_range(range);

        todo!()
    }
}

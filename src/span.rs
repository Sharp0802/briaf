use core::iter::FusedIterator;
use core::ops::Index;
use unicode_segmentation::UnicodeSegmentation;

/// A text span.
#[derive(Debug)]
pub struct Span {
    string: String,
    graphemes: Vec<usize>,
}

impl Span {
    /// Creates a new [`Span`].
    #[must_use]
    pub const fn new() -> Self {
        Self {
            string: String::new(),
            graphemes: Vec::new(),
        }
    }

    /// Returns grapheme clusters of the string.
    #[must_use]
    pub const fn graphemes(&self) -> Graphemes<'_> {
        Graphemes {
            str: self.string.as_str(),
            graphemes: self.graphemes.as_slice(),
        }
    }

    /// Returns the reference of underlying string.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        self.string.as_str()
    }

    /// Reserves at least requested amount of bytes more than current length.
    pub fn reserve(&mut self, additional: usize) {
        self.string.reserve(additional);
        self.graphemes.reserve(additional);
    }

    /// Appends a given string slice onto the end.
    pub fn push_str(&mut self, str: &str) {
        if str.is_empty() {
            return;
        }

        self.string.push_str(str);

        let done = self.graphemes.last().copied().unwrap_or(0);
        let target = &self.string[done..];

        self.graphemes.reserve(str.len());
        for (i, grapheme) in target.grapheme_indices(true) {
            self.graphemes.push(done + i + grapheme.len());
        }
    }
}

impl Default for Span {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}

impl<T: AsRef<str>> From<T> for Span {
    fn from(value: T) -> Self {
        let mut this = Self::new();
        this.push_str(value.as_ref());
        this
    }
}

/// A grapheme clusters of the span.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Graphemes<'a> {
    str: &'a str,
    graphemes: &'a [usize],
}

impl<'a> Graphemes<'a> {
    /// Returns the number of grapheme clusters.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.graphemes.len()
    }

    /// Returns `true` if and only if there is no grapheme clusters.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn find(&self, idx: usize) -> Option<usize> {
        self.graphemes.binary_search(&idx).ok()
    }

    /// Returns a reference to a n-th grapheme cluster,
    /// returning `None` if the given index is out of range.
    #[must_use]
    pub const fn get(&self, idx: usize) -> Option<&'a str> {
        if idx >= self.graphemes.len() {
            return None;
        }

        let curr = self.graphemes[idx];
        let prev = match idx.checked_sub(1) {
            Some(prev) => self.graphemes[prev],
            None => 0,
        };

        let cut = self.str.split_at(curr).0.split_at(prev).1;
        Some(cut)
    }

    /// Returns an iterator over grapheme clusters.
    #[must_use]
    pub const fn iter(&self) -> GraphemesIter<'_> {
        GraphemesIter::new(self)
    }
}

impl Index<usize> for Graphemes<'_> {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index out of range")
    }
}

impl<'a> IntoIterator for &'a Graphemes<'a> {
    type Item = &'a str;
    type IntoIter = GraphemesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GraphemesIter::new(self)
    }
}

/// An iterator over grapheme clusters.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphemesIter<'a> {
    start: usize,
    end: usize,
    graphemes: &'a Graphemes<'a>,
}

impl<'a> GraphemesIter<'a> {
    const fn new(graphemes: &'a Graphemes<'a>) -> Self {
        Self {
            start: 0,
            end: graphemes.len(),
            graphemes,
        }
    }
}

impl<'a> Iterator for GraphemesIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }

        let grapheme = self.graphemes.get(self.start)?;
        self.start += 1;
        Some(grapheme)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl DoubleEndedIterator for GraphemesIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }

        let grapheme = self.graphemes.get(self.end - 1)?;
        self.end -= 1;
        Some(grapheme)
    }
}

impl FusedIterator for GraphemesIter<'_> {}

impl ExactSizeIterator for GraphemesIter<'_> {
    fn len(&self) -> usize {
        self.end - self.start
    }
}

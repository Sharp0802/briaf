use core::iter::FusedIterator;
use core::ops::Index;
use harfrust::{Buffer, ContentType, Direction};
use unicode_segmentation::UnicodeSegmentation;

/// A text span.
#[derive(Debug)]
pub struct Span {
    buffer: Buffer,
    string: String,
    graphemes: Vec<usize>,
}

impl Span {
    /// Creates a new [`Span`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
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
        self.string.push_str(str);

        let done = self.graphemes.last().copied().unwrap_or(0);
        let target = &self.string[done..];

        self.graphemes.reserve(str.len());
        for (i, grapheme) in target.grapheme_indices(true) {
            self.graphemes.push(done + i + grapheme.len());
        }
    }

    /// Splits the span into two at given byte offset,
    /// returning `None` if `at` is not valid grapheme boundary.
    ///
    /// Returns a newly allocated [`Span`].
    /// `self` contains `[0, at)`, and returned [`Span`] contains `[at, len)`.
    pub fn split_off_checked(&mut self, at: usize) -> Option<Self> {
        if at == 0 {
            Some(core::mem::take(self))
        } else if at == self.as_str().len() {
            Some(Self::new())
        } else if let Some(idx) = self.graphemes().find(at) {
            let next_string = self.string.split_off(at);
            let next_graphs = self.graphemes.split_off(idx + 1);

            let buffer = split_unchecked(&mut self.buffer, self.string.len(), at);

            Some(Self {
                buffer,
                string: next_string,
                graphemes: next_graphs,
            })
        } else {
            None
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

fn split_unchecked(lhs: &mut Buffer, len: usize, at: usize) -> Buffer {
    let at32 = u32::try_from(at).expect("offset too large");

    let is_shaped = lhs.content_type() == Some(ContentType::Glyphs);
    let is_backward = is_shaped
        && matches!(
            lhs.direction(),
            Direction::RightToLeft | Direction::BottomToTop
        );

    let infos = lhs.glyph_infos();
    let n = infos.len();

    let cut = if at == 0 {
        if is_backward { n } else { 0 }
    } else if at == len {
        if is_backward { 0 } else { n }
    } else if is_backward {
        let i = infos
            .iter()
            .position(|g| g.cluster < at32)
            .expect("boundary unavailable");

        assert!(i == 0 || infos[i - 1].cluster != at32);

        i
    } else {
        let i = infos
            .iter()
            .position(|g| g.cluster >= at32)
            .expect("boundary unavailable");

        assert!(infos[i].cluster == at32);

        i
    };

    let mut rhs = Buffer::new();
    rhs.set_direction(lhs.direction());
    rhs.set_script(lhs.script());
    rhs.set_language(lhs.language().cloned());
    rhs.set_cluster_level(lhs.cluster_level());
    rhs.set_flags(lhs.flags());
    rhs.set_content_type(lhs.content_type());

    assert!(rhs.push_glyph_infos(&lhs.glyph_infos()[cut..]));

    if is_shaped {
        rhs.glyph_positions_mut()
            .copy_from_slice(&lhs.glyph_positions()[cut..]);
    }

    assert!(lhs.set_length(cut));

    rhs
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

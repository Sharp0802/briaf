use core::hash::Hash;

use crate::{FontChainId, Fract};

/// A style of the text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextStyle {
    /// Maximum width of the paragraph, in pixels.
    ///
    /// Ignored if it is zero or negative.
    pub width: Option<Fract>,
    /// Text alignment.
    ///
    /// Ignored if `width` is `None`.
    pub align: Align,
    /// Whether the full justification is used.
    ///
    /// Ignored if `width` is `None`.
    pub strech: bool,
    /// Wrapping method for overflows.
    ///
    /// Ignored if `width` is `None`.
    pub wrap: Wrap,
    /// Extra line spacing, in pixels.
    ///
    /// Ignored if it has negative value.
    pub line_spacing: Fract,
}

impl TextStyle {
    pub(crate) fn norm(self) -> Self {
        let width = match self.width {
            Some(v) if v > Fract::ZERO => Some(v),
            _ => None,
        };

        let line_spacing = self.line_spacing.max(0.0);

        if width.is_none() {
            Self {
                width,
                line_spacing,
                ..Default::default()
            }
        } else {
            Self {
                width,
                line_spacing,
                ..self
            }
        }
    }
}

/// An alignment of the text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Align {
    /// Follows the default alignment of the language.
    #[default]
    Default,
    /// Aligns to the left.
    Left,
    /// Aligns to the right.
    Right,
    /// Aligns to the middle.
    Middle,
}

/// A text wrapping method.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Wrap {
    /// Ignores overflows.
    No,
    /// Moves whole words to next lines.
    ///
    /// It breaks the word if a word is wider than maximum width.
    #[default]
    Word,
    /// Inserts breaks between letters.
    ///
    /// It overflows if a letter is wider than maximum width.
    Letter,
}

/// A style of the span.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SpanStyle {
    /// Font chain.
    pub font: Option<FontChainId>,
    /// Font size, in pixels, clamped to `1..` if out of that.
    pub size: Fract,
    /// Font weight, clamped to `1..1000` if out of that.
    pub weight: Fract,
    /// Text tilting.
    pub tilt: Option<Tilt>,
    /// Foreground color for monochrome glyphs,
    /// clamped to `0..=1` for each components out of that.
    pub color: Color,
    /// Extra letter spacing, in pixels, clamped to `0..` if out of that.
    pub spacing: Fract,
}

impl SpanStyle {
    pub(crate) const fn norm(self) -> Self {
        Self {
            font: self.font,
            size: self.size.max(1.0),
            weight: self.weight.clamp(1.0, 1000.0),
            tilt: match self.tilt {
                Some(tilt) => Some(tilt.norm()),
                None => None,
            },
            color: self.color.norm(),
            spacing: self.spacing.max(0.0),
        }
    }
}

/// A tilting of the text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tilt {
    /// An italic.
    ///
    /// `Oblique(-14.0)` is used instead,
    /// if the font has no `ital` support
    /// but has `slnt` support.
    Italic,
    /// An oblique, clamped to `-90..90` if out of that.
    ///
    /// `Italic` is used instead,
    /// if the font has no `slnt` support
    /// but has `ital` support.
    Oblique(Fract),
}

impl Tilt {
    const fn norm(self) -> Self {
        match self {
            Self::Italic => Self::Italic,
            Self::Oblique(v) => Self::Oblique(v.clamp(-90.0, 90.0)),
        }
    }
}

/// A straight RGBA color.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
pub struct Color {
    /// A red channel of the color.
    pub r: f32,
    /// A green channel of the color.
    pub g: f32,
    /// A blue channel of the color.
    pub b: f32,
    /// An alpha channel of the color.
    pub a: f32,
}

impl Color {
    /// Creates a new opaque [`Color`] from its components.
    #[must_use]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// Creates a new [`Color`] from its components.
    #[must_use]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    const fn norm(self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            a: self.a.clamp(0.0, 1.0),
        }
    }

    /// An opaque black.
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r.to_bits() == other.r.to_bits()
            && self.g.to_bits() == other.g.to_bits()
            && self.b.to_bits() == other.b.to_bits()
            && self.a.to_bits() == other.a.to_bits()
    }
}

impl Eq for Color {}

impl Hash for Color {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.r.to_bits().hash(state);
        self.g.to_bits().hash(state);
        self.b.to_bits().hash(state);
        self.a.to_bits().hash(state);
    }
}

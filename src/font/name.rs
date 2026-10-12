use alloc::string::String;
use core::ops::Deref;
use fontdb::Family as Family2;

/// A kind of special font families.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Family {
    /// A `serif` font family.
    Serif,
    /// A `sans-serif` font family.
    SansSerif,
    /// A `cursive` font family.
    Cursive,
    /// A `fantasy` font family.
    Fantasy,
    /// A `monospace` font family.
    Monospace,
}

/// A name of the font family.
///
/// The name `serif`, `sans-serif`, `cursive`,
/// `fantasy`, and `monospace` are reserved for
/// special font families (See [`Family`]).
///
/// Those families can be set using [`Fonts::with`](super::Fonts::with).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FontName<'a>(Family2<'a>);

impl Deref for FontName<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self.0 {
            Family2::Name(other) => other,
            Family2::Serif => "serif",
            Family2::SansSerif => "sans-serif",
            Family2::Cursive => "cursive",
            Family2::Fantasy => "fantasy",
            Family2::Monospace => "monospace",
        }
    }
}

impl<'a> From<&'a str> for FontName<'a> {
    fn from(value: &'a str) -> Self {
        Self(match value {
            "serif" => Family2::Serif,
            "sans-serif" => Family2::SansSerif,
            "cursive" => Family2::Cursive,
            "fantasy" => Family2::Fantasy,
            "monospace" => Family2::Monospace,
            other => Family2::Name(other),
        })
    }
}

impl<'a> From<FontName<'a>> for Family2<'a> {
    fn from(value: FontName<'a>) -> Self {
        value.0
    }
}

impl<'a> From<FontName<'a>> for String {
    fn from(value: FontName<'a>) -> Self {
        value.deref().into()
    }
}

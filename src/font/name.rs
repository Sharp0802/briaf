use core::ops::Deref;
use fontdb::Family;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FontName<'a>(Family<'a>);

impl Deref for FontName<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self.0 {
            Family::Name(other) => other,
            Family::Serif => "serif",
            Family::SansSerif => "sans-serif",
            Family::Cursive => "cursive",
            Family::Fantasy => "fantasy",
            Family::Monospace => "monospace",
        }
    }
}

impl<'a> From<&'a str> for FontName<'a> {
    fn from(value: &'a str) -> Self {
        Self(match value {
            "serif" => Family::Serif,
            "sans-serif" => Family::SansSerif,
            "cursive" => Family::Cursive,
            "fantasy" => Family::Fantasy,
            "monospace" => Family::Monospace,
            other => Family::Name(other),
        })
    }
}

impl<'a> From<FontName<'a>> for Family<'a> {
    fn from(value: FontName<'a>) -> Self {
        value.0
    }
}

impl<'a> From<FontName<'a>> for String {
    fn from(value: FontName<'a>) -> Self {
        value.deref().into()
    }
}

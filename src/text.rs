use crate::{Span, TextStyle};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Text {
    style: TextStyle,
    span: Span,
}

impl Text {
    #[must_use]
    pub const fn new(style: TextStyle) -> Self {
        Self {
            style,
            span: Span::new(),
        }
    }

    pub fn split_off_checked(&mut self, at: usize) -> Option<Self> {
        let span = self.span.split_off_checked(at)?;

        Some(Self {
            style: self.style.clone(),
            span,
        })
    }
}

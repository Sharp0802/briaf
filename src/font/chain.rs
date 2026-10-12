use alloc::sync::Arc;
use alloc::vec::Vec;
use fontdb::{Database, ID, Style, Weight};
use skrifa::{FontRef, MetadataProvider};

use crate::{Fonts, Query, Tilt};

/// A font chain.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FontChain(Arc<[ID]>);

impl FontChain {
    pub(crate) fn resolve(db: &Database, query: &Query) -> Self {
        let mut chain = Vec::with_capacity(query.names.len());

        for &name in query.names {
            let Some(id) = db.query(&fontdb::Query {
                families: &[name.into()],
                weight: query.hint.weight.map_or(Weight::NORMAL, Weight),
                style: query.hint.tilt.map_or(Style::Normal, |tilt| match tilt {
                    Tilt::Italic => Style::Italic,
                    Tilt::Oblique(_) => Style::Oblique,
                }),
                ..Default::default()
            }) else {
                continue;
            };

            chain.push(id);
        }

        Self(chain.into_boxed_slice().into())
    }

    pub(crate) fn resolve_for(&self, fonts: &Fonts, grapheme: &str) -> Option<ID> {
        self.0.as_ref().iter().copied().find(|&id| {
            fonts
                .db
                .with_face_data(id, |src, idx| {
                    FontRef::from_index(src, idx).is_ok_and(|font| {
                        grapheme
                            .chars()
                            .filter(|&ch| req_glyph(ch))
                            .all(|ch| font.charmap().map(ch).is_some())
                    })
                })
                .unwrap_or(false)
        })
    }
}

fn req_glyph(ch: char) -> bool {
    match ch {
        ch if ch.is_control() => false,
        '\u{200C}' | '\u{200D}'           // ZWNJ, ZWJ
            | '\u{FE00}'..='\u{FE0F}'    // Variation selectors
            | '\u{E0100}'..='\u{E01EF}'  // Supplementary VS
            | '\u{E0020}'..='\u{E007F}'  // Emoji tags
            => false,
        _ => true
    }
}

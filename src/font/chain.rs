use alloc::sync::Arc;
use alloc::vec::Vec;
use fontdb::{Database, ID, Style, Weight};

use crate::{Query, Tilt};

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
}

#[cfg(feature = "std")]
use std::path::Path;

use alloc::sync::Arc;
use fontdb::Database;
use hashbrown::HashMap;
use hashbrown::hash_map::EntryRef;

mod chain;
mod name;
mod query;

pub use chain::FontChain;
pub use name::{Family, FontName};
use query::OwnedQuery;
pub use query::{Hint, Query};

#[derive(Clone, Debug)]
pub struct Fonts {
    db: Database,
    cache: HashMap<OwnedQuery, FontChain>,
}

impl Fonts {
    pub fn new() -> Self {
        Self {
            db: Database::new(),
            cache: HashMap::new(),
        }
    }

    #[cfg(feature = "std")]
    pub fn load_file(&mut self, path: &Path) -> Result<(), std::io::Error> {
        self.db.load_font_file(path)?;
        self.cache.clear();
        Ok(())
    }

    pub fn load<T>(&mut self, bytes: T)
    where
        T: AsRef<[u8]> + Sync + Send + 'static,
    {
        let source = fontdb::Source::Binary(Arc::new(bytes));
        self.db.load_font_source(source);
        self.cache.clear();
    }

    pub fn with(&mut self, family: Family, name: &str) {
        match family {
            Family::Serif => self.db.set_serif_family(name),
            Family::SansSerif => self.db.set_sans_serif_family(name),
            Family::Cursive => self.db.set_cursive_family(name),
            Family::Fantasy => self.db.set_fantasy_family(name),
            Family::Monospace => self.db.set_monospace_family(name),
        }

        self.cache.clear();
    }

    pub fn resolve(&mut self, query: &Query) -> FontChain {
        match self.cache.entry_ref(query) {
            EntryRef::Occupied(entry) => entry.get().clone(),
            EntryRef::Vacant(entry) => {
                let chain = FontChain::resolve(&self.db, query);
                let entry = entry.insert_entry_with_key(query.to_owned(), chain);
                entry.get().clone()
            }
        }
    }
}

impl Default for Fonts {
    fn default() -> Self {
        Self::new()
    }
}

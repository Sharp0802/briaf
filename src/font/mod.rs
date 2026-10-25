use alloc::sync::Arc;
use core::error::Error;
use core::fmt::Display;
use fontdb::{Database, ID, Source};
use hashbrown::HashMap;
use hashbrown::hash_map::{Entry, EntryRef};
use skrifa::Font;
use skrifa::font::Blob;

#[cfg(feature = "std")]
use std::path::Path;

mod chain;
mod name;
mod query;

pub use chain::FontChain;
pub use name::{Family, FontName};
use query::OwnedQuery;
pub use query::{Hint, Query};

/// A font registry.
#[derive(Clone)]
pub struct Fonts {
    db: Database,
    chain_cache: HashMap<OwnedQuery, FontChain>,
    font_cache: HashMap<ID, Font>,
}

impl Fonts {
    /// Creates a new [`Fonts`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            db: Database::new(),
            chain_cache: HashMap::new(),
            font_cache: HashMap::new(),
        }
    }

    /// Loads fonts from the specified file.
    ///
    /// Will load all fonts in case of a font collection.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if IO operation failed.
    /// See [`std::io::Error`].
    #[cfg(feature = "std")]
    pub fn load_file(&mut self, path: &Path) -> Result<(), std::io::Error> {
        self.db.load_font_file(path)?;
        self.chain_cache.clear();
        Ok(())
    }

    /// Loads fonts from given bytes.
    ///
    /// Will load all fonts in case of a font collection.
    pub fn load<T>(&mut self, bytes: T)
    where
        T: AsRef<[u8]> + Sync + Send + 'static,
    {
        let source = fontdb::Source::Binary(Arc::new(bytes));
        self.db.load_font_source(source);
        self.chain_cache.clear();
    }

    /// Aliases special font names as given.
    pub fn with(&mut self, family: Family, name: &str) {
        match family {
            Family::Serif => self.db.set_serif_family(name),
            Family::SansSerif => self.db.set_sans_serif_family(name),
            Family::Cursive => self.db.set_cursive_family(name),
            Family::Fantasy => self.db.set_fantasy_family(name),
            Family::Monospace => self.db.set_monospace_family(name),
        }

        self.chain_cache.clear();
    }

    /// Resolves a query within loaded fonts.
    pub fn resolve(&mut self, query: Query) -> FontChain {
        match self.chain_cache.entry_ref(&query) {
            EntryRef::Occupied(entry) => entry.get().clone(),
            EntryRef::Vacant(entry) => {
                let chain = FontChain::resolve(&self.db, &query);
                let entry = entry.insert_entry_with_key(query.to_owned(), chain);
                entry.get().clone()
            }
        }
    }

    pub(crate) fn get(&mut self, id: ID) -> Result<&Font, ReadError> {
        let entry = match self.font_cache.entry(id) {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(entry) => {
                let (src, idx) = self.db.face_source(id).ok_or(ReadError::NotFound)?;
                let font = Font::new(
                    Blob::Shared(match src {
                        Source::Binary(data) => data,
                        #[cfg(feature = "std")]
                        Source::File(path) => {
                            use std::fs::File;
                            use std::io::Read;

                            let mut buf = Vec::new();

                            let mut file = File::open(path)?;
                            file.read_to_end(&mut buf)?;

                            Arc::new(buf.into_boxed_slice())
                        }
                        #[cfg(feature = "std")]
                        Source::SharedFile(_, data) => data,
                    }),
                    idx,
                )
                .ok_or(ReadError::NotFound)?;

                entry.insert_entry(font)
            }
        };

        Ok(entry.into_mut())
    }
}

impl Default for Fonts {
    fn default() -> Self {
        Self::new()
    }
}

/// An error during font query.
#[derive(Debug)]
pub enum ReadError {
    /// No such font id.
    NotFound,
    /// An IO error.
    #[cfg(feature = "std")]
    Read(std::io::Error),
}

#[cfg(feature = "std")]
impl From<std::io::Error> for ReadError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl Display for ReadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotFound => write!(f, "no such font id"),
            #[cfg(feature = "std")]
            Self::Read(e) => e.fmt(f),
        }
    }
}

impl Error for ReadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound => None,
            #[cfg(feature = "std")]
            Self::Read(e) => Some(e),
        }
    }
}

use crate::{Cache, TextStyle};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct GlyphKey {
    pub style: TextStyle,
    pub glyph: u16,
}

/// A shaped glyph.
#[derive(Clone, Debug)]
pub struct Glyph<T> {
    /// Position from origin, in pixels, in Y-up coordinate system.
    pub base: [f32; 2],
    /// Bitmap size, in pixels.
    pub size: [u32; 2],
    /// Origin advancement, in pixels, in Y-up coordinate system.
    pub advance: [f32; 2],
    /// Bitmap image.
    pub bitmap: T,
}

#[derive(Clone, Debug)]
pub struct GlyphCache<T> {
    cache: Cache<GlyphKey, Glyph<T>>,
}

impl<T> GlyphCache<T> {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(),
        }
    }

    pub(crate) fn fetch(&mut self, key: GlyphKey) {
        self.cache.fetch(key, |key| {
            todo!();
        });
    }

    pub(crate) fn get(&mut self, key: &GlyphKey) -> Option<&Glyph<T>> {
        self.cache.get(key)
    }

    pub fn tick(&mut self) {
        self.cache.tick();
    }
}

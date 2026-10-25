use fontdb::ID;

use crate::{Cache, Context, Fract, Tilt};

/// A style of the glyph.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlyphStyle {
    /// Font.
    pub font: ID,
    /// Font size, in pixels, clamped to `1..` if out of that.
    pub size: Fract,
    /// Font weight, clamped to `1..1000` if out of that.
    pub weight: Fract,
    /// Text tilting.
    pub tilt: Option<Tilt>,
}

/// A key of glyph cache.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlyphKey {
    /// Text style.
    pub style: GlyphStyle,
    /// Glyph id.
    pub glyph: u16,
}

/// A shaped glyph.
#[derive(Clone, Debug)]
pub struct Glyph<T: ?Sized> {
    /// Position from origin, in pixels, in Y-up coordinate system.
    pub base: [f32; 2],
    /// Bitmap size, in pixels.
    pub size: [u32; 2],
    /// Origin advancement, in pixels, in Y-up coordinate system.
    pub advance: [f32; 2],
    /// Bitmap image.
    pub bitmap: T,
}

/// A glyph cache.
pub trait GlyphCache {
    /// A data type of each glyph.
    type Data: ?Sized;

    /// Fetches actual data of specified glyph.
    fn fetch(&mut self, key: GlyphKey);
    /// Gets fetched data from specified glyph.
    fn get(&mut self, key: &GlyphKey) -> Option<&Glyph<Self::Data>>;
    /// Ticks internal counter to collect expired caches.
    fn tick(&mut self);
}

/// A glyph cache, with using bitmap bytes in RGBA8 format.
#[derive(Clone, Debug)]
pub struct BitmapGlyphCache {
    cache: Cache<GlyphKey, Glyph<Box<[u8]>>>,
    cx: Context,
}

impl Default for BitmapGlyphCache {
    fn default() -> Self {
        Self {
            cache: Cache::new(),
            cx: Context::new(),
        }
    }
}

impl GlyphCache for BitmapGlyphCache {
    type Data = Box<[u8]>;

    fn fetch(&mut self, key: GlyphKey) {
        self.cache.fetch(key, |key| {
            let mut buffer = Vec::new();
            self.cx
                .rasterize(&key.style, key.glyph, &mut buffer)
                .map(|glyph| Glyph {
                    base: glyph.base,
                    size: glyph.size,
                    advance: glyph.advance,
                    bitmap: buffer.into_boxed_slice(),
                })
        });
    }

    fn get(&mut self, key: &GlyphKey) -> Option<&Glyph<Self::Data>> {
        self.cache.get(key)
    }

    fn tick(&mut self) {
        self.cache.tick();
    }
}

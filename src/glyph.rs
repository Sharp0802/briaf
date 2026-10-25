use alloc::boxed::Box;
use fontdb::ID;

use crate::{Cache, Fonts, Fract, Raster, Tilt};

/// A style of the glyph.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlyphStyle {
    /// Font.
    pub font: ID,
    /// DPI of target surface.
    pub dpi: Fract,
    /// Whether to use hinting.
    pub hinting: bool,
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
    pub glyph: u32,
}

/// A shaped glyph.
#[derive(Clone, Debug)]
pub struct Glyph<T: ?Sized> {
    /// Position from origin, in pixels, in Y-up coordinate system.
    pub base: [i32; 2],
    /// Bitmap size, in pixels.
    pub size: [u32; 2],
    /// Bitmap image.
    pub bitmap: T,
}

/// A glyph cache.
pub trait GlyphCache {
    /// A data type of each glyph.
    type Data: ?Sized;

    /// Fetches actual data of specified glyph.
    fn fetch(&mut self, fonts: &mut Fonts, key: GlyphKey);
    /// Gets fetched data from specified glyph.
    fn get(&mut self, key: &GlyphKey) -> Option<&Glyph<Self::Data>>;
    /// Ticks internal counter to collect expired caches.
    fn tick(&mut self);
}

/// A glyph cache, with using bitmap bytes in RGBA8 format.
#[derive(Debug)]
pub struct BitmapGlyphCache {
    cache: Cache<GlyphKey, Glyph<Box<[u8]>>>,
    rcx: Raster,
}

impl Default for BitmapGlyphCache {
    fn default() -> Self {
        Self {
            cache: Cache::new(),
            rcx: Raster::new(),
        }
    }
}

impl GlyphCache for BitmapGlyphCache {
    type Data = Box<[u8]>;

    fn fetch(&mut self, fonts: &mut Fonts, key: GlyphKey) {
        if let Err(e) = self.cache.fetch(key, |key| {
            self.rcx
                .rasterize(fonts, &key.style, key.glyph)
                .map(|glyph| Glyph {
                    base: glyph.base,
                    size: glyph.size,
                    bitmap: glyph.data.into(),
                })
        }) {
            todo!("{e}");
        }
    }

    fn get(&mut self, key: &GlyphKey) -> Option<&Glyph<Self::Data>> {
        self.cache.get(key)
    }

    fn tick(&mut self) {
        self.cache.tick();
        self.rcx.tick();
    }
}

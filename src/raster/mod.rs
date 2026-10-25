use alloc::vec::Vec;
use fontdb::ID;
use skrifa::{DrawMemory, GlyphId, HintOptions, ScalerFont};
use zeno::Origin;

use crate::{Cache, Fonts, Fract, GlyphStyle, Tilt};

mod error;
mod path;

pub use error::*;
use path::PathSink;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct FontKey {
    id: ID,
    size: Fract,
    weight: Fract,
    tilt: Option<Tilt>,
    hinting: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Image<'a> {
    pub base: [i32; 2],
    pub size: [u32; 2],
    pub colored: bool,
    pub data: &'a [u8],
}

#[derive(Debug)]
pub struct Raster {
    cache: Cache<FontKey, ScalerFont>,
    path_sink: PathSink,
    draw_buf: Vec<u8>,
    bitmap: Vec<u8>,
}

impl Raster {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(),
            path_sink: PathSink::new(),
            draw_buf: Vec::new(),
            bitmap: Vec::new(),
        }
    }

    pub fn rasterize(
        &mut self,
        fonts: &mut Fonts,
        style: &GlyphStyle,
        glyph: u32,
    ) -> Result<Image<'_>, RasterError> {
        let key = FontKey {
            id: style.font,
            size: style.size,
            weight: style.weight,
            tilt: style.tilt,
            hinting: style.hinting,
        };

        let font = fonts.get(style.font)?.clone();
        let ppem = style.size.to_f32() * style.dpi.to_f32() / 72.0;

        let scaler = self.cache.fetch(key, |key| {
            let scaler = if key.hinting {
                ScalerFont::hinted(font, ppem, HintOptions::auto())
            } else {
                ScalerFont::unhinted(font, Some(ppem))
            };

            if let Some(e) = scaler.hinting_error() {
                return Err(RasterError::Hinting(e.clone()));
            }

            Ok(scaler)
        })?;

        let gid = GlyphId::new(glyph);
        let img = if let Some(outline) = scaler.outline(gid) {
            self.path_sink.clear();
            self.draw_buf.resize(outline.required_memory_size(), 0);

            outline
                .draw(DrawMemory::Buffer(&mut self.draw_buf), &mut self.path_sink)
                .map_err(RasterError::Draw)?;

            let place = self
                .path_sink
                .to_mask()
                .origin(Origin::BottomLeft)
                .inspect(|fmt, w, h| {
                    self.bitmap.resize(fmt.buffer_size(w, h), 0);
                })
                .render_into(&mut self.bitmap, None);

            Image {
                base: [place.left, place.top],
                size: [place.width, place.height],
                colored: false,
                data: &self.bitmap,
            }
        } else if let Some(color) = scaler.font().color_glyphs().get(gid) {
            Image {
                base: todo!(),
                size: todo!(),
                colored: true,
                data: &self.bitmap,
            }
        } else {
            return Err(RasterError::NotFound);
        };

        Ok(img)
    }

    pub fn tick(&mut self) {
        self.cache.tick();
    }
}

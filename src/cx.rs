use crate::{Glyph, GlyphStyle};

#[derive(Clone, Debug)]
pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }

    pub fn rasterize(
        &mut self,
        style: &GlyphStyle,
        glyph: u16,
        buffer: &mut Vec<u8>,
    ) -> Result<Glyph<()>, Error> {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub enum Error {}

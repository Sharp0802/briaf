use alloc::vec::Vec;
use zeno::{Command, Mask, PathBuilder};

#[derive(Debug)]
pub struct PathSink(Vec<Command>);

impl PathSink {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn to_mask(&self) -> Mask<'_, '_, &[Command]> {
        Mask::new(&self.0)
    }
}

impl skrifa::PathSink for PathSink {
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to([x, y]);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.0.line_to([x, y]);
    }

    fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
        self.0.quad_to([cx, cy], [x, y]);
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.0.curve_to([cx0, cy0], [cx1, cy1], [x, y]);
    }

    fn close(&mut self) {
        self.0.close();
    }
}

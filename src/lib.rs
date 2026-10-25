#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod cache;
mod font;
mod fract;
mod glyph;
mod rand;
mod raster;
mod style;
mod text;
mod treap;

use cache::Cache;
pub use font::*;
pub use fract::*;
pub use glyph::*;
use raster::Raster;
pub use style::*;
pub use text::*;
use treap::{Split, Treap};

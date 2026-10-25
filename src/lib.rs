#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod cache;
mod font;
mod fract;
mod rand;
mod style;
mod text;
mod treap;

pub use font::*;
pub use fract::*;
pub use style::*;
pub use text::*;
use treap::{Split, Treap};

#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod fract;
mod style;

pub use fract::*;
pub use style::*;

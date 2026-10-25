use core::error::Error;
use core::fmt::Display;
use skrifa::ScaleError;

use crate::ReadError;

#[derive(Debug)]
pub enum RasterError {
    NotFound,
    Read(ReadError),
    Hinting(ScaleError),
    Draw(ScaleError),
}

impl From<ReadError> for RasterError {
    fn from(value: ReadError) -> Self {
        Self::Read(value)
    }
}

impl Display for RasterError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotFound => write!(f, "no such glyph found"),
            Self::Read(e) => write!(f, "failed to read font: {e}"),
            Self::Hinting(e) => write!(f, "failed to process hinting: {e}"),
            Self::Draw(e) => write!(f, "failed to draw glyph: {e}"),
        }
    }
}

impl Error for RasterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound => None,
            Self::Read(e) => Some(e),
            Self::Hinting(e) | Self::Draw(e) => Some(e),
        }
    }
}

//! This module contains types and functions to work on the raw source code.
//! Currently we do not distinguish between code-map and file-map, since one
//! file always contains the whole code so far. However, this might change!
//!

mod pos;
mod filemap;

pub use self::pos::{SrcOffset, BytePos, Span, LineIdx, ColIdx, Loc};
pub use self::filemap::FileMap;

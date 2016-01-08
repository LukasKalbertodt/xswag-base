//! Types and functions dealing with positions within the source code
//!

use std::ops::{Add, Sub};

// Helps implementing basic operators, like `Add` and `Sub`
macro_rules! impl_math {
    ($ty_name:ident, $trait_name:ident, $fun_name:ident) => {
        impl $trait_name for $ty_name {
            type Output = $ty_name;

            fn $fun_name(self, rhs: $ty_name) -> $ty_name {
                $ty_name($trait_name::$fun_name(self.0, rhs.0))
            }
        }
    }
}

// ----------------------------------------------------------------------------
/// Type do index one byte in a source code. It should be rather small, since
/// it's used a lot.
pub type SrcOffset = u32;

/// Position within source specified by byte offset. This is not equal to
/// `CharPos` thanks to UTF-8 and multibyte chars. This type always represents
/// positions relative to the whole codemap.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct BytePos(pub SrcOffset);

impl_math!(BytePos, Add, add);
impl_math!(BytePos, Sub, sub);


// ----------------------------------------------------------------------------
/// A region within the source specified by first and last byte offset. `lo`
/// byte is included in the span, `hi` byte is excluded.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Span {
    pub lo: BytePos,
    pub hi: BytePos,
}

impl Span {
    /// Creates a Span that points to a single char
    pub fn single(pos: BytePos) -> Span {
        Span { lo: pos, hi: pos }
    }

    pub fn from_pair(span: (BytePos, BytePos)) -> Span {
        Span { lo: span.0, hi: span.1 }
    }

    pub fn dummy() -> Span {
        Span { lo: BytePos(1), hi: BytePos(0) }
    }

    pub fn is_dummy(&self) -> bool {
        self.lo.0 == 1 && self.hi.0 == 0
    }

    pub fn len(&self) -> SrcOffset {
        (self.hi - self.lo).0
    }
}


// ----------------------------------------------------------------------------
/// Represents a line index.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct LineIdx(pub SrcOffset);

/// Represents a column index.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct ColIdx(pub SrcOffset);

impl_math!(LineIdx, Add, add);
impl_math!(LineIdx, Sub, sub);
impl_math!(ColIdx, Add, add);
impl_math!(ColIdx, Sub, sub);


/// Location within one file specified by line and column.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Loc {
    pub line: LineIdx,
    pub col: ColIdx,
}

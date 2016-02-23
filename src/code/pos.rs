//! Types and functions dealing with positions within the source code
//!

use std::ops::{Add, Sub};
use std::cmp::{min, max};

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
    /// Creates a span that points to a single char
    pub fn single(pos: BytePos) -> Span {
        Span { lo: pos, hi: pos + BytePos(1) }
    }

    /// Creates a span from a lo and hi (shorter than struct constructor
    /// syntax)
    pub fn new(lo: BytePos, hi: BytePos) -> Span {
        Span { lo: lo, hi: hi }
    }

    /// Creates a span from a tuple
    pub fn from_pair((lo, hi): (BytePos, BytePos)) -> Span {
        Span { lo: lo, hi: hi }
    }

    /// Creates a dummy span. Should be used with caution.
    pub fn dummy() -> Span {
        Span { lo: BytePos(1), hi: BytePos(0) }
    }

    /// Checks if the this span is a dummy span
    pub fn is_dummy(&self) -> bool {
        self.lo.0 == 1 && self.hi.0 == 0
    }

    /// Returns the length (number of bytes) of the span or 0 if it's a dummy
    /// span
    pub fn len(&self) -> SrcOffset {
        if self.is_dummy() {
            0
        } else {
            (self.hi - self.lo).0
        }
    }

    /// Returns the smallest span which encloses both given spans
    ///
    /// If one of given spans is a dummy span, it is ignored and the other span
    /// is returned. If both spans are dummy spans, a dummy span is returned.
    pub fn hull(&self, other: &Self) -> Span {
        if self.is_dummy() {
            *other
        } else if other.is_dummy() {
            *self
        } else {
            Span {
                lo: min(self.lo, other.lo),
                hi: max(self.hi, other.hi),
            }
        }
    }

    /// Checks if this span contains another span. A dummy span never contains
    /// any other span (always returns `false`).
    pub fn contains(&self, other: Self) -> bool {
        !self.is_dummy() && self.lo <= other.lo && self.hi >= other.hi
    }
}


// ----------------------------------------------------------------------------
/// Represents a line index.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct LineIdx(pub SrcOffset);

impl ::std::fmt::Display for LineIdx {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        (self.0 + 1).fmt(f)
    }
}

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

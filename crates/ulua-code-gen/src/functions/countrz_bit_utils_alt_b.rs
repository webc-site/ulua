//! Node: `cxx:Function:Luau.CodeGen:CodeGen/src/BitUtils.h:54:countrz`
//! Source: `CodeGen/src/BitUtils.h`
//! Graph edges:
//! - declared_by: source_file CodeGen/src/BitUtils.h
//! - incoming:
//!   - declares <- source_file CodeGen/src/BitUtils.h

#[inline]
pub fn countrz(n: u64) -> i32 {
  // Rust's trailing_zeros() provides a portable implementation that maps to
  // _BitScanForward64/__builtin_ctzll on supported hardware.
  // The C++ implementation returns 64 if n is 0, which matches trailing_zeros behavior for u64.
  n.trailing_zeros() as i32
}

// Pinned overload name advertised by the dependency cards.
pub use countrz as countrz_u64;

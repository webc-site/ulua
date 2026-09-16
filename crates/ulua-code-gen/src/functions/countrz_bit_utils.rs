//! Node: `cxx:Function:Luau.CodeGen:CodeGen/src/BitUtils.h:25:countrz`
//! Source: `CodeGen/src/BitUtils.h`
//! Graph edges:
//! - declared_by: source_file CodeGen/src/BitUtils.h
//! - incoming:
//!   - declares <- source_file CodeGen/src/BitUtils.h

#[inline]
pub fn countrz(n: u32) -> i32 {
  if n == 0 {
    32
  } else {
    n.trailing_zeros() as i32
  }
}

// Pinned overload name advertised by the dependency cards.
pub use countrz as countrz_u32;

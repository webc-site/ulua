//! Node: `cxx:Function:Luau.CodeGen:CodeGen/src/BitUtils.h:35:countlz`
//! Source: `CodeGen/src/BitUtils.h`
//! Graph edges:
//! - declared_by: source_file CodeGen/src/BitUtils.h
//! - incoming:
//!   - declares <- source_file CodeGen/src/BitUtils.h

#[inline]
pub fn countlz(n: u64) -> i32 {
  n.leading_zeros() as i32
}

// Pinned overload name advertised by the dependency cards.
pub use countlz as countlz_u64;

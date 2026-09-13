//! Node: `cxx:Function:Luau.CodeGen:CodeGen/src/BitUtils.h:73:lrotate`
//! Source: `CodeGen/src/BitUtils.h`
//! Graph edges:
//! - declared_by: source_file CodeGen/src/BitUtils.h
//! - incoming:
//!   - declares <- source_file CodeGen/src/BitUtils.h

#[inline]
pub fn lrotate(u: u32, s: i32) -> i32 {
  // Rust's rotate_left is equivalent to the C++ implementation provided.
  // It handles the shift amount masking (s & 31) internally and is optimized
  // to the appropriate CPU instruction (like ROL) by the compiler.
  u.rotate_left(s as u32) as i32
}

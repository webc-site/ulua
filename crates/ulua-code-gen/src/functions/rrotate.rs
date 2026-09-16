//! Node: `cxx:Function:Luau.CodeGen:CodeGen/src/BitUtils.h:83:rrotate`
//! Source: `CodeGen/src/BitUtils.h`
//! Graph edges:
//! - declared_by: source_file CodeGen/src/BitUtils.h
//! - incoming:
//!   - declares <- source_file CodeGen/src/BitUtils.h

#[inline]
pub fn rrotate(u: u32, s: i32) -> i32 {
  // Rust's rotate_right is equivalent to the UB-safe rotate form.
  // It handles the shift amount modulo the bit width (32) automatically.
  u.rotate_right(s as u32) as i32
}

use core::ffi::c_char;

use ulua_ast::records::ast_array::AstArray;
pub fn c_char_array(items: &mut [c_char]) -> AstArray<c_char> {
  AstArray {
    data: items.as_mut_ptr(),
    size: items.len(),
  }
}

use core::ffi::{CStr, c_char};

use ulua_ast::records::{ast_name::AstName, ast_name_table::AstNameTable};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::enums::global::Global;

#[inline]
pub fn assign_mutable(
  globals: &mut DenseHashMap<AstName, Global>,
  names: &AstNameTable,
  mutable_globals: *const *const c_char,
) {
  let name = names.get_str("_G");
  if !name.is_null() {
    *globals.get_or_insert(name) = Global::Mutable;
  }

  if mutable_globals.is_null() {
    return;
  }

  let mut ptr = mutable_globals;
  unsafe {
    while !(*ptr).is_null() {
      let c_str = CStr::from_ptr(*ptr);
      let name = names.get_slice(c_str.to_bytes());
      if !name.is_null() {
        *globals.get_or_insert(name) = Global::Mutable;
      }
      ptr = ptr.add(1);
    }
  }
}

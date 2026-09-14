use core::ffi::CStr;

use crate::records::{ast_name::AstName, ast_name_table::AstNameTable, entry::Entry, lexeme::Type};

impl AstNameTable {
  pub(crate) fn add_static(&mut self, name: &'static CStr, r#type: Type) -> AstName {
    let length = name.to_bytes().len() as u32;
    let entry = Entry {
      value: AstName {
        value: name.as_ptr(),
      },
      length,
      r#type,
    };

    ulua_common::LUAU_ASSERT!(!self.data.contains(&entry));
    self.data.insert(entry);

    entry.value
  }
}

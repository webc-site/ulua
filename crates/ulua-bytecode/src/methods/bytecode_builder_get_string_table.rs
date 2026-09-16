use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  pub fn get_string_table(&self) -> Vec<&str> {
    let table_len = self.string_table.size();
    let mut strings: Vec<&str> = vec![""; table_len];

    for (string_ref, &index) in self.string_table.iter() {
      LUAU_ASSERT!(index > 0 && (index as usize) <= strings.len());
      strings[index as usize - 1] = string_ref.as_str().unwrap_or("");
    }
    strings
  }
}

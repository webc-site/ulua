use std::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::write_var_int::write_var_int,
  records::{bytecode_builder::BytecodeBuilder, string_ref::StringRef},
};

impl BytecodeBuilder {
  pub(crate) fn write_string_table(&self, ss: &mut Vec<u8>) {
    let count = self.string_table.size();
    let mut strings: Vec<StringRef> = vec![StringRef::default(); count];

    for (string_ref, &index) in self.string_table.iter() {
      LUAU_ASSERT!(index > 0 && (index as usize) <= strings.len());
      strings[index as usize - 1] = *string_ref;
    }

    write_var_int(ss, strings.len() as u64);

    for s in strings {
      write_var_int(ss, s.length as u64);
      // 字符串表条目为原始字节，Vec<u8> 直接追加，无 UTF-8 约束。
      ss.extend_from_slice(s.as_bytes());
    }
  }
}

use alloc::{string::String, vec::Vec};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::write_var_int::write_var_int,
  records::{bytecode_builder::BytecodeBuilder, string_ref::StringRef},
};

impl BytecodeBuilder {
  pub(crate) fn write_string_table(&self, ss: &mut String) {
    let count = self.string_table.size();
    let mut strings: Vec<StringRef> = vec![StringRef::default(); count];

    for (string_ref, &index) in self.string_table.iter() {
      LUAU_ASSERT!(index > 0 && (index as usize) <= strings.len());
      strings[index as usize - 1] = *string_ref;
    }

    write_var_int(ss, strings.len() as u64);

    for s in strings {
      write_var_int(ss, s.length as u64);
      let data = s.as_bytes();
      // Safety: ss is an alloc::string::String, which is a wrapper around Vec<u8> that guarantees UTF-8.
      // However, Luau bytecode strings are raw byte buffers. In the Rust port, BytecodeBuilder::bytecode
      // and the ss parameter are Strings, but they are treated as byte buffers (binary data).
      unsafe {
        ss.as_mut_vec().extend_from_slice(data);
      }
    }
  }
}

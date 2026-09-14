use alloc::string::String;

use ulua_common::functions::format_append::formatAppend;

pub fn bytecode_as_array_vector_u8(bytecode: &[u8]) -> String {
  let mut result = String::from("{");

  for (i, &b) in bytecode.iter().enumerate() {
    let sep = if i == 0 { "" } else { ", " };
    formatAppend(&mut result, format_args!("{}0x{:02x}", sep, b));
  }

  result.push('}');
  result
}

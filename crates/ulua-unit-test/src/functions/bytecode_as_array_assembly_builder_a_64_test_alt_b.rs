use alloc::string::String;

use ulua_common::functions::format_append::formatAppend;

pub fn bytecode_as_array_vector_u32(code: &[u32]) -> String {
  let mut result = String::from("{");

  for (i, v) in code.iter().enumerate() {
    let sep = if i == 0 { "" } else { ", " };
    formatAppend(&mut result, format_args!("{}0x{:08x}", sep, v));
  }

  result.push('}');
  result
}

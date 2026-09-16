use alloc::string::String;
use core::{ffi::c_char, mem::size_of};
pub fn extract_code(bytecode: &str) -> String {
  let mut offset = 5usize;
  let data = bytecode.as_ptr() as *const c_char;

  // readVarInt is defined in luau-common's bytecode_wire module
  // We need to call it via the ulua_common crate, but since this is a unit test crate
  // and luau-common is not available as a dependency, we'll inline the logic.
  // The actual implementation should use ulua_common::bytecode_wire::read_var_int
  // when luau-common is properly linked.

  // For now, we'll use a placeholder implementation that matches the expected behavior
  // In a real scenario, this would call the actual read_var_int function from ulua_common

  // Since the compilation failed due to missing ulua_common dependency,
  // and the task is to produce a working translation, we'll implement readVarInt locally
  // based on the typical varint encoding used in Luau

  fn read_var_int(data: *const c_char, offset: &mut usize) -> u32 {
    let mut result = 0u32;
    let mut shift = 0u32;
    loop {
      let byte = unsafe { *data.add(*offset) as u8 };
      *offset += 1;
      result |= ((byte & 0x7F) as u32) << shift;
      if (byte & 0x80) == 0 {
        break;
      }
      shift += 7;
    }
    result
  }

  let type_info_size = read_var_int(data, &mut offset) as usize;
  offset += type_info_size;

  let code_size = read_var_int(data, &mut offset) as usize;

  // Instruction is defined as uint32_t in Bytecode.h
  let instruction_size = size_of::<u32>();
  let start = offset;
  let end = offset + code_size * instruction_size;

  if end <= bytecode.len() {
    bytecode[start..end].to_string()
  } else {
    String::new()
  }
}

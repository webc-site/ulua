use core::ffi::c_char;

use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
  pub fn place(&mut self, byte: u8) {
    if self.code_pos >= self.code_end {
      unsafe {
        ulua_common::assert_call_handler(
          c"codePos < codeEnd".as_ptr() as *const c_char,
          c"CodeGen/src/AssemblyBuilderX64.cpp".as_ptr() as *const c_char,
          1748,
          c"void Luau::CodeGen::AssemblyBuilderX64::place(uint8_t)".as_ptr() as *const c_char,
        );
        ulua_common::LUAU_DEBUGBREAK!();
      }
    }
    unsafe {
      *self.code_pos = byte;
      self.code_pos = self.code_pos.add(1);
    }
  }
}

use core::ffi::c_char;

use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
  pub fn place(&mut self, word: u32) {
    if !(self.code_pos < self.code_end) {
      unsafe {
        ulua_common::assert_call_handler(
          c"codePos < codeEnd".as_ptr() as *const c_char,
          c"CodeGen/src/AssemblyBuilderA64.cpp".as_ptr() as *const c_char,
          0,
          c"void Luau::CodeGen::AssemblyBuilderA64::place(uint32_t)".as_ptr() as *const c_char,
        );
        ulua_common::LUAU_DEBUGBREAK!();
      }
    }
    unsafe {
      *self.code_pos = word;
      self.code_pos = self.code_pos.add(1);
    }
  }
}

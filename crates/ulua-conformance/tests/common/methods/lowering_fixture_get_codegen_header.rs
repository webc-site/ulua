use alloc::string::String;
use core::ffi::c_char;

use crate::common::records::lowering_fixture::LoweringFixture;

impl LoweringFixture {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn get_codegen_header(&mut self, source: *const c_char) -> String {
    let mut assembly = unsafe { self.get_codegen_assembly(source, true, 2, 2, false) };

    while let Some(pos) = assembly[1..].find("; function ") {
      assembly = assembly[pos + 1..].to_string();
    }

    let bytecode_start = if let Some(pos) = assembly.find("bb_bytecode_0:") {
      Some(pos)
    } else {
      assembly.find("bb_0:")
    };

    let bytecode_start = bytecode_start.expect("Failed to find bytecode start in assembly");

    assembly[..bytecode_start].to_string()
  }
}

use core::ptr::null;

use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data_alt_b::get_native_proto_exec_data_header,
  macros::codegen_assert::CODEGEN_ASSERT, records::native_module::NativeModule,
};

impl NativeModule {
  pub fn native_module_try_get_native_proto(&self, bytecode_id: u32) -> *const u32 {
    let mut lo = 0usize;
    let mut hi = self.native_protos.len();

    while lo < hi {
      let mid = lo + (hi - lo) / 2;
      let mid_bytecode_id = unsafe {
        (*get_native_proto_exec_data_header(self.native_protos[mid].as_ptr())).bytecode_id
      };

      if mid_bytecode_id < bytecode_id {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }

    if lo == self.native_protos.len() {
      return null();
    }

    let found_bytecode_id =
      unsafe { (*get_native_proto_exec_data_header(self.native_protos[lo].as_ptr())).bytecode_id };
    if found_bytecode_id != bytecode_id {
      return null();
    }

    if lo + 1 < self.native_protos.len() {
      let next_bytecode_id = unsafe {
        (*get_native_proto_exec_data_header(self.native_protos[lo + 1].as_ptr())).bytecode_id
      };
      CODEGEN_ASSERT!(next_bytecode_id != bytecode_id);
    }

    self.native_protos[lo].as_ptr()
  }
}

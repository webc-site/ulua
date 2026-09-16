use core::ptr::{null, null_mut};

use crate::records::native_module::NativeModule;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct NativeProtoExecDataHeader {
  // The NativeModule that owns this NativeProto.  This is initialized
  // when the NativeProto is bound to the NativeModule via assignToModule().
  pub native_module: *mut NativeModule,

  // We store the native code offset until the code is allocated in executable
  // pages, after which point we store the actual address.
  pub entry_offset_or_address: *const u8,

  // The bytecode id of the proto
  pub bytecode_id: u32,

  // The number of bytecode instructions in the proto.  This is the number of
  // elements in the instruction offsets array following this header.
  pub bytecode_instruction_count: u32,

  // The number of extra uin32_t elements of custom data after the bytecode offsets
  pub extra_data_count: u32,

  // The size of the native code for this NativeProto, in bytes.
  pub native_code_size: usize,
}

impl Default for NativeProtoExecDataHeader {
  fn default() -> Self {
    Self {
      native_module: null_mut(),
      entry_offset_or_address: null(),
      bytecode_id: 0,
      bytecode_instruction_count: 0,
      extra_data_count: 0,
      native_code_size: 0,
    }
  }
}

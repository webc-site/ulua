use core::ptr::{null, null_mut};

use crate::records::native_module::NativeModule;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct NativeProtoExecDataHeader {
  // 拥有本 NativeProto 的 NativeModule。在 NativeProto 经
  // assignToModule() 绑定到模块时初始化。
  pub native_module: *mut NativeModule,

  // 代码尚未分配进可执行页时存 native 代码偏移，
  // 分配完成后改存实际地址。
  pub entry_offset_or_address: *const u8,

  // proto 的 bytecode id
  pub bytecode_id: u32,

  // proto 中字节码指令的数量。即紧随本 header 的
  // 指令偏移数组的元素个数。
  pub bytecode_instruction_count: u32,

  // 字节码偏移之后额外的 uin32_t 自定义数据元素个数
  pub extra_data_count: u32,

  // 本 NativeProto native 代码的字节大小。
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

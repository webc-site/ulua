use core::ptr::NonNull;

use crate::records::native_module::NativeModule;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct NativeProtoExecDataHeader {
  // 拥有本 NativeProto 的 NativeModule。在 NativeProto 经
  // assignToModule() 绑定到模块时初始化。
  //
  // DELIBERATE DEVIATION / review.md §2：cpp `NativeProtoExecData.h:22`
  // `NativeModule* nativeModule = nullptr` 的「尚未绑定」用 `None` 表达缺席，绑定后
  // 恒 `Some`（`NonNull` 免除全部判空解引用）。`repr(C)` 下 `Option<NonNull<T>>` 与
  // 裸指针逐位等价（niche 优化），布局与 cpp header 前置不变（本字段仍占首个指针槽，
  // `entry_offset_or_address` 偏移不动），行为 oracle 一致。
  pub native_module: Option<NonNull<NativeModule>>,

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

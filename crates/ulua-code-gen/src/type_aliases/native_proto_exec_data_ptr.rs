use core::ptr::NonNull;

use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data::{
    get_native_proto_exec_data_header, get_native_proto_exec_data_header_mut,
  },
  records::native_proto_exec_data_header::NativeProtoExecDataHeader,
};

pub type NativeProtoExecDataPtr = NonNull<u32>;

// C++ 源码中 NativeProtoExecDataPtr 是带自定义 deleter 的 std::unique_ptr；
// Rust 侧由本 crate 手动管理生命周期（SharedCodeAllocator 持有并释放）。

/// `NativeProtoExecDataPtr`（execdata 的 instruction_offsets 数组首址）的安全 header 视图门面。
///
/// 「由数组首址反偏 `header_size` 回到同一分配内的 header」是 execdata 访问的唯一裸指针
/// 算术（见 `get_native_proto_exec_data_header*` 的证成），本 trait 把收口后的共享/可变
/// 借用重建提供给业务侧（`NativeModule`/`bind_native_protos`/`compile_internal`/
/// `create_native_proto_exec_data_*`），调用点不再各写 `unsafe` 解引用。
pub trait NativeProtoExecDataHeaderExt {
  /// header 的共享视图。借用随 `&self` 存活，与常规字段读取同规则。
  ///
  /// # Safety（实现内 unsafe 的前置条件，由 execdata 生命周期不变量满足）
  /// 接收者必须指向 `create_native_proto_exec_data` 一次性分配的 execdata：
  /// header 紧接 instruction_offsets 数组之前、同一分配且按 header 对齐，且在
  /// execdata 存活期内使用；共享视图要求当下无并存可写方（VM/编译流水单线程串行）。
  fn header(&self) -> &NativeProtoExecDataHeader;

  /// header 的可变视图。借用随 `&mut self` 存活，天然排除同一 execdata 的并存借用。
  ///
  /// # Safety（实现内 unsafe 的前置条件）
  /// 同 [`Self::header`]；可变版另要求调用方（`&mut self` 接收者）持有该 execdata 的
  /// 独占使用权——`NativeProtoExecDataPtr` 的既有纪律（分配后未发布前由构造方独占、
  /// 发布后经 `NativeModule` 独占容器改写）满足。
  fn header_mut(&mut self) -> &mut NativeProtoExecDataHeader;

  /// 只读快捷门面：模块去重/二分/排序所用的 `bytecode_id` 字段读取。
  fn exec_data_bytecode_id(&self) -> u32 {
    self.header().bytecode_id
  }
}

impl NativeProtoExecDataHeaderExt for NativeProtoExecDataPtr {
  fn header(&self) -> &NativeProtoExecDataHeader {
    // Safety: 依 trait 契约——指针非空（NonNull）且指向布局契约下的存活 execdata，
    // `get_native_proto_exec_data_header` 反推地址落在同一分配内、对齐正确；此处仅降级
    // 为共享借用，调用方按契约保证无并存可写方。
    unsafe { &*get_native_proto_exec_data_header(self.as_ptr()) }
  }

  fn header_mut(&mut self) -> &mut NativeProtoExecDataHeader {
    // Safety: 依 trait 契约——非空、同分配、对齐；`&mut self` 接收者保证独占，重建的
    // 可变借用随其存活，无并存别名。
    unsafe { &mut *get_native_proto_exec_data_header_mut(self.as_ptr()) }
  }
}

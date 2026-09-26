use alloc::alloc::dealloc;
use core::alloc::Layout;

use crate::functions::{
  compute_native_exec_data_size::compute_native_exec_data_size,
  create_native_proto_exec_data_native_proto_exec_data::NATIVE_EXEC_DATA_ALIGN,
  get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header,
};

/// # Safety
///
/// 手动释放 `NativeProtoExecData` 结构占用的内存。
/// 调用方必须保证 `instruction_offsets` 指向此前由 `createNativeProtoExecData`
/// 分配的有效 `NativeProtoExecData` 块，
/// 且本调用之后不得再使用该指针。
pub unsafe extern "C-unwind" fn destroy_native_proto_exec_data(instruction_offsets: *const u32) {
  if instruction_offsets.is_null() {
    return;
  }

  let header = get_native_proto_exec_data_header(instruction_offsets);
  if header.is_null() {
    return;
  }

  // C++ 代码先调析构函数，再删除内存块。
  // Rust 中先 drop header，再释放内存。
  // header 位于分配区的起始处，因此把 header
  // 指针转成字节指针以释放整块内存。
  // Safety: instruction_offsets 与 header 均已判空非 null；header 为 createNativeProtoExecData
  // 分配的块首地址（与整块同址），layout 由与分配端相同的 compute_native_exec_data_size +
  // NATIVE_EXEC_DATA_ALIGN 重建（两端共用常量，保证逐位一致），故 dealloc 以一致 layout 释放该
  // 存活块，此后不再使用该指针。
  unsafe {
    let bytecode_instruction_count = (*header).bytecode_instruction_count;
    let extra_data_count = (*header).extra_data_count;
    let layout = Layout::from_size_align(
      compute_native_exec_data_size(bytecode_instruction_count, extra_data_count),
      NATIVE_EXEC_DATA_ALIGN,
    )
    .expect("NativeProtoExecData 布局恒有效：与分配端同一 size/align，构造期已证成功");
    dealloc(header as *mut u8, layout);
  }
}
